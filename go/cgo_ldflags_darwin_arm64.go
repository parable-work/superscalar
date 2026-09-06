//go:build darwin && arm64

package superscalar

// #cgo LDFLAGS: -L${SRCDIR}/lib/darwin_arm64 -lsuperscalar_ffi -liconv
import "C"
