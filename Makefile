image:
	cargo clean
	cargo build --release
	rust-objcopy --strip-all -O binary target/aarch64-unknown-none-softfloat/release/kernel firmware/kernel8.img

dbg:
	objdump -D target/aarch64-unknown-none-softfloat/release/rust-os >> debug/objdump.txt
	xxd firmware/kernel8.img >> debug/xxdout.txt

clean:
	cargo clean
	rm debug/objdump.txt
	rm debug/xxdout.txt

pushnew:
	rm -f /volumes/bootfs/kernel8.img
	cp firmware/kernel8.img /volumes/bootfs/kernel8.img
