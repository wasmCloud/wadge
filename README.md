# Description

`west` is a testing framework, which lets you test your WebAssembly applications natively and using all your existing development tools (e.g. debuggers).

## Go

`west` provides `west-bindgen-go` utility, which walks the complete AST of your application and generates a binding file, which replaces functions with `wasmimport` directives, normally provided by the runtime, by a native implementation, which invokes those imports on a WebAssembly component (harness) running in an embedded Wasmtime instance. This lets you test you applications without changing implementation and achieving full test coverage.

# Design

`west` uses [`cabish`](https://github.com/wasmCloud/cabish) to read and write values over FFI to pass to embedded Wasmtime instance.
