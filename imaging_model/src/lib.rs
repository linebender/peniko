// Copyright 2026 the Imaging Model Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Data model for describing Linebender imaging operations.
//!
//! An imaging model defines the semantic operations an implementation consumes:
//! what gets drawn, how it is composited, and how stateful operations such as
//! clipping affect subsequent draws.
//!
//! This crate is the initial core of that model. The current scope is
//! intentionally small, starting with foundational clip-related types and
//! invariants that are useful across renderers.
//!
//! Expect iteration and occasional breaking changes while the model is refined.

#![no_std]

mod clip;

pub use crate::clip::{Clip, ClipGeometry};
pub use peniko::{BlendMode, Fill};
