//go:generate go run github.com/wasmCloud/wadge/cmd/wadge-bindgen-go -package bindings -output bindings.go

package bindings

import (
	_ "github.com/wasmCloud/wadge/bindings/wasiext/http/ext"
)
