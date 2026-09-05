//go:build darwin && amd64

package superscalar

// #cgo LDFLAGS: -L${SRCDIR}/lib/darwin_amd64 -lsuperscalar_ffi -liconv
import "C"
