PREFIX ?= /usr/local/bin
TARGET ?= debug

.PHONY: all build install uninstall clean
all: build

build:
ifeq (${TARGET}, release)
	cargo build --release
else
	cargo build
endif

install:
	install -d ${DESTDIR}${PREFIX}
	install -m 755 target/${TARGET}/x25519-keygen ${DESTDIR}${PREFIX}/x25519-keygen

uninstall:
	rm ${DESTDIR}${PREFIX}/x25519-keygen

clean:
	cargo clean

help:
	@echo "usage: make install TARGET=release"
	@echo "Default location will be: /usr/local/bin"
	@echo "To change default location use --prefix <new_location>"