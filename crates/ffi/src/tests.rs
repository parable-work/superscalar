//! The ABI tests, run against the built-in expansion of `export_c_abi!`.
//! The macro body is the only unsafe code a downstream expands, so proving it
//! here (and under miri in CI) proves it for every expansion.

use super::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::slice;
use superscalar::{scalar_for, Registry, ScalarId};

fn cstr(s: &str) -> CString {
    CString::new(s).expect("test input has no interior NUL")
}

#[test]
fn parse_valid_email_matches_core_and_frees() {
    let id = ScalarId::CONTACT_EMAIL.as_u32();
    let expected = scalar_for(ScalarId::CONTACT_EMAIL)
        .parse(Registry::builtin(), "Foo@Bar.com")
        .expect("valid email parses in core");
    let input = cstr("Foo@Bar.com");
    let result = unsafe { scalar_parse(id, input.as_ptr()) };
    assert!(result.ok);
    assert_eq!(result.value.kind, ValueKind::Str);
    assert!(result.error.is_null());
    let got = unsafe { CStr::from_ptr(result.value.str_ptr) }
        .to_str()
        .expect("canonical value is UTF-8");
    assert_eq!(got, expected);
    assert_eq!(result.value.str_len, expected.len());
    unsafe { scalar_result_free(result) };
}

fn error_message(result: &ScalarResult) -> String {
    assert!(!result.error.is_null());
    unsafe { CStr::from_ptr(result.error) }
        .to_str()
        .expect("error is UTF-8")
        .to_owned()
}

#[test]
fn parse_invalid_email_reports_pattern_error() {
    let result = unsafe {
        scalar_parse(
            ScalarId::CONTACT_EMAIL.as_u32(),
            cstr("not-an-email").as_ptr(),
        )
    };
    assert!(!result.ok);
    assert_eq!(result.error_category, ErrorCategory::Pattern);
    assert!(result.value.str_ptr.is_null());
    assert!(error_message(&result).starts_with("pattern:"));
    unsafe { scalar_result_free(result) };
}

#[test]
fn unknown_scalar_id_is_clean_error() {
    let result = unsafe { scalar_parse(9_999, cstr("anything").as_ptr()) };
    assert!(!result.ok);
    assert_eq!(result.error_category, ErrorCategory::Parse);
    assert_eq!(error_message(&result), "parse: unknown scalar id");
    unsafe { scalar_result_free(result) };
}

#[test]
fn null_input_is_clean_error() {
    let result = unsafe { scalar_parse(ScalarId::CONTACT_EMAIL.as_u32(), ptr::null()) };
    assert!(!result.ok);
    assert_eq!(error_message(&result), "parse: null input pointer");
    unsafe { scalar_result_free(result) };
}

#[test]
fn non_utf8_input_is_clean_error() {
    let bad = CString::new(vec![0xff_u8, 0xfe]).expect("no interior NUL");
    let result = unsafe { scalar_parse(ScalarId::CONTACT_EMAIL.as_u32(), bad.as_ptr()) };
    assert!(!result.ok);
    assert_eq!(error_message(&result), "parse: input was not valid UTF-8");
    unsafe { scalar_result_free(result) };
}

