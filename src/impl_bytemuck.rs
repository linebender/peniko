// Copyright 2024 the Peniko Authors
// SPDX-License-Identifier: Apache-2.0 OR MIT

#![allow(unsafe_code, reason = "unsafe is required for bytemuck unsafe impls")]

use crate::{Compose, Extend, Fill, Mix};

// Safety: The enum is `repr(u8)` and has only fieldless variants.
unsafe impl bytemuck::NoUninit for Compose {}

// Safety: The enum is `repr(u8)` and `0` is a valid value.
unsafe impl bytemuck::Zeroable for Compose {}

// Safety: The enum is `repr(u8)`.
unsafe impl bytemuck::checked::CheckedBitPattern for Compose {
    type Bits = u8;

    fn is_valid_bit_pattern(bits: &u8) -> bool {
        use bytemuck::Contiguous;
        // Don't need to compare against MIN_VALUE as this is u8 and 0 is the MIN_VALUE.
        *bits <= Self::MAX_VALUE
    }
}

// Safety: The enum is `repr(u8)`. All values are `u8` and fall within
// the min and max values.
unsafe impl bytemuck::Contiguous for Compose {
    type Int = u8;
    const MIN_VALUE: u8 = Self::Clear as u8;
    const MAX_VALUE: u8 = Self::PlusLighter as u8;
}

// Safety: The enum is `repr(u8)` and has only fieldless variants.
unsafe impl bytemuck::NoUninit for Extend {}

// Safety: The enum is `repr(u8)` and `0` is a valid value.
unsafe impl bytemuck::Zeroable for Extend {}

// Safety: The enum is `repr(u8)`.
unsafe impl bytemuck::checked::CheckedBitPattern for Extend {
    type Bits = u8;

    fn is_valid_bit_pattern(bits: &u8) -> bool {
        use bytemuck::Contiguous;
        // Don't need to compare against MIN_VALUE as this is u8 and 0 is the MIN_VALUE.
        *bits <= Self::MAX_VALUE
    }
}

// Safety: The enum is `repr(u8)`. All values are `u8` and fall within
// the min and max values.
unsafe impl bytemuck::Contiguous for Extend {
    type Int = u8;
    const MIN_VALUE: u8 = Self::Pad as u8;
    const MAX_VALUE: u8 = Self::Reflect as u8;
}

// Safety: The enum is `repr(u8)` and has only fieldless variants.
unsafe impl bytemuck::NoUninit for Fill {}

// Safety: The enum is `repr(u8)` and `0` is a valid value.
unsafe impl bytemuck::Zeroable for Fill {}

// Safety: The enum is `repr(u8)`.
unsafe impl bytemuck::checked::CheckedBitPattern for Fill {
    type Bits = u8;

    fn is_valid_bit_pattern(bits: &u8) -> bool {
        use bytemuck::Contiguous;
        // Don't need to compare against MIN_VALUE as this is u8 and 0 is the MIN_VALUE.
        *bits <= Self::MAX_VALUE
    }
}

// Safety: The enum is `repr(u8)`. All values are `u8` and fall within
// the min and max values.
unsafe impl bytemuck::Contiguous for Fill {
    type Int = u8;
    const MIN_VALUE: u8 = Self::NonZero as u8;
    const MAX_VALUE: u8 = Self::EvenOdd as u8;
}

// Safety: The enum is `repr(u8)` and has only fieldless variants.
unsafe impl bytemuck::NoUninit for Mix {}

// Safety: The enum is `repr(u8)` and `0` is a valid value.
unsafe impl bytemuck::Zeroable for Mix {}

// Safety: The enum is `repr(u8)`.
unsafe impl bytemuck::checked::CheckedBitPattern for Mix {
    type Bits = u8;

    fn is_valid_bit_pattern(bits: &u8) -> bool {
        *bits <= Self::Luminosity as u8 || *bits == Self::Clip as u8
    }
}

#[cfg(test)]
mod tests {
    use crate::{Compose, Extend, Fill, Mix};
    use bytemuck::{checked::try_from_bytes, Contiguous, Zeroable};
    use core::ptr;

    #[test]
    fn checked_bit_pattern() {
        let valid = bytemuck::bytes_of(&1_u8);
        let invalid = bytemuck::bytes_of(&200_u8);

        assert_eq!(Ok(&Compose::Copy), try_from_bytes::<Compose>(valid));
        assert!(try_from_bytes::<Compose>(invalid).is_err());

        assert_eq!(Ok(&Extend::Repeat), try_from_bytes::<Extend>(valid));
        assert!(try_from_bytes::<Extend>(invalid).is_err());

        assert_eq!(Ok(&Fill::EvenOdd), try_from_bytes::<Fill>(valid));
        assert!(try_from_bytes::<Fill>(invalid).is_err());

        assert_eq!(Ok(&Mix::Multiply), try_from_bytes::<Mix>(valid));
        assert!(try_from_bytes::<Mix>(invalid).is_err());
    }

