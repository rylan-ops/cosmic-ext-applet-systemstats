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
sudo cp target/release/cosmic-applet-systemstats /usr/bin/
sudo cp res/com.github.rylan-x.systemstats.desktop /usr/share/applications/
```
