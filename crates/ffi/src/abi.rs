//! The marshaling library behind the C ABI: result types, the `catch_unwind`
//! guard, C-string decoding, the per-hook runners, and the free functions.
//! Nothing here is `#[no_mangle]`; `export_c_abi!` emits the exported symbols
//! over these helpers for whichever registry the caller names.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;
use std::slice;
use superscalar::{ErrorKind, Registry, ScalarError, ScalarId};

/// Which `ScalarValue` field is live; `Bytes` carries object scalars.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ValueKind {
    Str = 0,
    Int = 1,
    Float = 2,
    Bool = 3,
    Bytes = 4,
}

/// Machine-readable error category; `None` is success, `Panic` is FFI-only
/// (a caught unwind), the rest map 1:1 onto `core::ErrorKind`.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorCategory {
    None = 0,
    Parse = 1,
    Pattern = 2,
    Length = 3,
    Range = 4,
    Enum = 5,
    Custom = 6,
    Empty = 7,
    Panic = 8,
}

impl ErrorCategory {
    fn from_kind(kind: ErrorKind) -> Self {
        match kind {
            ErrorKind::Parse => ErrorCategory::Parse,
            ErrorKind::Pattern => ErrorCategory::Pattern,
            ErrorKind::Length => ErrorCategory::Length,
            ErrorKind::Range => ErrorCategory::Range,
            ErrorKind::Enum => ErrorCategory::Enum,
            ErrorKind::Custom => ErrorCategory::Custom,
            ErrorKind::Empty => ErrorCategory::Empty,
        }
    }

    /// Stable lowercase form, prefixed onto the human `error` message.
    fn as_str(self) -> &'static str {
        match self {
            ErrorCategory::None => "none",
            ErrorCategory::Parse => "parse",
            ErrorCategory::Pattern => "pattern",
            ErrorCategory::Length => "length",
            ErrorCategory::Range => "range",
            ErrorCategory::Enum => "enum",
            ErrorCategory::Custom => "custom",
            ErrorCategory::Empty => "empty",
            ErrorCategory::Panic => "panic",
        }
    }
}

/// A canonical scalar value crossing the boundary; only the field selected by
/// `kind` is live, the rest are zeroed. Owned pointers freed via the matching
/// `*_free`.
#[repr(C)]
pub struct ScalarValue {
    pub kind: ValueKind,
    /// `Str`: UTF-8, NUL-terminated, `str_len` bytes (excluding the NUL).
    pub str_ptr: *mut c_char,
    pub str_len: usize,
    pub int_val: i64,
    pub float_val: f64,
    pub bool_val: bool,
    /// `Bytes`: `bytes_len` bytes, not NUL-terminated, bincode-encoded (not JSON).
    pub bytes_ptr: *mut u8,
    pub bytes_len: usize,
}

impl ScalarValue {
    /// No value: every pointer null, every scalar field zero. What a
    /// successful `validate` returns.
    pub fn empty() -> Self {
        ScalarValue {
            kind: ValueKind::Str,
            str_ptr: ptr::null_mut(),
            str_len: 0,
            int_val: 0,
            float_val: 0.0,
            bool_val: false,
            bytes_ptr: ptr::null_mut(),
            bytes_len: 0,
        }
    }

    /// String backing; fails only on an interior NUL in the canonical form.
    pub fn from_string(s: String) -> Result<Self, ScalarError> {
        let len = s.len();
        let owned = CString::new(s).map_err(|_| {
            ScalarError::new(ErrorKind::Custom, "canonical value contained interior NUL")
        })?;
        let mut v = Self::empty();
        v.kind = ValueKind::Str;
        v.str_ptr = owned.into_raw();
        v.str_len = len;
        Ok(v)
    }

    /// Int backing.
    pub fn from_int(value: i64) -> Self {
        let mut v = Self::empty();
        v.kind = ValueKind::Int;
        v.int_val = value;
        v
    }

    /// Float backing.
    pub fn from_float(value: f64) -> Self {
        let mut v = Self::empty();
        v.kind = ValueKind::Float;
        v.float_val = value;
        v
    }

