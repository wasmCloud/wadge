//go:build !wasm && !wasi && !wasip1 && !wasip2 && !wasm_unknown && !tinygo.wasm

package wadge

// #cgo               LDFLAGS: -lwadge
// #cgo android,arm64 LDFLAGS: -L${SRCDIR}/lib/aarch64-android
// #cgo darwin,amd64  LDFLAGS: -L${SRCDIR}/lib/x86_64-darwin
// #cgo darwin,arm64  LDFLAGS: -L${SRCDIR}/lib/aarch64-darwin
// #cgo linux,amd64   LDFLAGS: -L${SRCDIR}/lib/x86_64-linux
// #cgo linux,arm64   LDFLAGS: -L${SRCDIR}/lib/aarch64-linux
// #cgo linux,riscv64 LDFLAGS: -L${SRCDIR}/lib/riscv64-linux
// #cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/x86_64-windows
// #cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/aarch64-windows
// #cgo !windows      LDFLAGS: -lm -ldl -pthread
// #cgo windows       LDFLAGS: -lws2_32 -lole32 -loleaut32 -lntdll -lbcrypt -luserenv
// #include "./include/wadge.h"
// #include <stdlib.h>
import "C"

import (
	_ "embed"
	"errors"
	"fmt"
	"log"
	"runtime"
	"sync"
	"sync/atomic"
	"testing"
	"unsafe"
)

// Passthrough is a default passthrough Wasm component, which
// reexports all standard WASI interfaces
//
//go:embed lib/passthrough.wasm
var Passthrough []byte

var (
	errorHandlerMu sync.RWMutex
	errorHandler   atomic.Value

	instanceMu sync.RWMutex
	instance   *Instance
)

func init() {
	errorHandler.Store(func(err error) {
		log.Fatalf("failed to call instance: %s", err)
	})
}

func setErrorHandler(f func(error)) func(error) {
	return errorHandler.Swap(f).(func(error))
}

// SetErrorHandler atomically sets a global error handler and returns previous value.
// SetErrorHandler is safe for concurrent use.
func SetErrorHandler(f func(error)) func(error) {
	errorHandlerMu.Lock()
	defer errorHandlerMu.Unlock()
	return setErrorHandler(f)
}

// WithErrorHandler executes `f` with `handler` as the global error handler
// resetting it back to previous value once `f` returns
// WithErrorHandler is safe for concurrent use, but calling it from within `handler`
// or `f` will cause a deadlock
func WithErrorHandler(handler func(error), f func()) {
	errorHandlerMu.Lock()
	defer errorHandlerMu.Unlock()

	handler = setErrorHandler(handler)
	defer setErrorHandler(handler)

	f()
}

// CurrentErrorHandler atomically loads the current global error handler.
// CurrentErrorHandler is safe for concurrent use.
func CurrentErrorHandler() func(error) {
	return errorHandler.Load().(func(error))
}

// WithCurrentErrorHandler executes `f` with current global error handler.
// WithCurrentErrorHandler is safe for concurrent use, but calling it from within
// `f` will cause a deadlock.
func WithCurrentErrorHandler(f func(func(error))) {
	errorHandlerMu.RLock()
	defer errorHandlerMu.RUnlock()
	f(CurrentErrorHandler())
}

func setInstance(i *Instance) *Instance {
	i, instance = instance, i
	return i
}

// SetInstance atomically sets a global `wadge` Instance.
// SetInstance is safe for concurrent use.
func SetInstance(i *Instance) *Instance {
	instanceMu.Lock()
	defer instanceMu.Unlock()

	return setInstance(i)
}

// WithInstance executes `f` with `i` as the global `wadge` Instance
// resetting it back to previous value once `f` returns
// WithInstance is safe for concurrent use, but calling it from within
// `f` will cause a deadlock
func WithInstance(i *Instance, f func()) {
	instanceMu.Lock()
	defer instanceMu.Unlock()

	i = setInstance(i)
	defer setInstance(i)

	f()
}

