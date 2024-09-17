//go:generate go run github.com/rvolosatovs/west/cmd/west-bindgen-go -package bindings -output bindings.go

package bindings

import (
	_ "github.com/rvolosatovs/west/bindings/wasiext/http/ext"
)
