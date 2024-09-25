//go:generate go run github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go generate -w guest -o bindings ../../wit/sync

package sync

import (
	_ "go.wasmcloud.dev/wadge/tests/go/sync/bindings/wadge-test/sync/sync"
)
