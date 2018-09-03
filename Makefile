SOURCE_FILES = $(shell find src -type f -name '*.rs')

LIB := target/debug/libdome_key_map.a

$(LIB): $(SOURCE_FILES)
	cargo build

includer: clean $(LIB)
	gcc -o $@ includer.c $(LIB)

moder: moder.c $(LIB)
	gcc -g -Wall -Wextra -Werror -o $@ $< $(LIB)

.PHONY: clean
clean:
	rm -f includer moder
