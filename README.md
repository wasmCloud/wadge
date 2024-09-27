# Description

`wadge` is WebAssembly bridge allowing native applications invoke Wasm functions, normally provided by a Wasm runtime.
Main use cases for this project are:
- Testing WebAssembly applications natively, using existing development tools (e.g. debuggers)
- Extending native applications with WebAssembly plugins

## Go

`wadge` provides `wadge-bindgen-go` utility, which walks the complete AST of your application and generates a binding file, `bindings.wadge.go` by default.

The bindings file utilizes `//go:linkname` directives to replace unimplemented functions with `//go:wasmimport` directives, normally provided by the runtime,
using a native implementation, which invokes those imports on a WebAssembly component running in an embedded `wadge` instance.

For testing, Go library provides `RunTest` function, which plugs into the Go standard library `testing` framework.

# Design

`wadge` uses [`cabish`](https://github.com/wasmCloud/cabish) to read and write values over FFI to pass to an embedded WebAssembly runtime ([Wasmtime](https://github.com/bytecodealliance/wasmtime)).
