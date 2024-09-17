//go:generate go run github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go generate -w imports -o bindings ./wit

package west

// #include "./include/west.h"
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

func SetErrorHandler(f func(error)) func(error) {
	errorHandlerMu.Lock()
	defer errorHandlerMu.Unlock()
	return setErrorHandler(f)
}

func WithErrorHandler(handler func(error), f func()) {
	errorHandlerMu.Lock()
	defer errorHandlerMu.Unlock()

	handler = setErrorHandler(handler)
	defer setErrorHandler(handler)

	f()
}

func CurrentErrorHandler() func(error) {
	return errorHandler.Load().(func(error))
}

func WithCurrentErrorHandler(f func(func(error))) {
	errorHandlerMu.RLock()
	defer errorHandlerMu.RUnlock()
	f(CurrentErrorHandler())
}

func setInstance(i *Instance) *Instance {
	i, instance = instance, i
	return i
}

func SetInstance(i *Instance) *Instance {
	instanceMu.Lock()
	defer instanceMu.Unlock()

	return setInstance(i)
}

func WithInstance(i *Instance, f func()) {
	instanceMu.Lock()
	defer instanceMu.Unlock()

	i = setInstance(i)
	defer setInstance(i)

	f()
}

func withCurrentInstance[T any](f func(*Instance) T, handleErr func(error)) T {
	instanceMu.RLock()
	defer instanceMu.RUnlock()
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
	return f(instance)
}

func WithCurrentInstance[T any](f func(*Instance) T) T {
	return withCurrentInstance(f, func(err error) {
		log.Fatal(err)
	})
}

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

type Instance struct {
	ptr unsafe.Pointer
}

type Config struct {
	Wasm []byte
}

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
			len: C.ulong(len(wasm)),
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
