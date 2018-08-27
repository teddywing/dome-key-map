SOURCE_FILES = $(shell find src -type f -name '*.rs')

LIB := target/debug/libdome_key_map.a

$(LIB): $(SOURCE_FILES)
	cargo build

includer: clean $(LIB)
	gcc -o $@ includer.c $(LIB)

.PHONY: clean
clean:
	rm -f includer
