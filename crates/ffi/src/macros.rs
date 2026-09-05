//! `export_c_abi!`: the exported C symbols, emitted over a registry.

/// Emit the C ABI entry points over the registry that `$registry()` returns.
///
/// `$registry` is a path to a `fn() -> &'static Registry`, for example
/// `superscalar::Registry::builtin` or a downstream `acme::registry`.
/// The expansion is the nine `extern "C"` functions declared in
/// `superscalar.h`: `scalar_parse`, `scalar_normalize`, `scalar_validate`,
/// `scalar_coerce_lenient`, `scalar_result_free`, `scalar_parse_batch`,
/// `scalar_normalize_batch`, `scalar_validate_batch`,
/// `scalar_result_array_free`. Their names and signatures are the ABI and do
/// not vary with the registry.
///
/// The long form takes an `extra` block of items. They are emitted verbatim,
/// after `scalar_coerce_lenient` and before `scalar_result_free`, so a header
/// regenerated from the expansion keeps the order of the committed one. Items
/// written beside the macro call export just as well; the block only fixes
/// their position.
///
/// ```ignore
/// superscalar_ffi::export_c_abi!(superscalar::Registry::builtin);
///
/// superscalar_ffi::export_c_abi! {
///     registry = my_extension::registry,
///     extra = {
///         #[no_mangle]
///         pub unsafe extern "C" fn my_hash(json_in: *const c_char) -> ScalarResult { .. }
///     }
/// }
/// ```
///
/// `#[no_mangle]` exports the function name verbatim, so one cdylib invokes
/// this macro once, at the crate root. The macro reaches its helpers through
/// `$crate::`; the caller needs no imports beyond the ones its `extra` items
/// use.
///
/// The public arms of the macro refuse to expand in a build of this crate that
/// already carries the built-in assembly (`BUILTIN_ASSEMBLY`, the `builtin`
/// feature): a second expansion is a compile error naming the fix, instead of
/// an archive with two definitions of every symbol. The `@emit` arm is the
/// crate's own entry for that assembly and skips the check; a downstream has no
/// reason to use it.
#[macro_export]
macro_rules! export_c_abi {
    ($registry:path) => {
        $crate::export_c_abi! { registry = $registry, extra = {} }
    };
    (registry = $registry:path, extra = { $($extra:item)* }) => {
        const _: () = ::core::assert!(
            !$crate::BUILTIN_ASSEMBLY,
            "superscalar-ffi was built with its `builtin` feature and already exports the C ABI over Registry::builtin(); depend on it with default-features = false"
        );
        $crate::export_c_abi! { @emit registry = $registry, extra = { $($extra)* } }
    };
    (@emit registry = $registry:path, extra = { $($extra:item)* }) => {
        /// Validate shape and return the canonical normalized value.
        ///
        /// # Safety
        /// `input` must be null or a valid NUL-terminated C string valid for the call.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_parse(
            scalar_id: u32,
            input: *const ::std::os::raw::c_char,
        ) -> $crate::ScalarResult {
            $crate::run($registry(), scalar_id, input, $crate::Hook::Parse)
        }

        /// Transform toward canonical form without enforcing shape.
        ///
        /// # Safety
        /// `input` must be null or a valid NUL-terminated C string valid for the call.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_normalize(
            scalar_id: u32,
            input: *const ::std::os::raw::c_char,
        ) -> $crate::ScalarResult {
            $crate::run($registry(), scalar_id, input, $crate::Hook::Normalize)
        }

        /// Enforce shape; on success `value` is empty (read `ok`).
        ///
        /// # Safety
        /// `input` must be null or a valid NUL-terminated C string valid for the call.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_validate(
            scalar_id: u32,
            input: *const ::std::os::raw::c_char,
        ) -> $crate::ScalarResult {
            $crate::run($registry(), scalar_id, input, $crate::Hook::Validate)
        }

        /// Lenient ("flag, don't block") coercion: JSON value in, JSON value out.
        ///
        /// Unlike `scalar_parse`/`normalize`/`validate` (string in, string out), this
        /// carries the input and output as encoded JSON strings and reuses the same
        /// `ScalarResult`. `json_in` is a JSON document; on success `value` is the
        /// canonical coercion encoded as a JSON string (`"null"` for a null
        /// passthrough); on a coercion failure the core `ScalarError` is mapped onto
        /// `error`/`error_category` exactly as the strict hooks map theirs.
        ///
        /// # Safety
        /// `json_in` must be null or a valid NUL-terminated C string valid for the call.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_coerce_lenient(
            scalar_id: u32,
            json_in: *const ::std::os::raw::c_char,
        ) -> $crate::ScalarResult {
            $crate::run_coerce_lenient($registry(), scalar_id, json_in)
        }

        $($extra)*

        /// Release a `ScalarResult`'s owned allocations.
        ///
        /// # Safety
        /// `result` must have been produced by this crate and not already freed.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_result_free(result: $crate::ScalarResult) {
            $crate::free_result(result)
        }

        /// Batch `scalar_parse`. See `run_batch`.
        ///
        /// # Safety
        /// `inputs` must point to `len` valid NUL-terminated C strings.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_parse_batch(
            scalar_id: u32,
            inputs: *const *const ::std::os::raw::c_char,
            len: usize,
        ) -> $crate::ScalarResultArray {
            $crate::run_batch($registry(), scalar_id, inputs, len, $crate::Hook::Parse)
        }

        /// Batch `scalar_normalize`. See `run_batch`.
        ///
        /// # Safety
        /// `inputs` must point to `len` valid NUL-terminated C strings.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_normalize_batch(
            scalar_id: u32,
            inputs: *const *const ::std::os::raw::c_char,
            len: usize,
        ) -> $crate::ScalarResultArray {
            $crate::run_batch($registry(), scalar_id, inputs, len, $crate::Hook::Normalize)
        }

        /// Batch `scalar_validate`. See `run_batch`.
        ///
        /// # Safety
        /// `inputs` must point to `len` valid NUL-terminated C strings.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_validate_batch(
            scalar_id: u32,
            inputs: *const *const ::std::os::raw::c_char,
            len: usize,
        ) -> $crate::ScalarResultArray {
            $crate::run_batch($registry(), scalar_id, inputs, len, $crate::Hook::Validate)
        }

        /// Release a `ScalarResultArray` and every result it owns.
        ///
        /// # Safety
        /// `array` must have been produced by a `*_batch` function and not already
        /// freed.
        #[no_mangle]
        pub unsafe extern "C" fn scalar_result_array_free(array: $crate::ScalarResultArray) {
            $crate::free_result_array(array)
        }
    };
}
