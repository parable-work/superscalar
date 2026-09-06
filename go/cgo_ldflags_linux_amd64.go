//go:build linux && amd64

package superscalar

// #cgo LDFLAGS: -L${SRCDIR}/lib/linux_amd64 -lsuperscalar_ffi -lm -ldl -lpthread
import "C"
