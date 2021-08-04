build: 
	cargo build --release
install:
	cp ./target/release/todo-maker /usr/local/bin/
test-editor:
	./target/release/todo-maker -e Typora