    /// Bool backing.
    pub fn from_bool(value: bool) -> Self {
        let mut v = Self::empty();
        v.kind = ValueKind::Bool;
        v.bool_val = value;
        v
    }

    /// Bytes backing for object scalars.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let len = bytes.len();
        let boxed = bytes.into_boxed_slice();
        let mut v = Self::empty();
        v.kind = ValueKind::Bytes;
        v.bytes_ptr = Box::into_raw(boxed) as *mut u8;
        v.bytes_len = len;
        v
    }

    /// Reclaim this value's owned allocations.
    ///
    /// # Safety
    /// `self` must have been produced by this crate's marshaling helpers and
    /// not already freed.
    pub unsafe fn free(self) {
        if !self.str_ptr.is_null() {
            drop(CString::from_raw(self.str_ptr));
        }
        if !self.bytes_ptr.is_null() {
            drop(Box::from_raw(ptr::slice_from_raw_parts_mut(
                self.bytes_ptr,
                self.bytes_len,
            )));
        }
    }
}

/// The result of a single scalar hook; on success `value` is live and `error`
/// is null, on failure `value` is empty and `error`/`error_category` are set.
/// Freed with `scalar_result_free`.
#[repr(C)]
pub struct ScalarResult {
    pub ok: bool,
    pub value: ScalarValue,
    pub error_category: ErrorCategory,
    /// Human-readable `"category: message"`; null on success.
    pub error: *mut c_char,
}

/// A batch of results, owned as one allocation. Freed with
/// `scalar_result_array_free`.
#[repr(C)]
pub struct ScalarResultArray {
    pub ptr: *mut ScalarResult,
    pub len: usize,
}

pub fn make_ok(value: ScalarValue) -> ScalarResult {
    ScalarResult {
        ok: true,
        value,
        error_category: ErrorCategory::None,
        error: ptr::null_mut(),
    }
}

/// Compose the human `error` channel as `"category: message"`.
pub fn make_err(category: ErrorCategory, message: &str) -> ScalarResult {
    let composed = format!("{}: {}", category.as_str(), message);
    let owned = CString::new(composed).unwrap_or_else(|_| {
        CString::new("custom: error message contained interior NUL")
            .expect("static message has no NUL")
    });
    ScalarResult {
        ok: false,
        value: ScalarValue::empty(),
        error_category: category,
        error: owned.into_raw(),
    }
}

/// The catch_unwind seam: a panic becomes `ok: false` / `ErrorCategory::Panic`.
pub fn guard<F>(f: F) -> ScalarResult
where
    F: FnOnce() -> Result<ScalarValue, ScalarError>,
{
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(value)) => make_ok(value),
        Ok(Err(err)) => make_err(ErrorCategory::from_kind(err.kind), &err.message),
        Err(_) => make_err(ErrorCategory::Panic, "scalar hook panicked"),
    }
}

/// Which hook to run.
#[derive(Clone, Copy)]
pub enum Hook {
    Parse,
    Normalize,
    Validate,
}

/// Decode a caller-owned C string; the `Err` is the `ScalarResult` to return
/// for a null pointer or non-UTF-8 bytes.
///
/// # Safety
/// `input` must be null or a valid NUL-terminated C string that stays valid for
/// the lifetime `'a`.
pub unsafe fn decode_cstr<'a>(input: *const c_char) -> Result<&'a str, ScalarResult> {
    if input.is_null() {
        return Err(make_err(ErrorCategory::Parse, "null input pointer"));
    }
    CStr::from_ptr(input)
        .to_str()
        .map_err(|_| make_err(ErrorCategory::Parse, "input was not valid UTF-8"))
}

