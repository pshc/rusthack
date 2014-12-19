all:
	@cargo build

clean:
	@cargo clean
	@rm -f -- libtcod*.dylib terminal.png

run:
	@cargo run
