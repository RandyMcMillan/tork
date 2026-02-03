# Makefile for building Tork

# Go-related variables
GITCOMMIT:=$(shell git describe --dirty --always)
BINARY:=./tmp/tork
BUILDOPTS:=-v
GOPATH?=$(HOME)/go
MAKEPWD:=$(dir $(realpath $(firstword $(MAKEFILE_LIST))))
CGO_ENABLED?=0

# OS detection for FFI library extension
UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    LIB_EXT := so
    LD_LIB_PATH_VAR := LD_LIBRARY_PATH
else ifeq ($(UNAME_S),Darwin)
    LIB_EXT := dylib
    LD_LIB_PATH_VAR := DYLD_LIBRARY_PATH
else
    # Default to .so for other Unix-like systems, or fail explicitly for Windows if needed
    LIB_EXT := so
    LD_LIB_PATH_VAR := LD_LIBRARY_PATH
endif

# Main targets
.PHONY: all ffi-build tork clean clean-ffi generate-swagger run-tork-rs run-tork-rs-web

all: tork ffi-build ffi-rust-build

# Build the main Tork Go binary
tork: *.go go.* $(wildcard */**/*.go)
	CGO_ENABLED=$(CGO_ENABLED) go build $(BUILDOPTS) -ldflags="-s -w -X github.com/runabol/tork.GitCommit=$(GITCOMMIT)" -o $(BINARY) cmd/main.go

# FFI Go shared library builds
ffi-go-build: go.mod
	@echo "--- Downloading Go modules ---"
	go mod download
	@echo "--- Building ffi/libtork.$(LIB_EXT) ---"
	CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.$(LIB_EXT) ./ffi/
	@echo "--- Building ffi/libtork_web.$(LIB_EXT) ---"
	CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.$(LIB_EXT) ./ffi/
ifeq ($(UNAME_S),Darwin)
	@echo "--- Patching Go shared libraries with install_name_tool ---"
	install_name_tool -id "@rpath/libtork.dylib" ffi/libtork.dylib
	install_name_tool -id "@rpath/libtork_web.dylib" ffi/libtork_web.dylib
endif

# FFI Rust workspace build
ffi-rust-build: ffi-go-build
	@echo "--- Building Rust workspace ---"
	cargo build
ifeq ($(UNAME_S),Darwin)
	@echo "--- Patching Rust binaries with install_name_tool ---"
	install_name_tool -add_rpath "@loader_path/../ffi" target/debug/tork-rs
	install_name_tool -add_rpath "@loader_path/../ffi" target/debug/tork-rs-web
endif

# Run tork-rs binary
run-tork-rs: ffi-rust-build
	@echo "--- Running tork-rs ---"
	./target/debug/tork-rs

# Run tork-rs-web binary
run-tork-rs-web: ffi-rust-build
	@echo "--- Running tork-rs-web ---"
	./target/debug/tork-rs-web

# Clean targets
clean: clean-go clean-ffi
	rm -f tork

clean-go:
	go clean
	rm -f $(BINARY)

clean-ffi:
	@echo "--- Cleaning FFI build artifacts ---"
	rm -f ffi/libtork.$(LIB_EXT) ffi/libtork_web.$(LIB_EXT)
	cargo clean

# Swagger documentation generation
generate-swagger: docs/swagger.json

docs/swagger.json: *.go go.* $(wildcard */**/*.go)
	# Note: this command comes from https://github.com/swaggo/swag
	swag init  --parseDependency -g internal/coordinator/api/api.go --output docs
	rm docs/docs.go
	rm docs/swagger.yaml
