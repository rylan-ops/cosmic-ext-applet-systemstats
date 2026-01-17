# Build the release version
build-release:
    cargo build --release

# Install the applet (requires sudo, run 'just build-release' first)
install:
    install -Dm755 target/release/cosmic-ext-applet-systemstats /usr/bin/cosmic-ext-applet-systemstats
    install -Dm644 res/io.github.rylan_x.cosmic-ext-applet-systemstats.desktop /usr/share/applications/io.github.rylan_x.cosmic-ext-applet-systemstats.desktop
    install -Dm644 res/io.github.rylan_x.cosmic-ext-applet-systemstats.svg /usr/share/icons/hicolor/scalable/apps/io.github.rylan_x.cosmic-ext-applet-systemstats.svg

# Uninstall the applet (requires sudo)
uninstall:
    rm -f /usr/bin/cosmic-ext-applet-systemstats
    rm -f /usr/share/applications/io.github.rylan_x.cosmic-ext-applet-systemstats.desktop
    rm -f /usr/share/icons/hicolor/scalable/apps/io.github.rylan_x.cosmic-ext-applet-systemstats.svg

# Clean build artifacts
clean:
    cargo clean
