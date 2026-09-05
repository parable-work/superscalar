//go:build linux && arm64

package superscalar

// #cgo LDFLAGS: -L${SRCDIR}/lib/linux_arm64 -lsuperscalar_ffi -lm -ldl -lpthread
import "C"
