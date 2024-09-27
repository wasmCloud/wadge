//go:build wasm || wasi || wasip1 || wasip2 || wasm_unknown || tinygo.wasm

package wadge

import "testing"

// RunTest simply calls `f`. This function is only defined for WebAssembly targets
// to allow unit tests relying on `wadge` in native environments to compile for Wasm.
func RunTest(_ *testing.T, f func()) {
	f()
}
