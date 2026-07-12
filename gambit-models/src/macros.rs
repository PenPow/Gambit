macro_rules! define_map {
    (
        $(#[$meta:meta])*
        $name:ident, $key:ty, $count:expr
    ) => {
        $(#[$meta])*
        #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
        pub struct $name<T>([T; $count]);

        impl<T: Default> $name<T> {
            #[doc = concat!(
                "Creates a new ", stringify!($name), " with all values set to their default."
            )]
             pub fn new() -> Self {
                Self(std::array::from_fn(|_| T::default()))
            }
        }

        impl<T: Default> Default for $name<T> {
            fn default() -> Self {
                Self::new()
            }
        }

        impl<T: Copy> $name<T> {
            #[doc = concat!(
                "Create a ", stringify!($name), " filled with the same value"
            )]
            pub const fn filled(value: T) -> Self {
                Self([value; $count])
            }
        }

        impl<T> $name<T> {
            #[doc = concat!(
                "Create a ", stringify!($name), " by applying a function each index"
            )]
            pub fn from_fn<F: Fn(usize) -> T>(func: F) -> Self {
                Self(std::array::from_fn(func))
            }

            #[doc = concat!(
                "Iterates over `(", stringify!($key), ", &T)` pairs in index order."
            )]
            pub fn iter(&self) -> impl Iterator<Item = ($key, &T)> {
                <$key>::ALL.iter().copied().map(|k| (k, &self.0[k.bits() as usize]))
            }

            #[doc = concat!(
                "Iterates over `(", stringify!($key), ", &mut T)` pairs in index order."
            )]
            pub fn iter_mut(&mut self) -> impl Iterator<Item = ($key, &mut T)> {
                <$key>::ALL.iter().copied().zip(self.0.iter_mut())
            }

            #[doc = concat!(
                "Constructs an instance of `self` from a `[T; ", stringify!($count), "]`."
            )]
            pub const fn from_array(array: [T; $count]) -> Self {
                Self(array)
            }
        }

        impl<T> ::std::ops::Index<$key> for $name<T> {
            type Output = T;
            fn index(&self, key: $key) -> &T {
                &self.0[key.bits() as usize]
            }
        }

        impl<T> ::std::ops::IndexMut<$key> for $name<T> {
            fn index_mut(&mut self, key: $key) -> &mut T {
                &mut self.0[key.bits() as usize]
            }
        }

        impl<T> IntoIterator for $name<T> {
            type Item = T;
            type IntoIter = ::std::array::IntoIter<T, $count>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.into_iter()
            }
        }

        impl<'a, T> IntoIterator for &'a $name<T> {
            type Item = &'a T;
            type IntoIter = ::std::slice::Iter<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter()
            }
        }

        impl<'a, T> IntoIterator for &'a mut $name<T> {
            type Item = &'a mut T;
            type IntoIter = ::std::slice::IterMut<'a, T>;
            fn into_iter(self) -> Self::IntoIter {
                self.0.iter_mut()
            }
        }

        impl<T: Default> FromIterator<($key, T)> for $name<T> {
            fn from_iter<I: IntoIterator<Item = ($key, T)>>(iter: I) -> Self {
                let mut map = Self::default();
                for (key, value) in iter {
                    map[key] = value;
                }
                map
            }
        }

        impl<T> From<[T; $count]> for $name<T> {
            fn from(arr: [T; $count]) -> Self {
                Self(arr)
            }
        }

        impl<T> From<$name<T>> for [T; $count] {
            fn from(map: $name<T>) -> [T; $count] {
                map.0
            }
        }

        impl<T: ::std::fmt::Display> ::std::fmt::Display for $name<T> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                for key in <$key>::ALL {
                    writeln!(f, "{}: {}", key, self[key])?;
                }
                Ok(())
            }
        }
    };
}

macro_rules! define_squares {
    ($($name: ident = $value:expr), * $(,)?) => {
        impl Square {
            $(
                pub const $name: Self = unsafe { Self::from_index_unchecked($value) };
            )*
        }
    };
}

pub(crate) use define_map;
pub(crate) use define_squares;
