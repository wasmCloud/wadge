//go:generate go run ../cmd/west-bindgen-go -package bindings -output bindings.go

package bindings

import (
	_ "github.com/rvolosatovs/west/go/bindings/west/test/http-test"
)
