//! A 64-bit integer representation of a set of squares.
//!
//! A [`Bitboard`] is a `u64` where each bit corresponds to a single square:
//! bit 0 = `a1`, bit 1 = `b1`, etc..
//!
//! # Bit layout
//!
//! ```text
//! 56 57 58 59 60 61 62 63   <- rank 8 (a8..h8)
//! 48 49 50 51 52 53 54 55   <- rank 7
//! ...
//!  0  1  2  3  4  5  6  7   <- rank 1 (a1..h1)
//! ```
//!
//! # Set operations
//!
//! A [`Bitboard`] can perform all set operations either through the associated functions
//! as well as standard bit operations via operator overloads.
//!
//! The right-hand side of any bitwise operator can be any type implementing [`IntoBitboard`] - including [`Square`], [`File`] and [`Rank`]
//!
//! ```rust
//! # use gambit_models::bitboard::Bitboard;
//! # use gambit_models::location::file::File;
//! # use gambit_models::location::rank::Rank;
//! # use gambit_models::location::square::Square;
//! let e_file = File::E.bitboard();
//! let rank_4 = Bitboard::from(Rank::Four);
//!
//! let occupied = e_file | rank_4;
//! let e4 = occupied & Square::E4;
//!
//! assert_eq!(e4.into_iter().count(), 1);
//! ```
//!
//! # Iteration
//!
//! Iterating over a [`Bitboard`] yields each set [`Square`] in order of lowest index (`a1`) to highest index (`h8`)
//!
//! ```rust
//! # use gambit_models::bitboard::Bitboard;
//! # use gambit_models::location::rank::Rank;
//! for square in Rank::Five.bitboard() {
//!     println!("{square}")
//! }
//! ```
//!
//! # Features
//!
//! The [`Bitboard`] struct implements [`Display`](std::fmt::Display). To minimise dependencies, by default the output contains no colours.
//! By enabling the `colored` feature, displaying a Bitboard will colour code the bits to aid development.
//!
//! | Feature | Description |
//! |---------|-------------|
//! | `colored` | Coloured terminal output for [`Bitboard`] |
//!
//! # Further reading
//!
//! For more information, see the [Chess Programming Wiki](https://www.chessprogramming.org/Bitboards) as it contains extensive material on the usage of bitboards.

mod bitops;
mod fmt;
mod iterators;
mod parsers;

use crate::location::file::File;
use crate::location::rank::Rank;
use crate::location::square::Square;
use crate::traits::IntoBitboard;

// Expose to public API
pub use iterators::CarryRippler;

/// A 64-bit set of squares.
///
/// Each bit corresponds to one square: bit 0 = `a1`, bit 63 = `h8`.
/// All operations are branchless bitwise arithmetic over the underlying
/// `u64`. See the [module documentation][self] for the full bit layout
/// and usage examples.
#[repr(transparent)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Bitboard(u64);

impl Bitboard {
    /// An empty [`Bitboard`] with no squares set.
    ///
    /// This is the identity element for [`union`][Bitboard::union] and
    /// the absorbing element for [`intersection`][Bitboard::intersection].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// assert!(Bitboard::EMPTY.is_empty());
    /// assert_eq!(Bitboard::EMPTY.bits(), 0);
    /// ```
    pub const EMPTY: Bitboard = Bitboard::new(0);

    /// A [`Bitboard`] with all 64 squares set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// assert_eq!(Bitboard::UNIVERSE.bits(), u64::MAX);
    /// assert_eq!(Bitboard::UNIVERSE.into_iter().count(), 64);
    /// ```
    pub const UNIVERSE: Bitboard = Bitboard::new(u64::MAX);

