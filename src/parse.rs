// Copyright 2022 The peniko authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

/// Errors that may occur when parsing values from strings.
#[derive(Debug)]
pub enum ParseError {
    /// Invalid length.
    InvalidLength,
    /// An invalid hex digit.
    InvalidHexDigit { ch: char, index: usize },
    /// An invalid color name.
    InvalidColorName,
}
