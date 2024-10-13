all:
	cargo build --release
	mkdir -p out
	cp target/release/templated ./out