// Package superscalar is the Go cgo binding into the superscalar core via the fixed C ABI.
package superscalar

/*
#cgo CFLAGS: -I${SRCDIR}/include
#include <stdlib.h>
#include "superscalar.h"

// include/superscalar.h is a committed copy of crates/ffi/superscalar.h, kept
// in sync by crates/ffi/scripts/check_header.sh, so the module zip that
// `go get` downloads carries its own header.
//
// Link (LDFLAGS) directives are per-platform in cgo_ldflags_<goos>_<goarch>.go so
// a Rust-free build links the prebuilt static archive under
// lib/<goos>_<goarch> (hermetic delivery: no Rust toolchain needed). Populate
// that dir with scripts/build_ffi.sh (host build) or
// scripts/fetch_release_archive.sh (release assets) before building a consumer.
*/
import "C"

import (
	"errors"
	"unsafe"
)

// readResult copies the string out of C memory before the deferred free runs, so it stays valid.
func readResult(res C.ScalarResult) (string, error) {
	defer C.scalar_result_free(res)
	if bool(res.ok) {
		if res.value.str_ptr != nil {
			return C.GoStringN((*C.char)(res.value.str_ptr), C.int(res.value.str_len)), nil
		}
		return "", nil
	}
	msg := "scalar error"
	if res.error != nil {
		msg = C.GoString(res.error)
	}
	return "", errors.New(msg)
}

func callScalarParse(id uint32, value string) (string, error) {
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_parse(C.uint32_t(id), cs))
}

func callScalarNormalize(id uint32, value string) (string, error) {
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_normalize(C.uint32_t(id), cs))
}

func callScalarValidate(id uint32, value string) error {
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	_, err := readResult(C.scalar_validate(C.uint32_t(id), cs))
	return err
}

// callScalarCoerceLenient runs the "flag, don't block" coercion: jsonStr is a
// JSON document, the returned string is the serialized LenientCoerceResult JSON
// ({"value":<json|null>,"error":<{kind,message}|null>}) on success. The C ABI
// surfaces both an unknown id / invalid json and a captured coercion failure as
// a non-nil error (mirroring the strict callScalar* helpers); the result string
// carries the canonical value when it is nil.
func callScalarCoerceLenient(id uint32, jsonStr string) (string, error) {
	cs := C.CString(jsonStr)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_coerce_lenient(C.uint32_t(id), cs))
}
