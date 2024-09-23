//go:generate go run github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go generate -w guest -o bindings ../../wit/sync

package sync

import (
	_ "github.com/wasmCloud/west/tests/go/sync/bindings/west-test/sync/sync"
)
