LLVM_VERSION_SUFFIX ?=
RUST_TOOLCHAIN ?= nightly-2018-12-23
RUST_STD_HASH ?= 680eb5873ae41a92
RUST_LIB_HOME ?= ${HOME}/.rustup/toolchains/${RUST_TOOLCHAIN}-x86_64-unknown-linux-gnu/lib/

pgo-run: pgo
	./build-pgo/pgo-opt
	cargo run --release

.PHONY: pgo
pgo:
	mkdir -p build-pgo
	rustc \
		--out-dir=build-pgo \
		--emit llvm-bc \
		-C opt-level=3 \
		src/main.rs
	opt${LLVM_VERSION_SUFFIX} \
		-O3 \
		-pgo-instr-gen \
		-instrprof \
		build-pgo/main.bc \
		-o build-pgo/pgo.bc
	llc${LLVM_VERSION_SUFFIX} \
		-O3 \
		-relocation-model=pic \
		-filetype=obj \
		build-pgo/pgo.bc
	clang${LLVM_VERSION_SUFFIX} \
		-O3 \
		-Wl,-rpath=${RUST_LIB_HOME} \
		-flto \
		-fPIE \
		-fprofile-instr-generate \
		build-pgo/pgo.o \
		-L${RUST_LIB_HOME} \
		-L/usr/lib/ \
		-lstd-${RUST_STD_HASH} \
		-o build-pgo/pgo
	./build-pgo/pgo
	mv default.profraw build-pgo
	llvm-profdata${LLVM_VERSION_SUFFIX} merge -output=build-pgo/pgo.profdata build-pgo/default.profraw
	opt${LLVM_VERSION_SUFFIX} \
		-O3 \
		-pgo-instr-use \
		-pgo-test-profile-file=build-pgo/pgo.profdata \
		build-pgo/main.bc \
		-o build-pgo/pgo-opt.bc
	llc${LLVM_VERSION_SUFFIX} \
		-O3 \
		-relocation-model=pic \
		-filetype=obj \
		build-pgo/pgo-opt.bc
	clang${LLVM_VERSION_SUFFIX} \
		-O3 \
		-Wl,-rpath=${RUST_LIB_HOME} \
		-flto \
		-fPIE \
		-fprofile-instr-use=build-pgo/pgo.profdata \
		-L${RUST_LIB_HOME} \
		-lstd-${RUST_STD_HASH} \
		build-pgo/pgo-opt.o \
		-o build-pgo/pgo-opt

.PHONY: clean
clean:
	rm -rf build-pgo
