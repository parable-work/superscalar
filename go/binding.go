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

// cScalarNames holds one C copy of every canonical name in the registry, made
// once at init and never freed, so a call does not allocate for the name. The
// map is read-only after init and safe for concurrent use.
var cScalarNames = func() map[string]*C.char {
	names := make(map[string]*C.char, len(VALID_SCALARS))
	for _, name := range VALID_SCALARS {
		names[string(name)] = C.CString(string(name))
	}
	return names
}()

// cScalarName returns the C form of a canonical name. A name outside the
// registry gets a fresh copy the caller frees (owned is true); the core then
// reports it as an unknown scalar.
func cScalarName(canonical string) (name *C.char, owned bool) {
	if name, ok := cScalarNames[canonical]; ok {
		return name, false
	}
	return C.CString(canonical), true
}

func callScalarParse(canonical string, value string) (string, error) {
	cn, owned := cScalarName(canonical)
	if owned {
		defer C.free(unsafe.Pointer(cn))
	}
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_parse(cn, cs))
}

func callScalarNormalize(canonical string, value string) (string, error) {
	cn, owned := cScalarName(canonical)
	if owned {
		defer C.free(unsafe.Pointer(cn))
	}
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_normalize(cn, cs))
}

func callScalarValidate(canonical string, value string) error {
	cn, owned := cScalarName(canonical)
	if owned {
		defer C.free(unsafe.Pointer(cn))
	}
	cs := C.CString(value)
	defer C.free(unsafe.Pointer(cs))
	_, err := readResult(C.scalar_validate(cn, cs))
	return err
}

// callScalarCoerceLenient runs the "flag, don't block" coercion: jsonStr is a
// JSON document, the returned string is the serialized LenientCoerceResult JSON
// ({"value":<json|null>,"error":<{kind,message}|null>}) on success. The C ABI
// surfaces both an unknown scalar / invalid json and a captured coercion failure
// as a non-nil error (mirroring the strict callScalar* helpers); the result
// string carries the canonical value when it is nil.
func callScalarCoerceLenient(canonical string, jsonStr string) (string, error) {
	cn, owned := cScalarName(canonical)
	if owned {
		defer C.free(unsafe.Pointer(cn))
	}
	cs := C.CString(jsonStr)
	defer C.free(unsafe.Pointer(cs))
	return readResult(C.scalar_coerce_lenient(cn, cs))
}
