//go:generate go run go.wasmcloud.dev/wadge/cmd/wadge-bindgen-go -package bindings -output bindings.go

package bindings

import (
	_ "go.wasmcloud.dev/wadge/bindings/wasiext/http/ext"
)
