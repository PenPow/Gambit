use crate::bitboard::Bitboard;
use crate::error::{TryFromCharError, TryFromIntError};
use crate::location::map::file::FileMap;
use crate::traits::IntoBitboard;
use std::fmt::Write;

/// A file of the chessboard, from `a` to `h`.
///
/// Files are stored as `u8` with `#[repr(u8)]`:
/// `A = 0`, `B = 1`, ..., `H = 7`.
///
///
/// # Ordering
///
/// The derived [`Ord`] follows board convention: `A < B < ... < H`.
///
/// # Examples
///
/// ```rust
/// # use gambit_models::location::file::File;
/// assert_eq!(File::E.as_char(), 'e');
/// assert_eq!(File::A.distance(File::H), 7);
/// assert_eq!(File::C.offset(2), Some(File::E));
/// assert_eq!(File::H.offset(1), None);
/// ```
#[repr(u8)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    /// Total number of files on the board.
    pub const COUNT: usize = 8;

    /// The lowest file (`A`).
    pub const MIN: File = File::A;

    /// The highest file (`H`).
    pub const MAX: File = File::H;

    /// All 8 files in order from `A` to `H`.
    pub const ALL: [File; File::COUNT] = [
        File::A,
        File::B,
        File::C,
        File::D,
        File::E,
        File::F,
        File::G,
        File::H,
    ];

    /// Precomputed bitboard mask for each file.
    ///
    /// Non-const contexts should prefer using [`Self::MAP`].
    pub const BITBOARDS: [Bitboard; File::COUNT] = {
        let mut files = [Bitboard::EMPTY; File::COUNT];

        let mut file = 0;
        while file < File::COUNT {
            files[file] = Bitboard::from_file(File::from_index(file as u8));
            file += 1;
        }

        files
    };

    /// A precomputed [`FileMap`] containing each square bitboard.
    pub const MAP: FileMap<Bitboard> = FileMap::from_array(Self::BITBOARDS);

    /// Creates a `File` from its index.
    ///
    /// # Panics
    ///
    /// Panics if `index >= 8`.
    #[inline(always)]
    pub const fn from_index(index: u8) -> Self {
        Self::ALL[index as usize]
    }

    /// Creates a `File` from its index without bounds checking (in release builds).
    ///
    /// # Safety
    ///
    /// `index` must be in the range `0..=7`.Other values should be considered as undefined behaviour.
    ///
    /// Prefer [`File::from_index`] or [`TryFrom<u8>`] unless you have already established that `index`
    /// is in the range.
    #[inline(always)]
    pub const unsafe fn from_index_unchecked(index: u8) -> Self {
        debug_assert!(index < 8);

        // SAFETY: index is in range 0..=7 and so transmute to repr(u8) is safe
        unsafe { std::mem::transmute(index) }
    }

    /// Returns the lowercase letter for this file (`'a'`..=`'h'`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::file::File;
    /// assert_eq!(File::E.as_char(), 'e');
    /// assert_eq!(File::A.as_char(), 'a');
    /// assert_eq!(File::H.as_char(), 'h');
    /// ```
    #[inline]
    pub const fn as_char(self) -> char {
        (b'a' + (self as u8)) as char
    }

    /// Returns the absolute distance in files between `self` and `rhs`.
    ///
    /// Always returns a value in `0..=7`. Order does not matter:
    /// `a.distance(b) == b.distance(a)`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::file::File;
    /// assert_eq!(File::A.distance(File::H), 7);
    /// assert_eq!(File::E.distance(File::C), 2);
    /// assert_eq!(File::D.distance(File::D), 0);
    /// ```
    #[inline]
    pub const fn distance(self, rhs: File) -> u8 {
        (self as u8).abs_diff(rhs as u8)
    }

    /// Returns the file `rhs` steps away, or `None` if off the board.
    ///
    /// Positive `rhs` moves toward `H`; negative moves toward `A`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::location::file::File;
    /// assert_eq!(File::E.offset(2),  Some(File::G));
    /// assert_eq!(File::E.offset(-4), Some(File::A));
    /// assert_eq!(File::H.offset(1),  None);
    /// assert_eq!(File::A.offset(-1), None);
    /// ```
    #[inline]
    pub const fn offset(self, rhs: i8) -> Option<File> {
        let index = self as i8 + rhs;

        #[allow(clippy::manual_range_contains)]
        if index < 0 || index > 7 {
            return None;
        }

        // SAFETY: Check above validates index
        Some(unsafe { Self::from_index_unchecked(index as u8) })
    }

    /// Returns the raw `u8` index of this file (`A` = 0, `H` = 7).
    #[inline(always)]
    pub const fn bits(self) -> u8 {
        self as u8
    }

    /// Returns the precomputed bitboard mask for this file.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use gambit_models::bitboard::Bitboard;
    /// # use gambit_models::location::file::File;
    /// assert_eq!(File::A.bitboard(), Bitboard::from_file(File::A));
    /// assert_eq!(File::A.bitboard().bits(), 0x0101_0101_0101_0101);
    /// ```
    #[inline(always)]
    pub const fn bitboard(self) -> Bitboard {
        Self::BITBOARDS[self as usize]
    }
}

impl From<File> for u8 {
    #[inline(always)]
    fn from(file: File) -> u8 {
        file.bits()
    }
}

impl IntoBitboard for File {
    #[inline]
    fn into_bitboard(self) -> Bitboard {
        self.bitboard()
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.as_char())
    }
}

impl TryFrom<char> for File {
    type Error = TryFromCharError;

    fn try_from(character: char) -> Result<Self, Self::Error> {
        match character {
            'a'..='h' => Ok(File::from_index(character as u8 - b'a')),
            'A'..='H' => Ok(File::from_index(character as u8 - b'A')),
            _ => Err(TryFromCharError(character)),
        }
    }
}

impl TryFrom<u8> for File {
    type Error = TryFromIntError<u8>;

    fn try_from(index: u8) -> Result<Self, Self::Error> {
        if index < 8 {
            Ok(File::from_index(index))
        } else {
            Err(TryFromIntError(index))
        }
    }
}