/// Resolve the id in `registry`, decode the input, and run the hook under the
/// panic guard.
///
/// # Safety
/// `input` must be null or a valid NUL-terminated C string that stays valid for
/// the duration of the call.
pub unsafe fn run(
    registry: &Registry,
    scalar_id: u32,
    input: *const c_char,
    hook: Hook,
) -> ScalarResult {
    let Some(scalar) = registry.scalar(ScalarId(scalar_id)) else {
        return make_err(ErrorCategory::Parse, "unknown scalar id");
    };
    let decoded = match decode_cstr(input) {
        Ok(decoded) => decoded,
        Err(err) => return err,
    };
    guard(|| match hook {
        Hook::Parse => scalar
            .parse(registry, decoded)
            .and_then(ScalarValue::from_string),
        Hook::Normalize => scalar
            .normalize(registry, decoded)
            .and_then(ScalarValue::from_string),
        Hook::Validate => scalar
            .validate(registry, decoded)
            .map(|()| ScalarValue::empty()),
    })
}

/// Lenient coercion over `registry`: `json_in` is a JSON document; on success
/// the result `value` is the canonical coercion encoded as a JSON string
/// (`"null"` for a null passthrough); a coercion failure maps onto
/// `error`/`error_category` exactly as the strict hooks map theirs.
///
/// # Safety
/// `json_in` must be null or a valid NUL-terminated C string that stays valid
/// for the duration of the call.
pub unsafe fn run_coerce_lenient(
    registry: &Registry,
    scalar_id: u32,
    json_in: *const c_char,
) -> ScalarResult {
    let Some(def) = registry.def(ScalarId(scalar_id)) else {
        return make_err(ErrorCategory::Parse, "unknown scalar id");
    };
    let decoded = match decode_cstr(json_in) {
        Ok(decoded) => decoded,
        Err(err) => return err,
    };
    let value: serde_json::Value = match serde_json::from_str(decoded) {
        Ok(value) => value,
        Err(_) => return make_err(ErrorCategory::Parse, "invalid json"),
    };
    guard(|| {
        let result = registry.coerce_lenient(&value, def.canonical);
        if let Some(err) = result.error {
            return Err(err);
        }
        // `value` is `Option<Value>`; a null passthrough serializes to "null".
        let json_out = serde_json::to_string(&result.value).map_err(|err| {
            ScalarError::new(ErrorKind::Custom, format!("failed to encode value: {err}"))
        })?;
        ScalarValue::from_string(json_out)
    })
}

/// Release a `ScalarResult`'s owned allocations.
///
/// # Safety
/// `result` must have been produced by this crate and not already freed.
pub unsafe fn free_result(result: ScalarResult) {
    result.value.free();
    if !result.error.is_null() {
        drop(CString::from_raw(result.error));
    }
}

/// Run a hook over `len` inputs in one crossing. Element `i` of the returned
/// array equals the single-call result for `inputs[i]`.
///
/// # Safety
/// `inputs` must point to `len` valid NUL-terminated C strings (or be null when
/// `len == 0`).
pub unsafe fn run_batch(
    registry: &Registry,
    scalar_id: u32,
    inputs: *const *const c_char,
    len: usize,
    hook: Hook,
) -> ScalarResultArray {
    if len == 0 || inputs.is_null() {
        return ScalarResultArray {
            ptr: ptr::null_mut(),
            len: 0,
        };
    }
    let elements = slice::from_raw_parts(inputs, len);
    let mut out: Vec<ScalarResult> = Vec::with_capacity(len);
    for &input in elements {
        out.push(run(registry, scalar_id, input, hook));
    }
    // `Box::into_raw` (not `as_mut_ptr` + `mem::forget`, which is UB under
    // Stacked Borrows) keeps the returned pointer valid for the caller's reads.
    let ptr = Box::into_raw(out.into_boxed_slice()).cast::<ScalarResult>();
    ScalarResultArray { ptr, len }
}

/// Release a `ScalarResultArray` and every result it owns.
///
/// # Safety
/// `array` must have been produced by `run_batch` and not already freed.
pub unsafe fn free_result_array(array: ScalarResultArray) {
    if array.ptr.is_null() {
        return;
    }
    let boxed = Box::from_raw(ptr::slice_from_raw_parts_mut(array.ptr, array.len));
    for result in boxed.into_vec() {
        free_result(result);
    }
}
