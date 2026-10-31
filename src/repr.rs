/// A number of bits, as a type.
///
/// See [`Repr`].
pub struct Bits<const N: usize>;

/// Maps a number of bits to the narrowest unsigned integer that holds them.
///
/// It is implemented for [`Bits<N>`](Bits) with `N` from 1 to 64. The code
/// of a module that wraps the errors of another crate takes its underlying
/// type from it, because the number of bits of the wrapped error kinds is
/// known only when the module is compiled:
///
/// ```rust
/// # use tighterror::{Bits, Repr};
/// const KIND_BITS: usize = 9;
/// type R = <Bits<KIND_BITS> as Repr>::R;
/// assert_eq!(R::BITS, 16);
/// ```
#[diagnostic::on_unimplemented(
    message = "error kinds of `{Self}` don't fit in `u64`, the widest supported underlying type"
)]
pub trait Repr {
    /// The unsigned integer.
    type R;
}

macro_rules! impl_repr {
    ($r:ty: $($n:literal)*) => {
        $(impl Repr for Bits<$n> { type R = $r; })*
    };
}

impl_repr!(u8: 1 2 3 4 5 6 7 8);
impl_repr!(u16: 9 10 11 12 13 14 15 16);
impl_repr!(u32: 17 18 19 20 21 22 23 24 25 26 27 28 29 30 31 32);
impl_repr!(u64:
    33 34 35 36 37 38 39 40 41 42 43 44 45 46 47 48
    49 50 51 52 53 54 55 56 57 58 59 60 61 62 63 64);

#[cfg(test)]
mod tests {
    use super::*;
    use core::any::TypeId;

    fn repr<const N: usize>() -> TypeId
    where
        Bits<N>: Repr<R: 'static>,
    {
        TypeId::of::<<Bits<N> as Repr>::R>()
    }

    // Checks the type on both sides of every edge between two widths. It
    // simulates error kinds of 1, 8, 9, 16, 17, 32, 33 and 64 bits.
    #[test]
    fn test_repr() {
        // the narrowest and the widest number of bits of a u8
        assert_eq!(repr::<1>(), TypeId::of::<u8>());
        assert_eq!(repr::<8>(), TypeId::of::<u8>());

        // one bit more needs a u16, and so on up to a u64
        assert_eq!(repr::<9>(), TypeId::of::<u16>());
        assert_eq!(repr::<16>(), TypeId::of::<u16>());
        assert_eq!(repr::<17>(), TypeId::of::<u32>());
        assert_eq!(repr::<32>(), TypeId::of::<u32>());
        assert_eq!(repr::<33>(), TypeId::of::<u64>());
        assert_eq!(repr::<64>(), TypeId::of::<u64>());
    }
}