    /// Creates a [`Bitboard`] from a raw 64-bit integer.
    ///
    /// Each set bit in `bits` corresponds to the square at that index
    /// (0 = `a1`, 63 = `h8`).
    ///
    /// Prefer [`Bitboard::from_square`], [`Bitboard::from_rank`], or
    /// [`Bitboard::from_file`] (or the [`From`] trait implementations) for construction from library models, and
    /// [`Bitboard::EMPTY`] / [`Bitboard::UNIVERSE`] for the boundary values.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// let bb = Bitboard::new(0x0000_0000_0000_00FF);
    /// assert_eq!(bb.into_iter().count(), 8);
    /// ```
    #[inline(always)]
    pub const fn new(bits: u64) -> Bitboard {
        Self(bits)
    }

    /// Creates a [`Bitboard`] with a single bit set at the given square.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::square::Square;
    /// let bb = Bitboard::from_square(Square::E4);
    /// assert!(bb.contains(Square::E4));
    /// assert_eq!(bb.into_iter().count(), 1);
    /// ```
    #[inline(always)]
    pub const fn from_square(square: Square) -> Self {
        Bitboard::new(1 << square.bits())
    }

    /// Creates a [`Bitboard`] with all 8 squares on the given rank set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::rank::Rank;
    /// let rank1 = Bitboard::from_rank(Rank::One);
    /// assert_eq!(rank1.bits(), 0x0000_0000_0000_00FF);
    /// assert_eq!(rank1.into_iter().count(), 8);
    /// ```
    #[inline(always)]
    pub const fn from_rank(rank: Rank) -> Self {
        Bitboard::new(0xFF_u64 << ((rank.bits() as u32) << 3))
    }

    /// Creates a [`Bitboard`] with all 8 squares on the given file set.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::file::File;
    /// let a_file = Bitboard::from_file(File::A);
    /// assert_eq!(a_file.bits(), 0x0101_0101_0101_0101);
    /// assert_eq!(a_file.into_iter().count(), 8);
    /// ```
    #[inline(always)]
    pub const fn from_file(file: File) -> Self {
        Bitboard::new(0x0101_0101_0101_0101_u64 << (file as u32))
    }

