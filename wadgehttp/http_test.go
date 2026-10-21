package wadgehttp

import (
	"net/http"
	"testing"

	"go.wasmcloud.dev/wadge"
)

func TestNewOutgoingRequestPreservesAuthorityPort(t *testing.T) {
	wadge.RunTest(t, func() {
		req, err := http.NewRequest(http.MethodGet, "http://example.com:8443/foo?bar=baz", nil)
		if err != nil {
			t.Fatalf("failed to create request: %s", err)
		}

		out, _, err := NewOutgoingRequest(req)
		if err != nil {
			t.Fatalf("failed to construct outgoing request: %s", err)
		}
		defer out.ResourceDrop()

		authority := out.Authority()
		auth := authority.Some()
		if auth == nil {
			t.Fatal("expected authority to be set")
		}
		if *auth != "example.com:8443" {
			t.Fatalf("unexpected authority: got %q, want %q", *auth, "example.com:8443")
		}
	})
}

func TestNewOutgoingRequestUsesRequestHostWhenURLHostIsEmpty(t *testing.T) {
	wadge.RunTest(t, func() {
		req, err := http.NewRequest(http.MethodGet, "/foo?bar=baz", nil)
		if err != nil {
			t.Fatalf("failed to create request: %s", err)
		}
		req.Host = "example.com:8443"

		out, _, err := NewOutgoingRequest(req)
		if err != nil {
			t.Fatalf("failed to construct outgoing request: %s", err)
		}
		defer out.ResourceDrop()

		authority := out.Authority()
		auth := authority.Some()
		if auth == nil {
			t.Fatal("expected authority to be set from request host")
		}
		if *auth != "example.com:8443" {
			t.Fatalf("unexpected authority: got %q, want %q", *auth, "example.com:8443")
		}
	})
}
