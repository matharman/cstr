all: release

debug:
	@cargo build

release:
	@cargo build --release

clean:
	@cargo clean

install: release
	install target/release/cstr $(PREFIX)

install-strip: release
	install -s target/release/cstr $(PREFIX)

.PHONY: all debug release clean install install-strip