    /// Returns `true` if no squares are set.
    ///
    /// This is the inverse of [`any`][Bitboard::any].
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.bits() == Bitboard::EMPTY.bits()
    }

    /// Returns `true` if at least one square is set.
    ///
    /// This is the inverse of [`is_empty`][Bitboard::is_empty].
    #[inline]
    pub const fn any(self) -> bool {
        self.bits() != Bitboard::EMPTY.bits()
    }

    /// Returns `true` if the given square is set in this bitboard.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::square::Square;
    /// let bb = Square::E4.bitboard();
    /// assert!(bb.contains(Square::E4));
    /// assert!(!bb.contains(Square::E5));
    /// ```
    #[inline]
    pub const fn contains(self, square: Square) -> bool {
        self.bits() & square.bitboard().bits() != Bitboard::EMPTY.bits()
    }

    /// Returns the set-theoretic intersection of `self` and `b` (AND).
    ///
    /// The result contains exactly the squares present in both operands.
    /// Equivalent to `self & b`.
    #[doc(alias = "and")]
    #[inline]
    pub const fn intersection(self, b: Bitboard) -> Bitboard {
        Bitboard::new(self.bits() & b.bits())
    }

    /// Returns the set-theoretic union of `self` and `b` (OR).
    ///
    /// The result contains all squares present in either operand.
    /// Equivalent to `self | b`.
    #[doc(alias = "or")]
    #[inline]
    pub const fn union(self, b: Bitboard) -> Bitboard {
        Bitboard::new(self.bits() | b.bits())
    }

    /// Returns the complement of `self` — all squares *not* in `self`.
    ///
    /// Equivalent to `!self`.
    #[doc(alias = "not")]
    #[inline]
    pub const fn negation(self) -> Bitboard {
        Bitboard::new(!self.bits())
    }

    /// Returns the set difference: squares in `self` but not in `b`.
    ///
    /// Equivalent to `self & !b`.
    #[inline]
    pub const fn difference(self, b: Bitboard) -> Bitboard {
        Bitboard::new(self.bits() & !b.bits())
    }

    /// Returns the symmetric difference: squares in exactly one of `self` or `b`.
    ///
    /// Equivalent to `self ^ b`.
    #[doc(alias = "xor")]
    #[inline]
    pub const fn symmetric_difference(self, b: Bitboard) -> Bitboard {
        Bitboard::new(self.bits() ^ b.bits())
    }

    /// Returns `true` if every square in `self` is also in `b`.
    #[inline]
    pub const fn is_subset_of(self, b: Bitboard) -> bool {
        let a = self.bits();

        (a & b.bits()) == a
    }

    /// Returns `true` if every square in `b` is also in `self`.
    #[inline]
    pub const fn is_superset_of(self, b: Bitboard) -> bool {
        (self.bits() & b.bits()) == b.bits()
    }

    /// Returns `true` if `self` and `b` share no squares.
    #[inline]
    pub const fn is_disjoint(self, b: Bitboard) -> bool {
        (self.bits() & b.bits()) == Bitboard::EMPTY.bits()
    }

    /// Sets all squares in `bitboard` in `self` (in place OR).
    ///
    /// Accepts any type implementing [`IntoBitboard`], including
    /// [`Square`], [`File`], and [`Rank`].
    #[inline]
    pub fn add<T: IntoBitboard>(&mut self, bitboard: T) {
        *self |= bitboard;
    }

    /// Toggles all squares in `bitboard` in `self` (in place XOR).
    ///
    /// Accepts any type implementing [`IntoBitboard`], including
    /// [`Square`], [`File`], and [`Rank`].
    #[inline]
    pub fn toggle<T: IntoBitboard>(&mut self, bitboard: T) {
        *self ^= bitboard;
    }

    /// Clears all squares in `bitboard` from `self` (in place AND NOT).
    ///
    /// Unlike [`remove`][Bitboard::remove], this is a no-op if any
    /// of the squares are already absent.
    #[inline]
    pub fn discard<T: IntoBitboard>(&mut self, bitboard: T) {
        *self &= !bitboard.into_bitboard();
    }

    /// Sets or clears `square` according to `value`.
    #[inline]
    pub fn set(&mut self, square: Square, value: bool) {
        if value {
            self.add(square);
        } else {
            self.discard(square);
        }
    }

    /// Removes `square` from this bitboard, returning `true` if it was set.
    ///
    /// Use [`discard`][Bitboard::discard] if you do not need the return value.
    #[must_use = "use Bitboard::discard() if no return value needed"]
    #[inline]
    pub fn remove(&mut self, square: Square) -> bool {
        if self.contains(square) {
            self.toggle(square);
            true
        } else {
            false
        }
    }

    /// Gets the number of bits set to `1`
    #[inline]
    pub fn count(&self) -> u32 {
        self.bits().count_ones()
    }

    /// Removes all squares, setting this bitboard to [`EMPTY`][Bitboard::EMPTY].
    #[inline]
    pub const fn clear(&mut self) {
        self.0 = 0;
    }

    /// Removes and returns the lowest-index (LSB) set square, or `None` if empty.
    ///
    /// Used to iterate over set squares in ascending order. For a full iteration,
    /// prefer [`IntoIterator`]:
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::rank::Rank;
    /// for square in Rank::One.bitboard() {
    ///     println!("{square}");
    /// }
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            let square = Square::from_index(self.bits().trailing_zeros() as u8);
            *self = Bitboard::new(self.bits() & self.bits().wrapping_sub(1));

            Some(square)
        }
    }

    /// Removes and returns the highest-index (MSB) set square, or `None` if empty.
    ///
    /// The reverse of [`pop`][Bitboard::pop]. Used to iterate over set squares
    /// in descending order, or as the [`DoubleEndedIterator::next_back`]
    /// implementation for `BitboardIterator`.
    #[inline]
    pub fn pop_back(&mut self) -> Option<Square> {
        if self.is_empty() {
            None
        } else {
            let square = Square::from_index(63 - self.bits().leading_zeros() as u8);
            *self ^= square.bitboard();

            Some(square)
        }
    }

    /// Returns the raw 64-bit integer representation.
    #[inline(always)]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Bitboard::EMPTY
    }
}
