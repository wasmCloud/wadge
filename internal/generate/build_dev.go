//go:build dev

package main

import (
	"path/filepath"
)

func TargetPath(target string) string {
	if target == "" {
		return filepath.Join(TargetDir, "debug")
	}
	return filepath.Join(TargetDir, target, "debug")
}

func CargoBuild(target string, args ...string) error {
	base := []string{"build"}
	if target != "" {
		base = append(base, "--target", target)
	}
	return RunCargo(append(base, args...)...)
}
