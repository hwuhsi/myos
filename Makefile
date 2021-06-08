build:
	cargo bootimage

dbg: build 
	qemu-system-x86_64 -nographic \
	-m 64 \
	-drive format=raw,file=target/x86_64-myos/debug/bootimage-myos.bin \
	-s -S

# qemu-system-x86_64 -nographic -drive format=raw,file=target\x86_64-myos\debug\bootimage-myos.bin -m 512 -drive id=disk,file=testfs/myimage.img,format=raw,if=none -device ahci,id=ahci -device ide-hd,drive=disk,bus=ahci.0 -s -S