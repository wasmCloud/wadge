//go:generate go run github.com/wasmCloud/wadge/cmd/wadge-bindgen-go
//go:generate cargo build -p wasi-test-component --target wasm32-wasip1
//go:generate cp ../../../target/wasm32-wasip1/debug/wasi_test_component.wasm component.wasm

package wasi_test

import (
	"bytes"
	_ "embed"
	"io"
	"log"
	"log/slog"
	"net/http"
	"os"
	"testing"

	"github.com/stretchr/testify/assert"
	"github.com/wasmCloud/wadge"
	_ "github.com/wasmCloud/wadge/bindings"
	incominghandler "github.com/wasmCloud/wadge/tests/go/wasi/bindings/wasi/http/incoming-handler"
	"github.com/wasmCloud/wadge/wadgehttp"
)

//go:embed component.wasm
var component []byte

func init() {
	log.SetFlags(0)
	slog.SetDefault(slog.New(slog.NewTextHandler(os.Stderr, &slog.HandlerOptions{
		Level: slog.LevelDebug, ReplaceAttr: func(groups []string, a slog.Attr) slog.Attr {
			if a.Key == slog.TimeKey {
				return slog.Attr{}
			}
			return a
		},
	})))

	instance, err := wadge.NewInstance(&wadge.Config{
		Wasm: component,
	})
	if err != nil {
		log.Fatalf("failed to construct new instance: %s", err)
	}
	wadge.SetInstance(instance)
}

func TestIncomingHandler(t *testing.T) {
	wadge.RunTest(t, func() {
		req, err := http.NewRequest(http.MethodPost, "5", bytes.NewReader([]byte("foo bar baz")))
		if err != nil {
			t.Fatalf("failed to create new HTTP request: %s", err)
		}
		req.Header.Add("foo", "bar")
		req.Header.Add("foo", "baz")
		req.Header.Add("key", "value")
		resp, err := wadgehttp.HandleIncomingRequest(incominghandler.Exports.Handle, req)
		if err != nil {
			t.Fatalf("failed to handle incoming HTTP request: %s", err)
		}
		assert.Equal(t, 200, resp.StatusCode)
		assert.Equal(t, http.Header{
			"foo": {
				"bar",
				"baz",
			},
			"key": {
				"value",
			},
		}, resp.Header)
		buf, err := io.ReadAll(resp.Body)
		if err != nil {
			t.Fatalf("failed to read HTTP response body: %s", err)
		}
		if err := resp.Body.Close(); err != nil {
			t.Fatalf("failed to close response body: %s", err)
		}
		assert.Equal(t, []byte("🧭🧭🧭🧭🧭foo bar baz"), buf)
	})
}
