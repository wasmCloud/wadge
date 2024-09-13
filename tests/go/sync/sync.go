//go:generate go run github.com/ydnar/wasm-tools-go/cmd/wit-bindgen-go@v0.1.5 generate -w guest -o bindings ../../wit/sync

package sync

import (
	_ "github.com/rvolosatovs/west/tests/go/sync/bindings/west-test/sync/sync"
)
