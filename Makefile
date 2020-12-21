all: release

debug:
	@cargo build

release:
	@cargo build --release

clean:
	@cargo clean

install: release
	install target/release/json2cstr $(PREFIX)

install-strip: release
	install -s target/release/json2cstr $(PREFIX)

.PHONY: all debug release clean install install-strip
