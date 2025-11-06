.PHONY: build run install uninstall clean dev release

# Build configuration
BINARY_NAME = hyprland-settings
INSTALL_PREFIX = /usr/local
DESKTOP_FILE = $(BINARY_NAME).desktop

# Build the application in debug mode
build:
	cargo build

# Build the application in release mode
release:
	cargo build --release

# Run the application in development mode
run:
	cargo run

# Development mode with auto-reload (requires cargo-watch)
dev:
	cargo watch -x run

# Install the application
install: release
	@echo "Installing $(BINARY_NAME)..."
	@sudo install -Dm755 target/release/$(BINARY_NAME) $(INSTALL_PREFIX)/bin/$(BINARY_NAME)
	@sudo install -Dm644 assets/$(DESKTOP_FILE) $(INSTALL_PREFIX)/share/applications/$(DESKTOP_FILE)
	@sudo install -Dm644 assets/icon.svg $(INSTALL_PREFIX)/share/icons/hicolor/scalable/apps/$(BINARY_NAME).svg
	@echo "Installation complete!"

# Uninstall the application
uninstall:
	@echo "Uninstalling $(BINARY_NAME)..."
	@sudo rm -f $(INSTALL_PREFIX)/bin/$(BINARY_NAME)
	@sudo rm -f $(INSTALL_PREFIX)/share/applications/$(DESKTOP_FILE)
	@sudo rm -f $(INSTALL_PREFIX)/share/icons/hicolor/scalable/apps/$(BINARY_NAME).svg
	@echo "Uninstallation complete!"

# Clean build artifacts
clean:
	cargo clean

# Check code formatting
fmt:
	cargo fmt --check

# Run clippy linter
lint:
	cargo clippy -- -D warnings

# Run tests
test:
	cargo test

# Create distribution package
dist: release
	@mkdir -p dist
	@cp target/release/$(BINARY_NAME) dist/
	@cp -r assets dist/
	@tar czf $(BINARY_NAME)-$(shell cargo pkgid | cut -d# -f2).tar.gz dist
	@rm -rf dist
	@echo "Distribution package created!"

help:
	@echo "Available targets:"
	@echo "  build    - Build the application in debug mode"
	@echo "  release  - Build the application in release mode"
	@echo "  run      - Run the application"
	@echo "  dev      - Run in development mode with auto-reload"
	@echo "  install  - Install the application system-wide"
	@echo "  uninstall- Remove the application"
	@echo "  clean    - Remove build artifacts"
	@echo "  fmt      - Check code formatting"
	@echo "  lint     - Run clippy linter"
	@echo "  test     - Run tests"
	@echo "  dist     - Create distribution package"