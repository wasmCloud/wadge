// Code generated by wit-bindgen-go. DO NOT EDIT.

package environment

import (
	"go.bytecodealliance.org/cm"
)

// This file contains wasmimport and wasmexport declarations for "wasi:cli@0.2.1".

//go:wasmimport wasi:cli/environment@0.2.1 get-environment
//go:noescape
func wasmimport_GetEnvironment(result *cm.List[[2]string])

//go:wasmimport wasi:cli/environment@0.2.1 get-arguments
//go:noescape
func wasmimport_GetArguments(result *cm.List[string])

//go:wasmimport wasi:cli/environment@0.2.1 initial-cwd
//go:noescape
func wasmimport_InitialCWD(result *cm.Option[string])