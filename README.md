# System Stats Applet for COSMIC Desktop

A lightweight system monitoring applet for the COSMIC desktop environment.

## Features

**Planned for v1.0:**
- CPU usage monitoring
- Memory usage display
- Network upload/download speeds
- CPU temperature
- GPU temperature

**Current Status:** In development

## Installation

### Building using just

Dependencies:

```bash
sudo apt install just libxkbcommon-dev
```

Install:

```bash
just build-release
sudo just install
```

### Building from source
```bash
cargo build --release
sudo install -Dm755 target/release/cosmic-applet-systemstats /usr/bin/cosmic-applet-systemstats
sudo install -Dm644 res/com.github.rylan-x.systemstats.desktop /usr/share/applications/com.github.rylan-x.systemstats.desktop
```
