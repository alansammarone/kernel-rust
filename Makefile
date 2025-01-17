build-bootable:
	cargo bootimage

run:
	# qemu-system-x86_64 -s -S -drive format=raw,file=target/target/debug/bootimage-kernel.bin
# 	qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-kernel.bin
	qemu-system-x86_64 -drive format=raw,file=target/target/debug/bootimage-kernel.bin -no-reboot -device isa-debug-exit,iobase=0xf4,iosize=0x04 -serial stdio -display none
	



dis-mbr:
	# Copy MBR (first sector, first 512 bytes) to a file and dissasemble it
	# note: only first 440 bytes are executable
	# see https://wiki.osdev.org/MBR_(x86)
	dd if=target/target/debug/bootimage-kernel.bin of=/tmp/mbr count=1 bs=440
	/opt/homebrew/opt/binutils/bin/objdump  -D -b binary -mi386 -Maddr16,data16 /tmp/mbr

	# connect x86 gdb to remote gdbserver 
	#x86_64-elf-gdb -ex "target remote localhost:1234" -ex "set architecture i8086" -ex "set disassembly-flavor intel"

