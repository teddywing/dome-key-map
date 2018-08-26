includer: clean
	gcc -o $@ includer.c target/debug/libdome_key_map.a

.PHONY: clean
clean:
	rm -f includer
