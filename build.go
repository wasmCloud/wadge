//go:build !dev

//go:generate cargo build -p west-sys --release
//go:generate cargo build -p west-passthrough --target wasm32-unknown-unknown --release
//go:generate wasm-tools component new target/wasm32-unknown-unknown/release/west_passthrough.wasm -o lib/passthrough.wasm

package west

// #cgo darwin,amd64  LDFLAGS: -L${SRCDIR}/lib/x86_64-darwin   -lwest
// #cgo darwin,arm64  LDFLAGS: -L${SRCDIR}/lib/aarch64-darwin  -lwest
// #cgo linux,amd64   LDFLAGS: -L${SRCDIR}/lib/x86_64-linux    -lwest
// #cgo linux,arm64   LDFLAGS: -L${SRCDIR}/lib/aarch64-linux   -lwest
// #cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/x86_64-windows  -lwest
// #cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/aarch64-windows -lwest
import "C"
