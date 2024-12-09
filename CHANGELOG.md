# Changelog

<!-- Instructions

This changelog follows the patterns described here: <https://keepachangelog.com/en/1.0.0/>.

Subheadings to categorize changes are `added, changed, deprecated, removed, fixed, security`.

-->

The latest published Peniko release is [0.2.0](#020-2024-09-19) which was released on 2024-09-19.
You can find its changes [documented below](#020-2024-09-19).

## [Unreleased]

This release has an [MSRV] of 1.82.

### Added

- `Gradient`, `Image`, `Brush` now have `with_alpha` and `Gradient` also gets a `multiply_alpha` ([#67][] by [@waywardmonkeys][])
- `Gradient` now tracks a hue direction and interpolation color space ([#71][] by [@waywardmonkeys][])
- `Compose`, `Extend`, `Fill`, and `Mix` now optional impl `bytemuck` traits ([#72][] by [@waywardmonkeys][])

### Changed

- `Image` now stores the alpha as an `f32` ([#65][] by [@waywardmonkeys][])
- Use `color` crate. See below for details ([#63][] by [@waywardmonkeys][])

### Color Changes

The old code behind `peniko::Color` has been removed and color functionality is now provided by the [`color`] crate.

This leads to a number of breaking changes:

- `peniko::Color` is now a type alias for `AlphaColor<Srgb>` from the `color` crate.
- `AlphaColor` does not, at this time, impl `Default`, `PartialEq`, `PartialOrd`, or `Hash`.
- `Brush` and `BrushRef` no longer impl `PartialEq`.
- `ColorStop` no longer impls `Default` or `PartialOrd`.
- `Brush`, `BrushRef`, and `ColorStop` can be constructed from a variety of color types, although, for now, `Brush` and `BrushRef` convert this internally into an unclipped `AlphaColor<Srgb>`.
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

[@DJMcNab]: https://github.com/DJMcNab
[@ratmice]: https://github.com/ratmice
[@waywardmonkeys]: https://github.com/waywardmonkeys

[Unreleased]: https://github.com/linebender/peniko/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/linebender/peniko/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/linebender/peniko/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/linebender/peniko/releases/tag/v0.1.0
