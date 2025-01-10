build-bootable:
	cargo bootimage

run:
	qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-kernel.bin