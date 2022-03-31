
RUST_FLAGS := -C target-feature=-mmx,-sse,-sse2,-sse3,-ssse3,-sse4.1,-sse4.2,-3dnow,-3dnowa,-avx,-avx2,+soft-float
TARGET := x86_64-unknown-linux-gnu
MODE := release
CARGO_FLAGS := --target $(TARGET) -Z build-std=core,alloc,std --$(MODE)
ELF := ../target/$(TARGET)/$(MODE)/user_thread_switch

build:
	RUSTFLAGS="$(RUST_FLAGS)" cargo build $(CARGO_FLAGS)

run: build
	$(ELF)

disasm: build
	objdump -d $(ELF) > disasm.asm
