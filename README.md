# System Stats Applet for COSMIC Desktop

A lightweight system monitoring applet for the COSMIC desktop environment.

![System Stats Screenshot](res/screenshots/systemstats.png)

## Features

- CPU usage monitoring
- Memory usage display
- Network upload/download speeds
- CPU temperature
- GPU temperature

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
sudo install -Dm644 res/systemstats-icon.svg /usr/share/icons/hicolor/scalable/apps/systemstats-icon.svg
```

## Configuration

The applet can be configured via `~/.config/systemstats/config.toml`. A default configuration file is automatically created.

### Configuration Options

```toml
# Refresh interval in milliseconds (default: 1000 = 1 second)
refresh_interval_ms = 1000

[monitors]
# Toggle individual monitors on/off (default: all true)
cpu_usage = true
cpu_temperature = true
gpu_temperature = true
memory = true
network = true
```

After editing the config file, restart the applet/panel for changes to take effect.
