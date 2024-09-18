//go:build dev

//go:generate cargo build -p west-sys
//go:generate cargo build -p west-passthrough --target wasm32-unknown-unknown
//go:generate wasm-tools component new target/wasm32-unknown-unknown/debug/west_passthrough.wasm -o lib/passthrough.wasm

package west

// #cgo LDFLAGS: -L${SRCDIR}/target/debug -lwest_sys
import "C"
