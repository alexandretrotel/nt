# ntw

A macOS network toolkit providing real-time speed metrics and simple Wi-Fi interface management.

`ntw` is a fast, minimal CLI utility focused on two things:
- Live network throughput monitoring
- Managing preferred Wi-Fi networks on macOS

It is written in Rust, async-first, and designed to feel native to the macOS command line.

## Features

### Real-time network speed
- Live download / upload monitoring
- Per-interface monitoring
- Configurable units: `bps`, `kbps`, `mbps`, `gbps`
- Automatic default interface detection (macOS)

### Wi-Fi network management (macOS)
- List preferred Wi-Fi networks for an interface
- Remove a specific network by SSID
- Interactive multi-select removal
- Parallel removals for speed
- `--dry-run` mode to preview actions safely

## Platform Support

- **macOS only** (for now)
- Linux / Windows (compile-time error by design)

`ntw` relies on macOS system tools such as `networksetup` and `route`.

## Installation

### From source

```bash
git clone https://github.com/alexandretrotel/ntw.git
cd ntw
cargo install --path .
````

### Using Cargo

```bash
cargo install ntw
# or
cargo binstall ntw
```

## Usage

### Display real-time network speed

```bash
ntw speed
```

By default:

* Interface is auto-detected (falls back to `en0` or `eth0`)
* Unit is `Mbps`

#### Specify interface and unit

```bash
ntw speed --iface en1 --unit gbps
```

Output updates every second (by default):

```
↓     85.42 Mbps  ↑     12.03 Mbps
```

### Manage Wi-Fi networks

#### List preferred networks

```bash
ntw networks list
```

With explicit interface:

```bash
ntw networks list --iface en0
```

#### Remove a specific network

```bash
ntw networks remove --ssid MyWifi
```

#### Interactive removal (multi-select)

```bash
ntw networks remove
```

You will be prompted to select one or more networks to remove.

### Dry-run mode

All network-modifying commands support `--dry-run`:

```bash
ntw networks remove --ssid MyWifi --dry-run
```

Output example:

```
[dry-run] Would remove network 'MyWifi' from interface 'en0'
```

No system changes are made.

## Credits

* Async runtime powered by **Tokio**
* System metrics via **sysinfo**
* CLI parsing via **clap**
* Interactive prompts via **inquire**

## Contributing

Issues and PRs are welcome: [https://github.com/alexandretrotel/ntw/issues](https://github.com/alexandretrotel/ntw/issues)

## License

See [LICENSE](LICENSE) for details.
