//go:generate go run go.bytecodealliance.org/cmd/wit-bindgen-go generate -w guest -o bindings ../../wit/sync

package sync

import (
	_ "go.wasmcloud.dev/wadge/tests/go/sync/bindings/wadge-test/sync/sync"
)
