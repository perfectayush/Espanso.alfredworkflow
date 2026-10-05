OUT := Espanso.alfredworkflow

$(OUT): info.plist
	rm -f $@
	zip -j $@ $^

.PHONY: clean
clean:
	rm -f $(OUT)
