ELF := ../target/release/coroutine_switch2
PERF := sudo perf stat -e dTLB-load-misses,iTLB-load-misses,cs,L1-dcache-load-misses,L1-icache-load-misses,LLC-load-misses,LLC-store-misses,branch-load-misses,branch-misses,cache-misses

build:
	cargo build --release

run: build
	$(ELF)

disasm: build
	objdump -d $(ELF) > disasm.asm

perf: build
	$(PERF) $(ELF)