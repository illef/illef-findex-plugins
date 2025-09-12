build:
	cargo build --release

install: build
	mkdir -p ~/.cache/illef-findex-plugin
	mkdir -p ~/.cache/illef-findex-plugin/favicons/
	ln -sf $(CURDIR)/assets/translate.png ~/.cache/illef-findex-plugin/favicons/translate.png
	ln -sf $(CURDIR)/target/release/libfindex_raindrop.so ~/.cache/illef-findex-plugin/libfindex_raindrop.so
	ln -sf $(CURDIR)/target/release/libfindex_translator.so ~/.cache/illef-findex-plugin/libfindex_translator.so
	ln -sf $(CURDIR)/target/release/libzotero.so ~/.cache/illef-findex-plugin/libzotero.so
	ln -sf $(CURDIR)/target/release/libunicode_picker.so ~/.cache/illef-findex-plugin/libunicode_picker.so
	ln -sf $(CURDIR)/target/release/liblogseq.so ~/.cache/illef-findex-plugin/liblogseq.so
	ln -sf $(CURDIR)/scripts ~/.cache/illef-findex-plugin/
	ln -sf $(CURDIR)/assets/zotero-icons ~/.cache/illef-findex-plugin/
