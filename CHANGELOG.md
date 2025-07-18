# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

The latest published Peniko release is [0.4.0](#040-2025-04-30) which was released on 2025-04-30.
You can find its changes [documented below](#040-2025-04-30).

## [Unreleased]

This release has an [MSRV] of 1.82.

### Added

- `Style` now impl `PartialEq`. ([#114][] by [@liferooter][])

## [0.4.0][] (2025-04-30)

This release has an [MSRV] of 1.82.

### Added

- `ImageQuality` and `ImageFormat` now optionally impl `bytemuck` traits ([#104][] by [@waywardmonkeys][])

### Changed

- Update to `color` 0.3.0. ([#103][] by [@waywardmonkeys][])

## [0.3.2][] (2025-05-02)

This release has an [MSRV] of 1.82.

This change is a semver trick release, following the pattern documented at <https://github.com/dtolnay/semver-trick>.
`Blob`, `WeakBlob` and `Font` are now the types from Peniko's 0.4.0, which have the same public API as in the 0.3.0 series.
This enables compatibility between Vello 0.5.0 and Parley 0.3.0.

## [0.3.1][] (2025-01-20)

This release has an [MSRV] of 1.82.

### Changed

- Update to `color` 0.2.3. ([#95][] by [@waywardmonkeys][])

## [0.3.0][] (2024-12-18)

This release has an [MSRV] of 1.82.

### Added

- `Gradient`, `Image`, `Brush` now have `with_alpha` and `Gradient` also gets a `multiply_alpha` ([#67][] by [@waywardmonkeys][])
- `Gradient` now tracks a hue direction and interpolation color space ([#71][] by [@waywardmonkeys][])
- `Compose`, `Extend`, `Fill`, and `Mix` now optionally impl `bytemuck` traits ([#72][] by [@waywardmonkeys][])
- Add x/y extend modes and quality hint to images, rename `Format` to `ImageFormat` ([#77][] by [@dfrg][])

### Changed

- `Image` now stores the alpha as an `f32` ([#65][] by [@waywardmonkeys][])
- Use `color` crate. See below for details ([#63][] by [@waywardmonkeys][])
- `ColorStopsSource::collect_stops` now consumes `self` ([#87][] by [@waywardmonkeys][])

### Removed

- Removed the deprecated `Gradient::with_alpha_factor` in favor of `Gradient::multiply_alpha` ([#82][] by [@waywardmonkeys][])

### Color Changes

The old code behind `peniko::Color` has been removed and color functionality is now provided by the [`color`] crate.

This leads to a number of breaking changes:

- `peniko::Color` is now a type alias for `AlphaColor<Srgb>` from the `color` crate.
- `AlphaColor` does not, at this time, impl `Default`, `PartialOrd`, or `Hash`.
- `ColorStop` no longer impls `Default` or `PartialOrd`.
- `Brush`, `BrushRef`, and `ColorStop` can be constructed from a variety of color types, although, for now, `Brush` and `BrushRef` convert this internally into an unclipped `AlphaColor<Srgb>`.
- `ColorStops` is now a newtype wrapper, rather than a type alias for `SmallVec`.
  This allows it to be used with `CacheKey` from Color.
- The `color` crate is re-exported as `peniko::color`, so access to functionality from there is easy.
- The various pre-defined color constants like `peniko::Color::YELLOW` are no longer available.
  Instead, use the CSS palette provided within `color`:  `peniko::color::palette::css::YELLOW`.
- Similarly, parsing a color string is now provided by the `color` crate.

This is the first step towards providing better support for richer color functionality throughout the Linebender stack.

## [0.2.0][] (2024-09-19)

This release has an [MSRV] of 1.70.

### Added

- Breaking: An `alpha` multiplier to `Image` ([#40][] by [@DJMcNab][])
- `mint` feature to enable `mint` support in kurbo ([#46][] by [@waywardmonkeys][])

### Changed

- Breaking: Mark `Format` as `#[non_exhaustive]` ([#47][] by [@DJMcNab][])
- Rename `with_alpha_factor` to `multiply_alpha` ([#52][] by [@DJMcNab][])

## [0.1.1][] (2024-05-27)

This release has an [MSRV] of 1.70.

### Added

- `serde` feature to enable serde support ([#26] by [@ratmice][])

## [0.1.0][] (2024-02-15)

This release has an [MSRV] of 1.70.

- Initial release

[MSRV]: README.md#minimum-supported-rust-version-msrv
[`color`]: https://docs.rs/color/

[#26]: https://github.com/linebender/peniko/pull/26
[#40]: https://github.com/linebender/peniko/pull/40
[#46]: https://github.com/linebender/peniko/pull/46
[#47]: https://github.com/linebender/peniko/pull/47
[#52]: https://github.com/linebender/peniko/pull/52
[#63]: https://github.com/linebender/peniko/pull/63
[#65]: https://github.com/linebender/peniko/pull/65
[#67]: https://github.com/linebender/peniko/pull/67
[#71]: https://github.com/linebender/peniko/pull/71
[#72]: https://github.com/linebender/peniko/pull/72
[#77]: https://github.com/linebender/peniko/pull/77
[#82]: https://github.com/linebender/peniko/pull/82
[#87]: https://github.com/linebender/peniko/pull/87
[#95]: https://github.com/linebender/peniko/pull/95
[#103]: https://github.com/linebender/peniko/pull/103
[#104]: https://github.com/linebender/peniko/pull/104
[#114]: https://github.com/linebender/peniko/pull/114

[@dfrg]: https://github.com/dfrg
[@DJMcNab]: https://github.com/DJMcNab
[@liferooter]: https://github.com/liferooter
[@ratmice]: https://github.com/ratmice
[@waywardmonkeys]: https://github.com/waywardmonkeys

[Unreleased]: https://github.com/linebender/peniko/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/linebender/peniko/compare/v0.3.1...v0.4.0
[0.3.2]: https://github.com/linebender/peniko/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/linebender/peniko/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/linebender/peniko/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/linebender/peniko/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/linebender/peniko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/linebender/peniko/releases/tag/v0.1.0