#[test]
fn normalize_matches_core() {
    for input in ["Foo@Bar.com", "  MixedCase@Example.COM  "] {
        let expected = scalar_for(ScalarId::CONTACT_EMAIL)
            .normalize(Registry::builtin(), input)
            .expect("normalizes");
        let result =
            unsafe { scalar_normalize(ScalarId::CONTACT_EMAIL.as_u32(), cstr(input).as_ptr()) };
        assert!(result.ok);
        let got = unsafe { CStr::from_ptr(result.value.str_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(got, expected);
        unsafe { scalar_result_free(result) };
    }
}

#[test]
fn validate_reports_ok_without_a_value() {
    let ok = unsafe {
        scalar_validate(
            ScalarId::CONTACT_EMAIL.as_u32(),
            cstr("foo@bar.com").as_ptr(),
        )
    };
    assert!(ok.ok);
    assert!(ok.value.str_ptr.is_null());
    unsafe { scalar_result_free(ok) };

    let bad = unsafe { scalar_validate(ScalarId::CONTACT_EMAIL.as_u32(), cstr("nope").as_ptr()) };
    assert!(!bad.ok);
    unsafe { scalar_result_free(bad) };
}

#[test]
fn guard_converts_panic_to_error() {
    let prev = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    let result = guard(|| panic!("boom"));
    std::panic::set_hook(prev);
    assert!(!result.ok);
    assert_eq!(result.error_category, ErrorCategory::Panic);
    assert!(error_message(&result).starts_with("panic:"));
    unsafe { scalar_result_free(result) };
}

#[test]
fn dispatch_conforms_to_core_across_scalars() {
    let accepts = [
        (ScalarId::CONTACT_EMAIL, "Alice@Example.com"),
        (
            ScalarId::IDENTITY_UUID,
            "550e8400-e29b-41d4-a716-446655440000",
        ),
        (ScalarId::DESIGN_COLOR, "#ff0000"),
    ];
    for (id, input) in accepts {
        let expected = scalar_for(id)
            .parse(Registry::builtin(), input)
            .expect("core accepts sample");
        let result = unsafe { scalar_parse(id.as_u32(), cstr(input).as_ptr()) };
        assert!(result.ok, "{input} should parse for {id:?}");
        let got = unsafe { CStr::from_ptr(result.value.str_ptr) }
            .to_str()
            .unwrap();
        assert_eq!(got, expected, "{id:?}");
        unsafe { scalar_result_free(result) };
    }

    let rejects = [
        (ScalarId::IDENTITY_UUID, "not-a-uuid"),
        (ScalarId::CONTACT_EMAIL, ""),
    ];
    for (id, input) in rejects {
        assert!(
            scalar_for(id).parse(Registry::builtin(), input).is_err(),
            "core rejects {input} for {id:?}"
        );
        let result = unsafe { scalar_parse(id.as_u32(), cstr(input).as_ptr()) };
        assert!(!result.ok, "{input} should reject for {id:?}");
        unsafe { scalar_result_free(result) };
    }
}

// Every ValueKind round-trips through its marshaling helper and frees
// cleanly (run under miri/ASAN in CI for the no-leak proof).
#[test]
fn every_value_kind_round_trips_and_frees() {
    let str_val = ScalarValue::from_string("canonical".to_owned()).expect("no interior NUL");
    assert_eq!(str_val.kind, ValueKind::Str);
    assert_eq!(str_val.str_len, "canonical".len());
    let got = unsafe { CStr::from_ptr(str_val.str_ptr) }.to_str().unwrap();
    assert_eq!(got, "canonical");
    unsafe { str_val.free() };

    let int_val = ScalarValue::from_int(-42);
    assert_eq!(int_val.kind, ValueKind::Int);
    assert_eq!(int_val.int_val, -42);
    unsafe { int_val.free() };

    let float_val = ScalarValue::from_float(2.5);
    assert_eq!(float_val.kind, ValueKind::Float);
    assert_eq!(float_val.float_val, 2.5);
    unsafe { float_val.free() };

    let bool_val = ScalarValue::from_bool(true);
    assert_eq!(bool_val.kind, ValueKind::Bool);
    assert!(bool_val.bool_val);
    unsafe { bool_val.free() };

    let bytes_val = ScalarValue::from_bytes(vec![1, 2, 3, 0, 255]);
    assert_eq!(bytes_val.kind, ValueKind::Bytes);
    assert_eq!(bytes_val.bytes_len, 5);
    let bytes = unsafe { slice::from_raw_parts(bytes_val.bytes_ptr, bytes_val.bytes_len) };
    assert_eq!(bytes, &[1, 2, 3, 0, 255]);
    unsafe { bytes_val.free() };
}

// Free-discipline smoke: parse+free cycles guard against use-after-free /
// double-free; the no-leak assertion runs under miri/ASAN in CI.
#[test]
fn repeated_parse_and_free_is_stable() {
    let id = ScalarId::CONTACT_EMAIL.as_u32();
    let iterations = if cfg!(miri) { 16 } else { 10_000 };
    for i in 0..iterations {
        let input = if i % 2 == 0 {
            cstr("ok@example.com")
        } else {
            cstr("bad")
        };
        let result = unsafe { scalar_parse(id, input.as_ptr()) };
        unsafe { scalar_result_free(result) };
    }
}

fn snapshot(result: &ScalarResult) -> (bool, ErrorCategory, Option<String>) {
    if result.ok && !result.value.str_ptr.is_null() {
        let value = unsafe { CStr::from_ptr(result.value.str_ptr) }
            .to_str()
            .unwrap()
            .to_owned();
        (true, result.error_category, Some(value))
    } else if result.ok {
        (true, result.error_category, None)
    } else {
        (false, result.error_category, Some(error_message(result)))
    }
}

#[test]
fn parse_batch_equals_n_singles() {
    let id = ScalarId::CONTACT_EMAIL.as_u32();
    let inputs = ["Foo@Bar.com", "not-an-email", "X@Y.io", ""];
    let owned: Vec<CString> = inputs.iter().map(|s| cstr(s)).collect();
    let ptrs: Vec<*const c_char> = owned.iter().map(|c| c.as_ptr()).collect();

    let array = unsafe { scalar_parse_batch(id, ptrs.as_ptr(), ptrs.len()) };
    assert_eq!(array.len, inputs.len());
    let batch = unsafe { slice::from_raw_parts(array.ptr, array.len) };

    for (i, input) in inputs.iter().enumerate() {
        let single = unsafe { scalar_parse(id, cstr(input).as_ptr()) };
        assert_eq!(
            snapshot(&batch[i]),
            snapshot(&single),
            "element {i} ({input})"
        );
        unsafe { scalar_result_free(single) };
    }
    unsafe { scalar_result_array_free(array) };
}

#[test]
fn empty_batch_is_null_and_frees() {
    let array = unsafe { scalar_parse_batch(ScalarId::CONTACT_EMAIL.as_u32(), ptr::null(), 0) };
    assert!(array.ptr.is_null());
    assert_eq!(array.len, 0);
    unsafe { scalar_result_array_free(array) };
}

// Regression: the batch array pointer must stay valid for reads through
// `array.ptr` after `run_batch` returns (the old `as_mut_ptr` + `mem::forget`
// idiom was UB under Stacked Borrows, caught by miri).
#[test]
fn batch_pointer_is_valid_for_reads_after_return() {
    let id = ScalarId::CONTACT_EMAIL.as_u32();
    let owned: Vec<CString> = ["a@b.com", "bad", "c@d.io"]
        .iter()
        .map(|s| cstr(s))
        .collect();
    let ptrs: Vec<*const c_char> = owned.iter().map(|c| c.as_ptr()).collect();

    let array = unsafe { scalar_parse_batch(id, ptrs.as_ptr(), ptrs.len()) };
    assert_eq!(array.len, 3);
    assert!(!array.ptr.is_null());

    let batch = unsafe { slice::from_raw_parts(array.ptr, array.len) };
    assert!(batch[0].ok);
    let first = unsafe { CStr::from_ptr(batch[0].value.str_ptr) }
        .to_str()
        .unwrap();
    assert_eq!(first, "a@b.com");
    assert!(!batch[1].ok);
    assert!(batch[2].ok);

    unsafe { scalar_result_array_free(array) };
}

#[test]
fn coerce_lenient_trims_a_money_string_and_frees() {
    // Finance.Money is an Int scalar: a padded numeric string coerces to the
    // bare integer, encoded as a JSON document ("12345").
    let id = ScalarId::FINANCE_MONEY.as_u32();
    let input = cstr("\" 12345 \"");
    let result = unsafe { scalar_coerce_lenient(id, input.as_ptr()) };
    assert!(result.ok, "money string should coerce");
    assert_eq!(result.value.kind, ValueKind::Str);
    let got = unsafe { CStr::from_ptr(result.value.str_ptr) }
        .to_str()
        .unwrap();
    assert_eq!(got, "12345");
    unsafe { scalar_result_free(result) };
}

#[test]
fn coerce_lenient_reports_a_bad_date_as_error() {
    let id = ScalarId::TEMPORAL_DATE.as_u32();
    let input = cstr("\"not-a-date\"");
    let result = unsafe { scalar_coerce_lenient(id, input.as_ptr()) };
    assert!(!result.ok, "an unparseable date should fail");
    assert!(result.value.str_ptr.is_null());
    assert!(!result.error.is_null());
    unsafe { scalar_result_free(result) };
}

#[test]
fn coerce_lenient_null_passthrough_is_a_null_value() {
    let id = ScalarId::FINANCE_MONEY.as_u32();
    let input = cstr("null");
    let result = unsafe { scalar_coerce_lenient(id, input.as_ptr()) };
    assert!(result.ok, "a JSON null passes through cleanly");
    let got = unsafe { CStr::from_ptr(result.value.str_ptr) }
        .to_str()
        .unwrap();
    assert_eq!(got, "null");
    unsafe { scalar_result_free(result) };
}

#[test]
fn coerce_lenient_invalid_json_is_clean_error() {
    let id = ScalarId::FINANCE_MONEY.as_u32();
    let result = unsafe { scalar_coerce_lenient(id, cstr("{not json").as_ptr()) };
    assert!(!result.ok);
    assert_eq!(result.error_category, ErrorCategory::Parse);
    assert_eq!(error_message(&result), "parse: invalid json");
    unsafe { scalar_result_free(result) };
}

#[test]
fn coerce_lenient_unknown_id_is_clean_error() {
    let result = unsafe { scalar_coerce_lenient(9_999, cstr("123").as_ptr()) };
    assert!(!result.ok);
    assert_eq!(error_message(&result), "parse: unknown scalar id");
    unsafe { scalar_result_free(result) };
}
