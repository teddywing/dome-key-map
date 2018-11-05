SOURCE_FILES := $(shell find src -type f -name '*.rs')

LIB := target/debug/libdome_key_map.a
LIB_RELEASE := target/release/libdome_key_map.a

DKESS_LIB_DEBUG := ~/Library/Developer/Xcode/DerivedData/dome_key_event_source_simulator-*/Build/Products/Debug/libdome_key_event_source_simulator.a
DKESS_LOCAL_LIB_DEBUG := target/debug/deps/libdome_key_event_source_simulator.a

DKESS_LIB_RELEASE := ~/Library/Developer/Xcode/DerivedData/dome_key_event_source_simulator-*/Build/Products/Release/libdome_key_event_source_simulator.a
DKESS_LOCAL_LIB_RELEASE := target/release/deps/libdome_key_event_source_simulator.a


# Build

.PHONY: build
build: $(LIB)

$(LIB): $(SOURCE_FILES) $(DKESS_LOCAL_LIB_DEBUG)
	cargo build


# Release

.PHONY: release
release: $(LIB_RELEASE)

$(LIB_RELEASE): $(SOURCE_FILES) $(DKESS_LOCAL_LIB_RELEASE)
	cargo build --release


# dome_key_event_source_simulator

$(DKESS_LIB_DEBUG):
	$(MAKE) -C lib/dome_key_event_source_simulator $@

$(DKESS_LOCAL_LIB_DEBUG): $(DKESS_LIB_DEBUG)
	mkdir -p target/debug/deps
	cp -a $< $@

$(DKESS_LIB_RELEASE):
	$(MAKE) -C lib/dome_key_event_source_simulator $@

$(DKESS_LOCAL_LIB_RELEASE): $(DKESS_LIB_RELEASE)
	mkdir -p target/release/deps
	cp -a $< $@
