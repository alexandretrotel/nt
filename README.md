# ntw

A macOS network toolkit providing real-time speed metrics and simple Wi‑Fi interface management.

`ntw` is a fast, minimal CLI utility focused on two things:
- Live network throughput monitoring
- Managing preferred Wi‑Fi networks on macOS

It is written in Rust, async-first, and designed to feel native to the macOS command line.

## Features

### Real-time network speed
- Live download / upload monitoring
- Per-interface monitoring
- Configurable units: `auto`, `bps`, `kbps`, `mbps`, `gbps`
- Automatic unit selection (`auto`) for readable displays
- Automatic default interface detection (macOS)

### Wi‑Fi network management (macOS)
- List preferred Wi‑Fi networks for an interface
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
```

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

- Interface is auto-detected (falls back to `en0` or `eth0`)
- Unit is `auto` (automatically selects bps/kbps/Mbps/Gbps for readable output)
- Output updates every second (by default)

You can change the update interval with the `--delay` option:

```bash
ntw speed --delay 500
```

The displayed throughput is normalized to "per second" regardless of the actual interval.

#### Specify interface and unit

```bash
ntw speed --iface en1 --unit mbps
```

If you prefer automatic, explicit `auto` shows the best unit for the current value:

```bash
ntw speed --unit auto
```

Example output:

```
↓     85.42 Mbps  ↑     12.03 Mbps
```

#### How `auto` chooses units

When `--unit auto` (or the default) is used, `ntw` selects the most appropriate unit based on the measured bits/second using SI (decimal) thresholds:

- < 1,000 bits/sec → `bps` (e.g., `512 bps`)
- ≥ 1,000 and < 1,000,000 → `kbps` (e.g., `12,345 Kbps`)
- ≥ 1,000,000 and < 1,000,000,000 → `Mbps` (e.g., `85.42 Mbps`)
- ≥ 1,000,000,000 → `Gbps` (e.g., `1.23 Gbps`)

Formatting rules are chosen to balance precision and readability (for example, small Mbps values show more decimals while large values are compact). You can override `auto` by passing any of `--unit bps|kbps|mbps|gbps`.

> Note: Networking commonly uses decimal SI prefixes: 1 kbps = 1,000 bps; 1 Mbps = 1,000,000 bps.

### Manage Wi‑Fi networks

#### List preferred networks

```bash
ntw list
```

With explicit interface:

```bash
ntw list --iface en0
```

#### Remove a specific network

```bash
ntw remove --ssid MyWifi
```

#### Interactive removal (multi-select)

```bash
ntw remove
```

You will be prompted to select one or more networks to remove.

### Dry-run mode

All network-modifying commands support `--dry-run`:

```bash
ntw remove --ssid MyWifi --dry-run
```

Output example:

```
[dry-run] Would remove network 'MyWifi' from interface 'en0'
```

No system changes are made.

## Changelog

See `CHANGELOG.md` for details about recent changes (including the addition of `auto` unit selection and top-level `list`/`remove` commands).

## Credits

- Async runtime powered by **Tokio**
- System metrics via **sysinfo**
- CLI parsing via **clap**
- Interactive prompts via **inquire**

## Contributing

Issues and PRs are welcome: https://github.com/alexandretrotel/ntw/issues

## License

See `LICENSE` for details.
