package main

import (
	"flag"
	"log"
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

var (
	Root = func() string {
		_, f, _, _ := runtime.Caller(0)
		return filepath.Join(f, "..", "..", "..")
	}()

	TargetDir = filepath.Join(Root, "target")
	LibDir    = filepath.Join(Root, "lib")
)

func init() {
	log.SetFlags(0)
	flag.Parse()
}

func Run(cmd *exec.Cmd) error {
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	return cmd.Run()
}

func RunCargo(args ...string) error {
	return Run(exec.Command("cargo", args...))
}

func main() {
	if err := RunCargo("run", "-p", "generate-header"); err != nil {
		log.Fatalf("failed to generate `wadge-sys` C header: %s", err)
	}
	if err := Run(exec.Command(
		"go",
		"run",
		"github.com/bytecodealliance/wasm-tools-go/cmd/wit-bindgen-go",
		"generate",
		"-w",
		"imports",
		"-o",
		"bindings",
		filepath.Join(Root, "wit"),
	)); err != nil {
		log.Fatalf("failed to generate WIT bindings: %s", err)
	}
	if err := CargoBuild(CARGO_TARGET, "-p", "wadge-sys"); err != nil {
		log.Fatalf("failed to build FFI: %s", err)
	}
	libTarget := filepath.Join(LibTargetDir, "libwadge.a")
	if err := os.RemoveAll(libTarget); err != nil {
		log.Fatalf("failed to remove FFI: %s", err)
	}
	if err := os.Link(
		filepath.Join(TargetPath(CARGO_TARGET), "libwadge_sys.a"),
		libTarget,
	); err != nil {
		log.Fatalf("failed to hard link FFI: %s", err)
	}
	if err := CargoBuild("wasm32-unknown-unknown", "-p", "wadge-passthrough"); err != nil {
		log.Fatalf("failed to build passthrough component: %s", err)
	}
	if err := Run(exec.Command(
		"wasm-tools",
		"component",
		"new",
		filepath.Join(TargetPath("wasm32-unknown-unknown"), "wadge_passthrough.wasm"),
		"-o",
		filepath.Join(LibDir, "passthrough.wasm"),
	)); err != nil {
		log.Fatalf("failed to create passthrough component: %s", err)
	}
}
