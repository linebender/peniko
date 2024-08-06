# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

The latest published Peniko release is [0.1.1](#011-2024-05-27) which was released on 2024-05-27.
You can find its changes [documented below](#011-2024-05-27).

## [Unreleased]

This release has an [MSRV] of 1.70.

### Added

- Breaking: An `alpha` multiplier to `Image` ([#40][] by [@DJMcNab][])
- `mint` feature to enable `mint` support in kurbo ([#46][] by [@waywardmonkeys][])

### Changed

- Breaking: Mark `Format` as `#[non_exhaustive]` ([#47][] by [@DJMcNab][])

### Fixed

...

## [0.1.1][] (2024-05-27)

This release has an [MSRV] of 1.70.

### Added

- `serde` feature to enable serde support ([#26] by [@ratmice][])

## [0.1.0][] (2024-02-15)

This release has an [MSRV] of 1.70.

- Initial release

[MSRV]: README.md#minimum-supported-rust-version-msrv

[#26]: https://github.com/linebender/peniko/pull/26
[#40]: https://github.com/linebender/peniko/pull/40
[#46]: https://github.com/linebender/peniko/pull/46
[#47]: https://github.com/linebender/peniko/pull/47

[@DJMcNab]: https://github.com/DJMcNab
[@ratmice]: https://github.com/ratmice
[@waywardmonkeys]: https://github.com/waywardmonkeys

[Unreleased]: https://github.com/linebender/peniko/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/linebender/peniko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/linebender/peniko/releases/tag/v0.1.0
