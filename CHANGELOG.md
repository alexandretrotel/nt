# Changelog

All notable changes to this project will be documented in this file.

The format is loosely based on "Keep a Changelog" and [Semantic Versioning](https://semver.org/).

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
