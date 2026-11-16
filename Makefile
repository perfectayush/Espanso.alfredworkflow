OUT := Espanso.alfredworkflow
BIN := alfred-espanso
TARGETS := aarch64-apple-darwin x86_64-apple-darwin
SRC := Cargo.toml Cargo.lock $(wildcard src/*.rs)

$(OUT): info.plist icon.png $(BIN)
	rm -f $@
	zip -j $@ $^

$(BIN): $(SRC)
	$(foreach t,$(TARGETS),cargo build --release --target $(t);)
	lipo -create -output $@ $(foreach t,$(TARGETS),target/$(t)/release/$(BIN))

.PHONY: test clean
test:
	cargo test

clean:
	cargo clean
	rm -f $(OUT) $(BIN)
