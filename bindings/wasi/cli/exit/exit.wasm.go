// Code generated by wit-bindgen-go. DO NOT EDIT.

package exit

// This file contains wasmimport and wasmexport declarations for "wasi:cli@0.2.1".

//go:wasmimport wasi:cli/exit@0.2.1 exit
//go:noescape
func wasmimport_Exit(status0 uint32)

//go:wasmimport wasi:cli/exit@0.2.1 exit-with-code
//go:noescape
func wasmimport_ExitWithCode(statusCode0 uint32)