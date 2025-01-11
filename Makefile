build-bootable:
	cargo bootimage

run:
# 	qemu-system-x86_64 -s -S -drive format=raw,file=target/target/debug/bootimage-kernel.bin
	qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-kernel.bin