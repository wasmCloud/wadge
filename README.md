# `wadge`: a WebAssembly bridge

`wadge` is a WebAssembly "bridge" enabling native applications to invoke Wasm functions that are ordinarily provided by a Wasm runtime. Use cases for this project include:

- Testing WebAssembly applications natively, using existing development tools (e.g. debuggers)
- Extending native applications with WebAssembly plugins

Currently, `wadge` supports Go applications written for compilation to WebAssembly components.

## Bridging Wasm functions and native code

In the testing case, when you write a component using [WebAssembly Interface Type (WIT)](https://component-model.bytecodealliance.org/design/wit.html) interfaces, standard Go tools like `go test` don’t know how to natively stub the interfaces.

In order to maintain a typical, idiomatic Go development experience, you need a “bridge” between your Go toolchain and a WebAssembly runtime that makes interfaces “just work” when it's time to test your code.

`wadge` (for “Wasm-bridge”) is a framework that bridges the gap, so you can work on your code in a Go-native way. This pattern can be extended beyond testing to extend native Go applications with WebAssembly functions.

## How it works

`wadge` handles interfaces belonging to the [WebAssembly System Interface (WASI) 0.2](https://github.com/WebAssembly/WASI/tree/main/wasip2) automatically, but also allows for testing of custom interfaces defined in WIT.

- `wadge` uses [`cabish`](https://github.com/wasmCloud/cabish) to read and write values over FFI, passing to an embedded WebAssembly runtime ([Wasmtime](https://github.com/bytecodealliance/wasmtime)).
- The `wadge-bindgen-go` utility walks the complete AST of your application and generates a binding file (`bindings.wadge.go`) by default.
- The bindings file utilizes `//go:linkname` directives to replace unimplemented functions with `//go:wasmimport` directives—normally provided by the runtime—using a native implementation. In turn, the native implementation invokes those imports on a WebAssembly component running in an embedded `wadge` instance, operating in a "harness" pattern.

### Testing with a WASI interface

Add `wadge` to your project:

```
go get go.wasmcloud.dev/wadge
```

Include the `wadge` bindgen in your `tools.go` like below:

```go
//go:build tools

package main

import (
	_ "go.bytecodealliance.org/cmd/wit-bindgen-go"
	_ "go.wasmcloud.dev/wadge/cmd/wadge-bindgen-go"
)
```

Write a test for the application in a new file called `<filename>_test.go` (see [example](https://github.com/wasmCloud/wadge/blob/main/examples/go/http/http_test.go))

```
go mod tidy && go mod download
```

Generate `wadge` bindings for your test:

```
go run go.wasmcloud.dev/wadge/cmd/wadge-bindgen-go
```

This generates `bindings.wadge.go`.

Run `go test`:

```
go test
```

```
PASS
ok  	example/http/test	0.296s
```

### Writing a test

Writing tests with `wadge` generally works as you would expect writing ordinary tests in Go:

- Use the standard `testing` package
- Write tests in `<name>_test.go` files
- Target functions with tests using the `TestTargetFunction` naming convention

For more on testing in Go, see the [`testing` package documentation](https://pkg.go.dev/testing).

See the `examples/go/http` directory for examples of a simple HTTP server and associated test using `wasi:http`.

There are a couple of `wadge`-specific details to note:

`RunTest` is written as a method of `wadge`:

```go
func TestIncomingHandler(t *testing.T) {
	wadge.RunTest(t, func() {
		req, err := http.NewRequest("", "test", nil)
		if err != nil {
			t.Fatalf("failed to create new HTTP request: %s", err)
		}
```

`wadgehttp` is used to call the component's export over the `HandleIncomingRequest` interface of [`wasi:http`](https://github.com/WebAssembly/wasi-http):

```go
resp, err := wadgehttp.HandleIncomingRequest(incominghandler.Exports.Handle, req)
```

In the `examples/go/http` test, the line above represents the only manual WASI-specific step: we pass in the component's export and a standard library HTTP request, and we receive a standard library HTTP response.

From this point on, we can test as usual—for example, making standard `assert` statements:

```go
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
```

### Testing a component with custom interfaces

In order to write a test for a component that uses a custom interface defined in WIT, you can call `wadge` in an `init` function of your test file to use a custom instance of the **platform harness**: a component that exports all of the functionality imported by your application.

Typically the platform harness is instantiated automatically and exports standard WASI functionality. When using custom interfaces, you can create a new instance:

```go
instance, err := wadge.NewInstance(&wadge.Config{
		Wasm: component,
	})
	if err != nil {
		log.Fatalf("failed to construct new instance: %s", err)
	}
	wadge.SetInstance(instance)
```

You can see this in practice in the [`wadge/tests/go/wasi` directory](https://github.com/wasmCloud/wadge/tree/main/tests/go/wasi). The component in this directory uses custom `fib` and `leftpad` interfaces (defined in the `/wit/` subfolder) and can be tested using `go test`.

The `init` function in the [test file](https://github.com/wasmCloud/wadge/blob/main/tests/go/wasi/wasi_test.go) for this application is as follows:

```go
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
```

