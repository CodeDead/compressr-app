.PHONY: release
release:
	cargo build --release
	sh .AppImage/createAppImage.sh "$(VERSION)"
