SOURCE_FILES = $(shell find src -type f -name '*.rs')

LIB := target/debug/libdome_key_map.a

DKESS_LIB_DEBUG := ~/Library/Developer/Xcode/DerivedData/dome_key_event_source_simulator-*/Build/Products/Debug/libdome_key_event_source_simulator.a
DKESS_LOCAL_LIB_DEBUG := target/debug/deps/libdome_key_event_source_simulator.a

.PHONY: build
build: $(LIB)

$(LIB): $(SOURCE_FILES) $(DKESS_LOCAL_LIB_DEBUG)
	cargo build

includer: clean $(LIB)
	gcc -o $@ includer.c $(LIB)

moder: moder.c $(LIB)
	gcc -g -Wall -Wextra -Werror -o $@ $< $(LIB)

.PHONY: clean
clean:
	rm -f includer moder

$(DKESS_LIB_DEBUG):
	$(MAKE) -C lib/dome_key_event_source_simulator $@

$(DKESS_LOCAL_LIB_DEBUG): $(DKESS_LIB_DEBUG)
	mkdir -p target/debug/deps
	cp -a $< $@
