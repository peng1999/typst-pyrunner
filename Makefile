BUILD_TYPE ?= debug
TMP_WASM ?= /tmp/typst-pyrunner.wasm

# extract version from toml
VERSION := $(shell grep '^version' pkg/typst.toml | head -n1 | sed -E 's/version\s*=\s*"(.+)"/\1/')
PKG := pyrunner
LOCAL := $(HOME)/.local/share/typst/packages/local
INSTALL_DIR := $(LOCAL)/$(PKG)/$(VERSION)

.PHONY: all install

all: pkg/typst-pyrunner.wasm

pkg/typst-pyrunner.wasm: target/wasm32-wasip1/${BUILD_TYPE}/typst_pyrunner.wasm
	wasi-stub $^ -o $(TMP_WASM) --stub-function wasi_snapshot_preview1:random_get,wasi_snapshot_preview1:environ_get,wasi_snapshot_preview1:environ_sizes_get,wasi_snapshot_preview1:clock_time_get -r 0
	wasi-stub $(TMP_WASM) -o $(TMP_WASM) --stub-function wasi_snapshot_preview1:fd_prestat_get -r 8
	wasi-stub $(TMP_WASM) -o $@ --stub-module wasi_snapshot_preview1

install: all
	@echo "Installing to $(INSTALL_DIR)..."
	rm -rf $(INSTALL_DIR)
	mkdir -p $(INSTALL_DIR)
	cp pkg/* $(INSTALL_DIR)/