func withCurrentInstance[T any](f func(*Instance) T, handleErr func(error)) T {
	instanceMu.RLock()
	if instance == nil {
		instanceMu.RUnlock()
		func() {
			instanceMu.Lock()
			defer instanceMu.Unlock()

			if instance == nil {
				i, err := NewInstance(nil)
				if err != nil {
					handleErr(err)
				}
				instance = i
			}
		}()
		instanceMu.RLock()
	}
	defer instanceMu.RUnlock()
	return f(instance)
}

// WithCurrentInstance executes `f` with current global `wadge` Instance.
// WithCurrentInstance is safe for concurrent use, but calling it from within
// `f` will cause a deadlock.
// If no global `wadge` Instance has been configured, WithCurrentInstance will
// attempt to use `Passthrough`. It will call `log.Fatal` if instantiating it fails.
func WithCurrentInstance[T any](f func(*Instance) T) T {
	return withCurrentInstance(f, func(err error) {
		log.Fatal(err)
	})
}

// RunTest executes `f` with current global `wadge` Instance and global error handler,
// which calls `t.Fatal` on errors.
// RunTest is safe for concurrent use, but calling it from within
// `f` will cause a deadlock.
func RunTest(t *testing.T, f func()) {
	WithErrorHandler(
		func(err error) {
			t.Fatalf("failed to call instance: %s", err)
		},
		func() {
			withCurrentInstance(
				func(_ *Instance) struct{} {
					f()
					return struct{}{}
				},
				func(err error) {
					t.Fatal(err)
				},
			)
		},
	)
}

// Instance is an instantiated Wasm component in `wadge` runtime
type Instance struct {
	ptr unsafe.Pointer
}

// Config is `wadge` runtime configuration
type Config struct {
	// Wasm is the component bytes to instantiate, this can either be
	// binary Wasm or WAT.
	// In case a Wasm module is specified here, the runtime will componentize it.
	Wasm []byte
}

// NewInstance instantiates a new Wasm component in `wadge` runtime given a `Config`.
func NewInstance(conf *Config) (*Instance, error) {
	var pinner runtime.Pinner
	defer pinner.Unpin()

	wasm := Passthrough
	if conf != nil {
		if len(conf.Wasm) > 0 {
			wasm = conf.Wasm
		}
	}
	wasmPtr := unsafe.SliceData(wasm)
	pinner.Pin(wasmPtr)
	ptr := C.instance_new(C.Config{
		wasm: C.List_u8{
			ptr: (*C.uchar)(wasmPtr),
			len: C.uintptr_t(len(wasm)),
		},
	})
	if ptr == nil {
		n := C.error_len()
		buf := make([]C.char, n)
		if n = C.error_take(unsafe.SliceData(buf), n); n > 0 {
			err := errors.New(C.GoStringN(unsafe.SliceData(buf), C.int(n)))
			return nil, fmt.Errorf("failed to create an instance: %w", err)
		} else {
			return nil, errors.New("failed to create an instance")
		}
	}
	instance := &Instance{ptr: ptr}
	runtime.SetFinalizer(instance, func(instance *Instance) {
		C.instance_free(instance.ptr)
	})
	return instance, nil
}

// Call calls function `name` within `instance` with arguments passed according to
// `cabish` specification
func (i Instance) Call(instance string, name string, args ...unsafe.Pointer) error {
	instanceC := C.CString(instance)
	defer C.free(unsafe.Pointer(instanceC))
	nameC := C.CString(name)
	defer C.free(unsafe.Pointer(nameC))

	if !C.instance_call(i.ptr, instanceC, nameC, unsafe.SliceData(args)) {
		n := C.error_len()
		buf := make([]C.char, n)
		if n = C.error_take(unsafe.SliceData(buf), n); n > 0 {
			err := errors.New(C.GoStringN(unsafe.SliceData(buf), C.int(n)))
			return fmt.Errorf("failed to call function on an instance: %w", err)
		} else {
			return errors.New("failed to call function on an instance")
		}
	}
	return nil
}