    #[test]
    fn contiguous() {
        let compose1 = Compose::PlusLighter;
        let compose2 = Compose::from_integer(compose1.into_integer());
        assert_eq!(Some(compose1), compose2);

        assert_eq!(None, Compose::from_integer(255));

        let extend1 = Extend::Repeat;
        let extend2 = Extend::from_integer(extend1.into_integer());
        assert_eq!(Some(extend1), extend2);

        assert_eq!(None, Extend::from_integer(255));

        let fill1 = Fill::EvenOdd;
        let fill2 = Fill::from_integer(fill1.into_integer());
        assert_eq!(Some(fill1), fill2);

        assert_eq!(None, Fill::from_integer(255));
    }

    #[test]
    fn zeroable() {
        let compose = Compose::zeroed();
        assert_eq!(compose, Compose::Clear);

        let extend = Extend::zeroed();
        assert_eq!(extend, Extend::Pad);

        let fill = Fill::zeroed();
        assert_eq!(fill, Fill::NonZero);

        let mix = Mix::zeroed();
        assert_eq!(mix, Mix::Normal);
    }

    /// Tests that the [`Contiguous`] impl for [`Compose`] is not trivially incorrect.
    const _: () = {
        let mut value = 0;
        while value <= Compose::MAX_VALUE {
            // Safety: In a const context, therefore if this makes an invalid Compose, that will be detected.
            let it: Compose = unsafe { ptr::read((&raw const value).cast()) };
            // Evaluate the enum value to ensure it actually has a valid tag
            if it as u8 != value {
                unreachable!();
            }
            value += 1;
        }
    };

    /// Tests that the [`Contiguous`] impl for [`Extend`] is not trivially incorrect.
    const _: () = {
        let mut value = 0;
        while value <= Extend::MAX_VALUE {
            // Safety: In a const context, therefore if this makes an invalid Extend, that will be detected.
            let it: Extend = unsafe { ptr::read((&raw const value).cast()) };
            // Evaluate the enum value to ensure it actually has a valid tag
            if it as u8 != value {
                unreachable!();
            }
            value += 1;
        }
    };

    /// Tests that the [`Contiguous`] impl for [`Fill`] is not trivially incorrect.
    const _: () = {
        let mut value = 0;
        while value <= Fill::MAX_VALUE {
            // Safety: In a const context, therefore if this makes an invalid Fill, that will be detected.
            let it: Fill = unsafe { ptr::read((&raw const value).cast()) };
            // Evaluate the enum value to ensure it actually has a valid tag
            if it as u8 != value {
                unreachable!();
            }
            value += 1;
        }
    };
}

#[cfg(doctest)]
/// Doctests aren't collected under `cfg(test)`; we can use `cfg(doctest)` instead
mod doctests {
    /// Validates that any new variants in `Compose` has led to a change in the `Contiguous` impl.
    /// Note that to test this robustly, we'd need 256 tests, which is impractical.
    /// We make the assumption that all new variants will maintain contiguousness.
    ///
    /// ```compile_fail,E0080
    /// use bytemuck::Contiguous;
    /// use peniko::Compose;
    /// const {
    ///     let value = Compose::MAX_VALUE + 1;
    ///     // Safety: In a const context, therefore if this makes an invalid Compose, that will be detected.
    ///     // (Indeed, we rely upon that)
    ///     let it: Compose = unsafe { core::ptr::read((&raw const value).cast()) };
    ///     // Evaluate the enum value to ensure it actually has an invalid tag
    ///     if it as u8 != value {
    ///         unreachable!();
    ///     }
    /// }
    /// ```
    const _COMPOSE: () = {};

    /// Validates that any new variants in `Extend` has led to a change in the `Contiguous` impl.
    /// Note that to test this robustly, we'd need 256 tests, which is impractical.
    /// We make the assumption that all new variants will maintain contiguousness.
    ///
    /// ```compile_fail,E0080
    /// use bytemuck::Contiguous;
    /// use peniko::Extend;
    /// const {
    ///     let value = Extend::MAX_VALUE + 1;
    ///     let it: Extend = unsafe { core::ptr::read((&raw const value).cast()) };
    ///     // Evaluate the enum value to ensure it actually has an invalid tag
    ///     if it as u8 != value {
    ///         unreachable!();
    ///     }
    /// }
    /// ```
    const _EXTEND: () = {};

    /// Validates that any new variants in `Fill` has led to a change in the `Contiguous` impl.
    /// Note that to test this robustly, we'd need 256 tests, which is impractical.
    /// We make the assumption that all new variants will maintain contiguousness.
    ///
    /// ```compile_fail,E0080
    /// use bytemuck::Contiguous;
    /// use peniko::Fill;
    /// const {
    ///     let value = Fill::MAX_VALUE + 1;
    ///     let it: Fill = unsafe { core::ptr::read((&raw const value).cast()) };
    ///     // Evaluate the enum value to ensure it actually has an invalid tag
    ///     if it as u8 != value {
    ///         unreachable!();
    ///     }
    /// }
    /// ```
    const _FILL: () = {};
}
