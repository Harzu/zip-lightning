install:
	cargo build --release
	cp ./target/release/zip-lightning $(HOME)/bin
	cd $(HOME)/bin; chmod +x zip-lightning