# Building the Tork Rust FFI Bindings

This document covers building the Go CGo shared libraries and Rust crates that
provide FFI access to Tork's core types and web API types.

## Architecture Overview

```
┌──────────────────────────────────────────────────────────────┐
│  Rust application code                                       │
│                                                              │
│  ┌─────────────────────┐   ┌──────────────────────────────┐  │
│  │  tork-rs             │   │  tork-rs-web                 │  │
│  │  (core types)        │   │  (web/REST API types)        │  │
│  │                      │   │                              │  │
│  │  Job, Task, Node,    │◄──│  JobSummary, ScheduledJob,   │  │
│  │  Metrics, version()  │   │  QueueInfo, HealthCheck,     │  │
│  │                      │   │  TaskLogPart, Page<T>,       │  │
│  └──────────┬───────────┘   │  parse_jobs_page(), ...      │  │
│             │               └──────────────┬───────────────┘  │
│             │ links libtork.so             │ links libtork_web.so
│             │                              │                  │
├─────────────┼──────────────────────────────┼──────────────────┤
│  C ABI      │                              │                  │
│  ┌──────────▼───────────┐   ┌──────────────▼───────────────┐  │
│  │  ffi/libtork.so      │   │  ffi/libtork_web.so          │  │
│  │  ffi/libtork.h       │   │  ffi/libtork_web.h           │  │
│  │  31 exported symbols │   │  39 exported symbols         │  │
│  └──────────┬───────────┘   └──────────────┬───────────────┘  │
│             │  CGo -buildmode=c-shared     │                  │
│  ┌──────────▼──────────────────────────────▼───────────────┐  │
│  │  ffi/ffi.go          ffi/ffi_web.go                     │  │
│  │  Go source with //export annotations                    │  │
│  └──────────┬──────────────────────────────────────────────┘  │
│             │                                                 │
│  ┌──────────▼──────────────────────────────────────────────┐  │
│  │  Tork Go packages                                       │  │
│  │  github.com/runabol/tork (Job, Task, Node, Metrics)     │  │
│  │  github.com/runabol/tork/broker (QueueInfo)             │  │
│  │  github.com/runabol/tork/health (HealthCheckResult)     │  │
│  │  github.com/runabol/tork/datastore (Page)               │  │
│  └─────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

The build happens in two phases:

1. **Go phase** — CGo compiles `ffi/*.go` into shared libraries (`.so` / `.dylib`
   / `.dll`) and generates C headers.
2. **Rust phase** — `bindgen` reads the C headers and generates raw FFI
   bindings; the Rust crates wrap these in safe, idiomatic APIs.


## Prerequisites

### All Platforms

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| **Go** | 1.23+ (toolchain 1.24.2) | Build shared libraries via CGo |
| **Rust** | 1.70+ (edition 2021) | Build the Rust crates |
| **Cargo** | (ships with Rust) | Rust package manager |
| **libclang** | 6.0+ | Required by `bindgen` for parsing C headers |
| **C compiler** | gcc or clang | Required by CGo and bindgen |
| **Git** | 2.x | Cloning the repository |

### Linux (Debian / Ubuntu)

```bash
# System packages
sudo apt-get update
sudo apt-get install -y build-essential libclang-dev pkg-config

# Go (if not already installed)
GO_VERSION=1.24.2
wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
export PATH="/usr/local/go/bin:$PATH"

# Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

For ARM64 systems, replace `amd64` with `arm64` in the Go download URL.

### Linux (Fedora / RHEL / CentOS)

```bash
sudo dnf install -y gcc clang-devel pkg-config

# Go
GO_VERSION=1.24.2
wget https://go.dev/dl/go${GO_VERSION}.linux-amd64.tar.gz
sudo tar -C /usr/local -xzf go${GO_VERSION}.linux-amd64.tar.gz
export PATH="/usr/local/go/bin:$PATH"

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

### Linux (Arch)

```bash
sudo pacman -S base-devel clang go rust
```

### macOS

```bash
# Xcode command-line tools (provides clang)
xcode-select --install

# Homebrew packages
brew install go llvm

# Add LLVM to environment for bindgen
export LLVM_CONFIG_PATH="$(brew --prefix llvm)/bin/llvm-config"
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
```

Both Intel (amd64) and Apple Silicon (arm64) are supported. CGo will
produce `libtork.dylib` / `libtork_web.dylib` instead of `.so` on macOS.

### Windows

Building on Windows requires either **MSYS2/MinGW** or **WSL2**.

#### Option A: WSL2 (Recommended)

Use WSL2 with any Linux distribution and follow the Linux instructions above.
This is the most reliable path.

#### Option B: MSYS2 / MinGW-w64

```powershell
# Install MSYS2 from https://www.msys2.org
# Open an MSYS2 MinGW64 shell, then:

pacman -S mingw-w64-x86_64-toolchain mingw-w64-x86_64-clang mingw-w64-x86_64-go
```

Install Rust from https://rustup.rs using the `x86_64-pc-windows-gnu` target.

CGo will produce `tork.dll` / `tork_web.dll` on Windows.

#### Option C: Visual Studio + Pre-built LLVM

```powershell
# Install Visual Studio Build Tools with C++ workload
# Install LLVM from https://github.com/llvm/llvm-project/releases
# Install Go from https://go.dev/dl/
# Install Rust from https://rustup.rs (use x86_64-pc-windows-msvc target)

# Set environment variables
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\lib"
```

### Verify Prerequisites

```bash
go version          # go1.24.2 or later
rustc --version     # 1.70.0 or later
cargo --version     # should match rustc
cc --version        # gcc or clang
llvm-config --version  # 6.0+ (or check that libclang is findable)
```


## Quick Start (Full Build)

From the repository root:

```bash
# 1. Download Go module dependencies
go mod download

# 2. Build the core shared library
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/

# 3. Build the web shared library (includes core symbols)
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/

# 4. Build and test tork-rs (core types)
cd tork-rs && cargo test && cd ..

# 5. Build and test tork-rs-web (web API types)
cd tork-rs-web && cargo test && cd ..
```

On macOS, change `.so` to `.dylib`.  On Windows (MinGW), change to `.dll`.

Both shared libraries are built from the same `./ffi/` package — the web library
is a strict superset that contains all core symbols plus the web API symbols.


## Step-by-Step Build

### Phase 1: Go Shared Libraries

#### 1a. Core Library (`libtork.so`)

```bash
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
```

Produces:
- `ffi/libtork.so` — shared library (≈3 MB)
- `ffi/libtork.h` — generated C header

Exports 31 symbols covering: `Job`, `Task`, `Node`, `Metrics` types; JSON
serialization/deserialization; memory management; job and task state constants;
`version()` / `git_commit()`.

#### 1b. Web Library (`libtork_web.so`)

```bash
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/
```

Produces:
- `ffi/libtork_web.so` — shared library (≈6 MB)
- `ffi/libtork_web.h` — generated C header

Exports all 31 core symbols plus 39 web-specific symbols covering:
`JobSummary`, `ScheduledJob`, `ScheduledJobSummary`, `QueueInfo`,
`HealthCheckResult`, `TaskLogPart`, `Page`; typed page parsers; queue
classification helpers; scheduled-job/health/queue constants.

#### Go Build Options

```bash
# With version stamp (recommended for release builds)
GITCOMMIT=$(git describe --dirty --always)
CGO_ENABLED=1 go build -buildmode=c-shared \
  -ldflags="-X github.com/runabol/tork.GitCommit=$GITCOMMIT" \
  -o ffi/libtork_web.so ./ffi/

# macOS (produces .dylib + .h)
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.dylib ./ffi/
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.dylib ./ffi/

# Windows MinGW (produces .dll + .h)
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/tork.dll ./ffi/
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/tork_web.dll ./ffi/

# Cross-compile for Linux ARM64 from an AMD64 host
CGO_ENABLED=1 GOOS=linux GOARCH=arm64 CC=aarch64-linux-gnu-gcc \
  go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
```

### Phase 2: Rust Crates

#### 2a. tork-rs (Core Types)

```bash
cd tork-rs
cargo build
cargo test
```

The `build.rs` script:
1. Finds `ffi/libtork.h` relative to the crate's `Cargo.toml`.
2. Invokes `bindgen` to generate raw Rust FFI declarations.
3. Links `libtork.so` and sets `rpath` for runtime discovery.

#### 2b. tork-rs-web (Web API Types)

```bash
cd tork-rs-web
cargo build
cargo test
```

Same mechanism but links `libtork_web.so`. Depends on `tork-rs` via a path
dependency and re-exports its core types.

#### Rust Build Options

```bash
# Release build
cargo build --release

# Run a specific test
cargo test test_job_summary_roundtrip

# See test output
cargo test -- --nocapture

# Check without building (faster CI lint step)
cargo check
```


## macOS-Specific Notes

### Library Extension

CGo produces `.dylib` files on macOS. The `build.rs` scripts look for `.so` by
default. To build on macOS, either:

**Option A** — Rename after build:
```bash
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.dylib ./ffi/
cp ffi/libtork.dylib ffi/libtork.so

CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.dylib ./ffi/
cp ffi/libtork_web.dylib ffi/libtork_web.so
```

**Option B** — Build with `.so` extension directly (Go accepts this on macOS):
```bash
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/
```

### `rpath` and `install_name`

On macOS, `cargo test` might fail with a "library not found" error. Fix by
setting `DYLD_LIBRARY_PATH`:

```bash
export DYLD_LIBRARY_PATH="$(pwd)/ffi:$DYLD_LIBRARY_PATH"
cd tork-rs && cargo test && cd ..
cd tork-rs-web && cargo test && cd ..
```

Or use `install_name_tool` to patch the library:
```bash
install_name_tool -id "@rpath/libtork.dylib" ffi/libtork.dylib
install_name_tool -id "@rpath/libtork_web.dylib" ffi/libtork_web.dylib
```

### Apple Silicon / Intel

No special flags are needed — Go and Rust will build for the host architecture
by default. Cross-compiling between arm64 and amd64 on macOS requires the
appropriate Go cross-compiler and a matching Rust target:

```bash
# From arm64 Mac, build for amd64
CGO_ENABLED=1 GOARCH=amd64 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
rustup target add x86_64-apple-darwin
cd tork-rs && cargo build --target x86_64-apple-darwin
```


## Windows-Specific Notes

### Library Names

On Windows, the shared libraries are `.dll` files and the link libraries are
`.lib` or `.a` files depending on toolchain.

```powershell
# MinGW
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/tork.dll ./ffi/

# The Rust build.rs needs adjustment for Windows library names.
# Set the environment variable to help the linker find the DLL:
$env:PATH = "$PWD\ffi;$env:PATH"
```

### Adjusting `build.rs` for Windows

The `build.rs` files use `-Wl,-rpath` which is a Unix linker flag. On Windows,
this flag is ignored. Instead, ensure the `.dll` files are either:

1. In the same directory as the test executable, or
2. In a directory listed in `PATH`.

The simplest approach: add the `ffi/` directory to `PATH` before running
`cargo test`.

### MSVC vs MinGW

- **MinGW** — CGo works out of the box. Use `x86_64-pc-windows-gnu` Rust target.
- **MSVC** — Requires more setup. Use `x86_64-pc-windows-msvc` Rust target and
  ensure `LIBCLANG_PATH` points to a valid LLVM installation.


## Environment Variables Reference

| Variable | When Needed | Example |
|----------|-------------|---------|
| `CGO_ENABLED` | Always (Go build) | `CGO_ENABLED=1` |
| `LIBCLANG_PATH` | bindgen can't find libclang | `/usr/lib/llvm-14/lib` |
| `LLVM_CONFIG_PATH` | macOS with Homebrew LLVM | `$(brew --prefix llvm)/bin/llvm-config` |
| `DYLD_LIBRARY_PATH` | macOS runtime can't find .dylib | `$(pwd)/ffi` |
| `LD_LIBRARY_PATH` | Linux runtime can't find .so | `$(pwd)/ffi` |
| `PATH` | Windows runtime can't find .dll | `$PWD\ffi;$env:PATH` |
| `CC` | Cross-compiling with CGo | `aarch64-linux-gnu-gcc` |
| `GOOS` / `GOARCH` | Cross-compiling Go | `linux` / `arm64` |
| `BINDGEN_EXTRA_CLANG_ARGS` | Custom clang flags for bindgen | `--sysroot=/path/to/sysroot` |


## Using the Crates in Your Project

### tork-rs Only (Core Types)

```toml
# Cargo.toml
[dependencies]
tork-rs = { path = "path/to/tork/tork-rs" }
```

```rust
use tork_rs::{Job, Task, Node, Metrics, job_state, task_state};

fn main() {
    println!("tork {}", tork_rs::version());

    let job = Job::from_json(r#"{"id":"j1","name":"build","state":"RUNNING"}"#)
        .expect("valid json");
    println!("{}: {}", job.name, job.state);

    let task = Task::from_json(r#"{"id":"t1","state":"RUNNING","image":"alpine"}"#)
        .unwrap();
    assert!(task.is_active());
    assert_eq!(task.state, task_state::RUNNING);
}
```

### tork-rs-web (Web API Types)

```toml
# Cargo.toml
[dependencies]
tork-rs-web = { path = "path/to/tork/tork-rs-web" }
# tork-rs is re-exported — no need to add it separately
```

```rust
use tork_rs_web::{
    // Re-exported core types
    Job, Task, Node, Metrics,
    // Web API types
    JobSummary, ScheduledJob, ScheduledJobSummary,
    QueueInfo, HealthCheck, TaskLogPart, Page, RawPage,
    // Page parsers
    parse_jobs_page, parse_scheduled_jobs_page, parse_log_page,
    // Queue helpers
    is_coordinator_queue, is_worker_queue, is_task_queue,
    // Constants
    scheduled_job_state, health_status, queue,
};

fn process_api_response(body: &str) {
    // Parse a GET /jobs response
    let page = parse_jobs_page(body).expect("valid page");
    for job in &page.items {
        println!("{}: {} ({})", job.id, job.name, job.state);
    }
    println!("page {}/{}", page.number, page.total_pages);

    // Parse a GET /health response
    let health = HealthCheck::from_json(r#"{"status":"UP","version":"0.1.151"}"#)
        .unwrap();
    assert!(health.is_up());

    // Queue classification
    assert!(is_coordinator_queue(queue::PENDING));
    assert!(is_task_queue(queue::DEFAULT));
}
```


## Running Tests

```bash
# All tests for both crates
cd tork-rs     && cargo test && cd ..
cd tork-rs-web && cargo test && cd ..

# Verbose output
cd tork-rs     && cargo test -- --nocapture && cd ..
cd tork-rs-web && cargo test -- --nocapture && cd ..
```

Expected results:

| Crate | Tests | What They Cover |
|-------|-------|-----------------|
| `tork-rs` | 7 | Job roundtrip, Task parsing, Task.is_active(), Node parsing, Metrics parsing, version(), invalid JSON |
| `tork-rs-web` | 14 | JobSummary roundtrip/timestamps, ScheduledJob full/summary, QueueInfo roundtrip, HealthCheck up/down, TaskLogPart, RawPage, parse_jobs_page, parse_scheduled_jobs_page, parse_log_page, queue classification, invalid JSON, constants |

If tests fail with a "library not found" error, see the platform-specific
sections above for `LD_LIBRARY_PATH` / `DYLD_LIBRARY_PATH` / `PATH` fixes.


## Troubleshooting

### `go build` fails: "cgo: C compiler not found"

Install a C compiler:
```bash
# Debian/Ubuntu
sudo apt-get install build-essential
# macOS
xcode-select --install
# Fedora
sudo dnf install gcc
```

### `go build` fails: module download errors

```bash
go mod download
# If behind a proxy:
GOPROXY=https://proxy.golang.org,direct go mod download
```

### `cargo build` fails: "Unable to generate bindings" / "libclang not found"

bindgen requires `libclang`. Install it and, if needed, point bindgen to it:

```bash
# Debian/Ubuntu
sudo apt-get install libclang-dev

# Fedora
sudo dnf install clang-devel

# macOS (Homebrew)
brew install llvm
export LIBCLANG_PATH="$(brew --prefix llvm)/lib"

# Manually specify (any platform)
export LIBCLANG_PATH=/usr/lib/llvm-14/lib  # adjust path
```

### `cargo test` fails: "cannot open shared object file" / "library not loaded"

The shared library can't be found at runtime. The `build.rs` sets `rpath` but
this only works when the relative path hasn't changed.

```bash
# Linux
export LD_LIBRARY_PATH="$(pwd)/ffi:$LD_LIBRARY_PATH"

# macOS
export DYLD_LIBRARY_PATH="$(pwd)/ffi:$DYLD_LIBRARY_PATH"

# Windows (PowerShell)
$env:PATH = "$PWD\ffi;$env:PATH"
```

### `cargo test` fails: "undefined symbol: tork_web_*"

You built `libtork.so` but not `libtork_web.so`, or the web library is stale.
Rebuild:

```bash
CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/
```

### bindgen generates empty or incomplete bindings

Check that the header file exists and is readable:

```bash
ls -la ffi/libtork.h ffi/libtork_web.h
```

If headers are missing, the Go build step didn't produce them — re-run the
`go build -buildmode=c-shared` commands.

### Cross-compilation: "cannot find -ltork"

When cross-compiling Rust for a different target, you need a shared library
built for that target too. Build the Go shared library for the target
architecture first:

```bash
CGO_ENABLED=1 GOOS=linux GOARCH=arm64 CC=aarch64-linux-gnu-gcc \
  go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
```


## CI/CD Examples

### GitHub Actions

```yaml
name: FFI Build & Test
on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: '1.24'

      - uses: dtolnay/rust-toolchain@stable

      - name: Install system dependencies
        run: sudo apt-get update && sudo apt-get install -y libclang-dev

      - name: Download Go modules
        run: go mod download

      - name: Build Go shared libraries
        run: |
          CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
          CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/

      - name: Test tork-rs
        working-directory: tork-rs
        run: cargo test

      - name: Test tork-rs-web
        working-directory: tork-rs-web
        run: cargo test

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v4

      - uses: actions/setup-go@v5
        with:
          go-version: '1.24'

      - uses: dtolnay/rust-toolchain@stable

      - name: Install LLVM
        run: brew install llvm

      - name: Download Go modules
        run: go mod download

      - name: Build Go shared libraries
        run: |
          CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
          CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/

      - name: Test Rust crates
        env:
          LIBCLANG_PATH: /opt/homebrew/opt/llvm/lib
          DYLD_LIBRARY_PATH: ${{ github.workspace }}/ffi
        run: |
          cd tork-rs && cargo test && cd ..
          cd tork-rs-web && cargo test && cd ..
```

### GitLab CI

```yaml
ffi-build:
  image: golang:1.24
  before_script:
    - apt-get update && apt-get install -y libclang-dev curl build-essential
    - curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    - source "$HOME/.cargo/env"
  script:
    - go mod download
    - CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
    - CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/
    - cd tork-rs && cargo test && cd ..
    - cd tork-rs-web && cargo test && cd ..
```

### Dockerfile

```dockerfile
FROM golang:1.24 AS go-builder
WORKDIR /src
COPY go.mod go.sum ./
RUN go mod download
COPY . .
RUN CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork.so ./ffi/
RUN CGO_ENABLED=1 go build -buildmode=c-shared -o ffi/libtork_web.so ./ffi/

FROM rust:1.83 AS rust-builder
RUN apt-get update && apt-get install -y libclang-dev
WORKDIR /src
COPY --from=go-builder /src .
RUN cd tork-rs && cargo build --release
RUN cd tork-rs-web && cargo build --release
RUN cd tork-rs && cargo test
RUN cd tork-rs-web && cargo test
```


## File Reference

```
tork/
├── ffi/
│   ├── ffi.go              # CGo exports: core types (31 symbols)
│   ├── ffi_web.go          # CGo exports: web API types (39 symbols)
│   ├── libtork.h           # Generated C header (core)
│   ├── libtork.so          # Built shared library (core) — gitignored
│   ├── libtork_web.h       # Generated C header (core + web)
│   └── libtork_web.so      # Built shared library (core + web) — gitignored
├── tork-rs/
│   ├── Cargo.toml          # Core crate manifest
│   ├── Cargo.lock
│   ├── build.rs            # bindgen + linker config for libtork.so
│   └── src/
│       └── lib.rs          # Safe Rust API: Job, Task, Node, Metrics, ...
├── tork-rs-web/
│   ├── Cargo.toml          # Web crate manifest (depends on tork-rs)
│   ├── Cargo.lock
│   ├── build.rs            # bindgen + linker config for libtork_web.so
│   └── src/
│       └── lib.rs          # Safe Rust API: JobSummary, QueueInfo, Page, ...
└── FFI_BUILD.md            # This file
```

### What Gets Committed vs. Gitignored

**Committed** (source of truth):
- `ffi/ffi.go`, `ffi/ffi_web.go` — Go CGo source
- `ffi/libtork.h`, `ffi/libtork_web.h` — generated headers (committed for convenience)
- `tork-rs/`, `tork-rs-web/` — full Rust source including `Cargo.lock`

**Gitignored** (build artifacts):
- `ffi/libtork.so`, `ffi/libtork_web.so` — shared libraries
- `tork-rs/target/`, `tork-rs-web/target/` — Rust build output


## API Quick Reference

### tork-rs Exports

| Type / Function | Description |
|-----------------|-------------|
| `Job` | Core job type with `from_json()`, `to_json()` |
| `Task` | Core task type with `from_json()`, `is_active()` |
| `Node` | Worker node with `from_json()` |
| `Metrics` | Cluster metrics with `from_json()` |
| `version()` | Returns the tork version string |
| `git_commit()` | Returns the build git commit hash |
| `job_state::*` | Constants: `PENDING`, `SCHEDULED`, `RUNNING`, `CANCELLED`, `COMPLETED`, `FAILED`, `RESTART` |
| `task_state::*` | Constants: `CREATED`, `PENDING`, `SCHEDULED`, `RUNNING`, `CANCELLED`, `STOPPED`, `COMPLETED`, `FAILED`, `SKIPPED` |

### tork-rs-web Exports

| Type / Function | REST Endpoint |
|-----------------|---------------|
| `JobSummary` | `GET /jobs`, `POST /jobs` |
| `ScheduledJob` | `GET /scheduled-jobs/:id` |
| `ScheduledJobSummary` | `GET /scheduled-jobs` |
| `QueueInfo` | `GET /queues`, `GET /queues/:name` |
| `HealthCheck` | `GET /health` |
| `TaskLogPart` | `GET /tasks/:id/log`, `GET /jobs/:id/log` |
| `Page<T>` / `RawPage` | Paginated response wrapper |
| `parse_jobs_page()` | Typed parser for job list pages |
| `parse_scheduled_jobs_page()` | Typed parser for scheduled job pages |
| `parse_log_page()` | Typed parser for log pages |
| `is_coordinator_queue()` | Queue classification helper |
| `is_worker_queue()` | Queue classification helper |
| `is_task_queue()` | Queue classification helper |
| `scheduled_job_state::*` | `ACTIVE`, `PAUSED` |
| `health_status::*` | `UP`, `DOWN` |
| `queue::*` | `PENDING`, `STARTED`, `COMPLETED`, `ERROR`, `DEFAULT`, `HEARTBEAT`, `JOBS`, `LOGS`, `PROGRESS`, `REDELIVERIES`, `EXCLUSIVE_PREFIX` |

Plus all `tork-rs` types re-exported via `pub use tork_rs::*`.
