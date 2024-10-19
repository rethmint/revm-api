//go:build darwin && !sys_revm

package api

// #cgo LDFLAGS: -Wl,-rpath,${SRCDIR} -L${SRCDIR} -lrevmapi
import "C"
