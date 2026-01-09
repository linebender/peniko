# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

The latest published Peniko release is [0.6.0](#060-2026-01-09) which was released on 2026-01-09.
You can find its changes [documented below](#060-2026-01-09).

## [Unreleased]

This release has an [MSRV] of 1.85.

## [0.6.0][] (2026-01-09)

This release has an [MSRV] of 1.85.

### Added

- Derive `Eq` and `Hash` on `InterpolationAlphaSpace`. ([#148][] by [@sagudev][])
- There is now a `From` conversion from `&ImageData` to `ImageBrushRef`. ([#147][] by [@DJMcNab][])

### Changed

- Breaking change: Update to [Kurbo v0.13.0](https://github.com/linebender/kurbo/releases/tag/v0.13.0). ([#155][] by [@waywardmonkeys][])

### Removed

- Breaking change: `Mix::Clip` has been removed; it was previously deprecated in v0.5.0. ([#124][] by [@DJMcNab][])

## [0.5.0][] (2025-10-01)

This release has an [MSRV] of 1.82.

### Added

- `Style` now implements `PartialEq`. ([#114][] by [@liferooter][])
- Add `Bgra8` variant to `ImageFormat`. ([#120][] by [@sagudev][])
- Provide `ImageAlphaType` with `ImageData`. ([#121][] by [@sagudev][])
- Breaking change: Add `InterpolationAlphaSpace` to `Gradient` to chose how color channels should be handled when interpolating between transparent colors. ([#115][] by [@sagudev][])

### Changed

- Breaking change: Each `GradientKind` now contains a corresponding internal struct, from struct variants.
  `GradientKind::Linear { ... }` is now written as `LinearGradientPosition { ... }.into()` (or, more explicitly, as `GradientKind::Linear(LinearGradientPosition { ... })`).
  The equivalent transform applies for `GradientKind::Sweep { ... }` and `GradientKind::Radial { ... }`. ([#119][] by [@nicoburns][])
- Breaking change: `Image` has been renamed to `ImageBrush`, which now consists of an `ImageData` and an `ImageSampler`. ([#117][], [#123][] by [@nicoburns][], [@DJMcNab][])
  To create an `ImageBrush` easily, you can use `ImageData {...}.into()`.
- Breaking change: `Font` has been renamed to `FontData` to match `ImageData`. ([#126][] by [@nicoburns][])
- Breaking change: The angle directions of `SweepGradientPosition` are now described to be clockwise in a Y-down coordinate system, as is common in computer graphics.
  This is reversed from the most likely reading of the previous wording.
  More generally, the angle directions are now described numerically to be unambiguous across coordinate systems.
  The angle unit is also now specified to be in radians. ([#130][] by [@tomcur][])
- Breaking change: Update to [Kurbo v0.12.0](https://github.com/linebender/kurbo/releases/tag/v0.12.0). ([#127][] by [@nicoburns][])
- `Mix::Clip` is now deprecated. To access the same functionality of optimised clips in Vello, you should now use `push_clip_layer`. ([#144][] by [@DJMcNab][])

## [0.4.1][] (2025-09-15)

This release has an [MSRV] of 1.82.

### Changed

- Use [Linebender Resource Handle](#linebender-resource-handle) for `Font`, `Blob`, and `WeakBlob`. ([#129][] by [@DJMcNab][], [@nicoburns][])

### Linebender Resource Handle

Peniko's `Font` (and therefore also `Blob`) are used as vocabulary types for font resources between crates.
However, this means that when Peniko made semver-incompatible releases, those crates could no longer (easily) interoperate.
To resolve this, `Font`, `Blob`, and `WeakBlob` are now re-exports from a new crate called [Linebender Resource Handle](https://crates.io/crates/linebender_resource_handle).
These types have identical API as in previous releases, but will now be the same type across Peniko versions.

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
[#115]: https://github.com/linebender/peniko/pull/115
[#117]: https://github.com/linebender/peniko/pull/117
[#119]: https://github.com/linebender/peniko/pull/119
[#120]: https://github.com/linebender/peniko/pull/120
[#121]: https://github.com/linebender/peniko/pull/121
[#123]: https://github.com/linebender/peniko/pull/123
[#124]: https://github.com/linebender/peniko/pull/124
[#126]: https://github.com/linebender/peniko/pull/126
[#127]: https://github.com/linebender/peniko/pull/127
[#129]: https://github.com/linebender/peniko/pull/129
[#130]: https://github.com/linebender/peniko/pull/130
[#144]: https://github.com/linebender/peniko/pull/144
[#147]: https://github.com/linebender/peniko/pull/147
[#148]: https://github.com/linebender/peniko/pull/148
[#155]: https://github.com/linebender/peniko/pull/155

[@dfrg]: https://github.com/dfrg
[@DJMcNab]: https://github.com/DJMcNab
[@liferooter]: https://github.com/liferooter
[@nicoburns]: https://github.com/nicoburns
[@ratmice]: https://github.com/ratmice
[@sagudev]: https://github.com/sagudev
[@tomcur]: https://github.com/tomcur
[@waywardmonkeys]: https://github.com/waywardmonkeys

[Unreleased]: https://github.com/linebender/peniko/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/linebender/peniko/compare/v0.5.0...v0.6.0
<!-- Note that this still comparing against 0.4.0, because 0.4.1 is a cherry-picked patch -->
[0.5.0]: https://github.com/linebender/peniko/compare/v0.4.0...v0.5.0
[0.4.1]: https://github.com/linebender/peniko/compare/v0.4.0...v0.4.1
<!-- Note that this still comparing against 0.3.1, because 0.3.2 is a cherry-picked patch -->
[0.4.0]: https://github.com/linebender/peniko/compare/v0.3.1...v0.4.0
[0.3.2]: https://github.com/linebender/peniko/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/linebender/peniko/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/linebender/peniko/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/linebender/peniko/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/linebender/peniko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/linebender/peniko/releases/tag/v0.1.0
