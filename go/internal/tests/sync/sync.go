//go:generate go run github.com/ydnar/wasm-tools-go/cmd/wit-bindgen-go@v0.1.5 generate -w guest -o bindings ../../../../tests/wit/sync
//go:generate go run ../../../cmd/west-bindgen-go

package sync

import (
	_ "github.com/rvolosatovs/west/go/internal/tests/sync/bindings/west-test/sync/sync"
)
