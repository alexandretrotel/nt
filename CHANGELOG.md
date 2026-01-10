# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on "Keep a Changelog" and [Semantic Versioning](https://semver.org/).

## v0.2.1

### Changed
- Reworked several internal function signatures to accept string slices where ownership is not required:
  - `app::speed::run` and `platform::macos::speed::realtime_speed` now take `&str` for the interface name instead of `String`. This avoids an allocation / move when the caller already has a `String`.
  - Network removal paths now use `Option<&str>` instead of `&Option<String>`:
    - `app::networks::run_remove` and `app::networks::remove::run` accept `Option<&str>`.
    - `app::dispatch::run` now forwards `ssid.as_deref()` to the `run_remove` call.
  - These changes improve ergonomics and reduce unnecessary cloning while keeping behavior identical.
- Left intentional ownership where required:
  - `platform::macos::networks::remove::remove_networks_parallel` still converts the interface name to an owned `String` for each spawned blocking task (`spawn_blocking`) as those tasks must own their data (`'static`).

### Fixed
- N/A for this release.

### Added
- N/A.

### Removed
- N/A.

### Migration notes
- CLI behavior remains unchanged: clap still produces owned `String`s for command-line arguments, and the top-level command dispatch continues to work the same. No changes needed for typical CLI usage.
- If you directly call the internal functions from other code (e.g., from external crates or tests), update any call sites that passed `&Option<String>` or `String` as appropriate:
  - Replace `&ssid` (where `ssid: Option<String>`) with `ssid.as_deref()`.
  - For speed-related functions, you can pass a `&iface_string` or `&str` instead of moving ownership.
- Be mindful of lifetimes when passing `&str` into async functions: these borrows are fine when you `await` immediately (as in current code). If you later spawn the returned future to run in the background, you will need to provide an owned `String` (or otherwise ensure the borrow outlives the future).

## v0.2.0

### Added
- `auto` unit selection for speed display
  - The CLI now supports `--unit auto` (and `Unit::Auto`) which chooses the most readable unit for the current measured rate (bps / kbps / Mbps / Gbps).
  - Automatic unit selection uses decimal SI thresholds (1_000, 1_000_000, 1_000_000_000).
  - Sensible precision rules applied per unit to keep numbers compact and human-friendly (e.g., more decimals for small Mbps values).
- Improved README documenting `auto` mode, selection rules, and examples.

### Changed
- CLI shape: network management commands moved from a `networks` subcommand group to top-level verbs:
  - `ntw networks list` → `ntw list`
  - `ntw networks remove` → `ntw remove`
  - The top-level commands keep the same flags (`--iface`, `--ssid`, `--dry-run`) but are now easier to discover as top-level verbs.
- Default unit for `ntw speed` changed from `Mbps` to `Auto` (so users get readable units by default).
- Formatting helpers updated to apply unit-specific rules (no decimals for `bps`/`kbps`, adaptive decimals for `Mbps`/`Gbps`).

### Fixed
- N/A for this release.

### Removed
- N/A.

### Migration notes
- Scripts, aliases, CI steps, tooling, or documentation that previously invoked `ntw networks list` or `ntw networks remove` should be updated to the new top-level commands:
  - Replace `ntw networks list` with `ntw list`.
  - Replace `ntw networks remove --ssid <SSID>` with `ntw remove --ssid <SSID>`.
- If you used the explicit `--unit mbps` behavior, nothing changes — explicit unit flags continue to work. If you relied on the CLI default previously being `Mbps`, note that the default is now `Auto`.
- If you want to preserve old invocations for downstream users, consider adding shell aliases or wrapper scripts that translate old calls to the new form.
