module go.wasmcloud.dev/wadge

go 1.24.0

require (
	github.com/stretchr/testify v1.11.1
	go.bytecodealliance.org/cm v0.3.0
	golang.org/x/tools v0.40.0
)

require (
	github.com/coreos/go-semver v0.3.1 // indirect
	github.com/davecgh/go-spew v1.1.1 // indirect
	github.com/docker/libtrust v0.0.0-20160708172513-aabc10ec26b7 // indirect
	github.com/klauspost/compress v1.18.0 // indirect
	github.com/opencontainers/go-digest v1.0.0 // indirect
	github.com/pmezard/go-difflib v1.0.0 // indirect
	github.com/regclient/regclient v0.8.3 // indirect
	github.com/sirupsen/logrus v1.9.3 // indirect
	github.com/tetratelabs/wazero v1.9.0 // indirect
	github.com/ulikunitz/xz v0.5.14 // indirect
	github.com/urfave/cli/v3 v3.3.3 // indirect
	go.bytecodealliance.org v0.6.3-0.20250520224056-999af0bcfafa // indirect
	golang.org/x/mod v0.31.0 // indirect
	golang.org/x/sync v0.19.0 // indirect
	golang.org/x/sys v0.39.0 // indirect
	gopkg.in/yaml.v3 v3.0.1 // indirect
)

replace go.wasmcloud.dev/wadge v0.8.0 => ./.

tool go.bytecodealliance.org/cmd/wit-bindgen-go
