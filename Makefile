OUT := Espanso.alfredworkflow

$(OUT): info.plist icon.png
	rm -f $@
	zip -j $@ $^

.PHONY: clean
clean:
	rm -f $(OUT)
