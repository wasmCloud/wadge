//go:build !dev

//go:generate cargo build -p west-sys --release
//go:generate cargo build -p west-passthrough --target wasm32-unknown-unknown --release
//go:generate wasm-tools component new target/wasm32-unknown-unknown/release/west_passthrough.wasm -o lib/passthrough.wasm

package west

// #cgo               LDFLAGS: -lwest
// #cgo linux         LDFLAGS: -lm
// #cgo darwin,amd64  LDFLAGS: -L${SRCDIR}/lib/x86_64-darwin
// #cgo darwin,arm64  LDFLAGS: -L${SRCDIR}/lib/aarch64-darwin
// #cgo linux,amd64   LDFLAGS: -L${SRCDIR}/lib/x86_64-linux
// #cgo linux,arm64   LDFLAGS: -L${SRCDIR}/lib/aarch64-linux
// #cgo linux,riscv64 LDFLAGS: -L${SRCDIR}/lib/riscv64-linux
// #cgo windows,amd64 LDFLAGS: -L${SRCDIR}/lib/x86_64-windows
// #cgo windows,arm64 LDFLAGS: -L${SRCDIR}/lib/aarch64-windows
import "C"
