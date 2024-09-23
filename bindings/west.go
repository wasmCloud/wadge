//go:generate go run github.com/wasmCloud/west/cmd/west-bindgen-go -package bindings -output bindings.go

package bindings

import (
	_ "github.com/wasmCloud/west/bindings/wasiext/http/ext"
)
