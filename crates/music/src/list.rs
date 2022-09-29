use std::iter::FromIterator;

// ********************************************************************************************** //
// * List                                                                                       * //
// ********************************************************************************************** //

pub const CAPACITY: usize = 12;

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct List<T> {
    items: [T; 12],
    len: u8,
}

impl<T> List<T> {
    pub fn new() -> Self
    where
        T: Whatever,
    {
        Self {
            items: std::array::from_fn(|_| T::whatever()),
            len: 0,
        }
    }

    pub const fn from_raw(len: u8, items: [T; 12]) -> Self {
        Self { items, len }
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn push(&mut self, item: T) {
        assert!(self.len() != CAPACITY);
        self.items[self.len()] = item;
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T>
    where
        T: Whatever,
    {
        if self.len() == 0 {
            None
        } else {
            let pop = std::mem::replace(&mut self.items[self.len() - 1], T::whatever());
            self.len -= 1;
            Some(pop)
        }
    }
}

impl<T: Whatever> FromIterator<T> for List<T> {
    fn from_iter<I: IntoIterator<Item = T>>(items: I) -> Self {
        let mut list = List::new();

        for item in items {
            list.push(item);
        }

        list
    }
}

impl<const N: usize, T: Whatever> From<[T; N]> for List<T> {
    fn from(items: [T; N]) -> Self {
        Self::from_iter(items)
    }
}

impl<T> AsRef<[T]> for List<T> {
    fn as_ref(&self) -> &[T] {
        &self.items[..self.len()]
    }
}

impl<T> AsMut<[T]> for List<T> {
    fn as_mut(&mut self) -> &mut [T] {
        let len = self.len();
        &mut self.items[..len]
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for List<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_ref()).finish()
    }
}

// ********************************************************************************************** //
// * Whatever                                                                                   * //
// ********************************************************************************************** //

pub trait Whatever {
    fn whatever() -> Self;
}

mod whatever {
    use crate::*;

    fn whatever<T: Whatever>() -> T {
        T::whatever()
    }

    impl<T> Whatever for Option<T> {
        fn whatever() -> Self {
            None
        }
    }

    impl<T: Whatever> Whatever for List<T> {
        fn whatever() -> Self {
            List::new()
        }
    }

    macro_rules! whatever {
        ($($Type:ty: $value:expr,)*) => { $(
            impl Whatever for $Type {
                fn whatever() -> Self {
                    $value
                }
            }
        )* };
    }

    whatever! {
        u8: u8::default(),
        Accidental: Natural,
        NoteName: A,
        IntervalName: Unisson,
        Note: Note(whatever(), whatever()),
        Interval: Interval(whatever(), whatever()),
        Intervals: Intervals(whatever()),
    }
}
