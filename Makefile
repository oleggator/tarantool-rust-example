all: build install

build:
	cargo build --release

install:
	cp target/release/libprocedures.so tarantool \
		|| cp target/release/libprocedures.dylib tarantool
