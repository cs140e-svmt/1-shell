#![cfg_attr(test, feature(inclusive_range_syntax))]
#![no_std]

#[cfg(test)]
mod tests;

use core::ops::Index;
use core::ops::IndexMut;

pub struct Iter<'a, T: 'a> {
    slice: &'a [T],
    index: usize
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.index;

        if index < self.slice.len() {
            self.index += 1;
            Some(&self.slice[index])
        }
        else {
            None
        }
    }
}


/// A contiguous array type backed by a slice.
///
/// `StackVec`'s functionality is similar to that of `std::Vec`. You can `push`
/// and `pop` and iterate over the vector. Unlike `Vec`, however, `StackVec`
/// requires no memory allocation as it is backed by a user-supplied slice. As a
/// result, `StackVec`'s capacity is _bounded_ by the user-supplied slice. This
/// results in `push` being fallible: if `push` is called when the vector is
/// full, an `Err` is returned.
#[derive(Debug)]
pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize
}

impl<'a, T: 'a> StackVec<'a, T> {
    /// Constructs a new, empty `StackVec<T>` using `storage` as the backing
    /// store. The returned `StackVec` will be able to hold `storage.len()`
    /// values.
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        StackVec {
            storage: storage,
            len: 0
        }
    }

    /// Constructs a new `StackVec<T>` using `storage` as the backing store. The
    /// first `len` elements of `storage` are treated as if they were `push`ed
    /// onto `self.` The returned `StackVec` will be able to hold a total of
    /// `storage.len()` values.
    ///
    /// # Panics
    ///
    /// Panics if `len > storage.len()`.
    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        if len > storage.len() {
            panic!("Specified length is greater than storage length");
        }

        StackVec {
            storage: storage,
            len: len
        }
    }

    /// Returns the number of elements this vector can hold.
    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    /// Shortens the vector, keeping the first `len` elements. If `len` is
    /// greater than the vector's current length, this has no effect. Note that
    /// this method has no effect on the capacity of the vector.
    pub fn truncate(&mut self, len: usize) {
        if len < self.len {
            self.len = len;
        }
    }

    /// Extracts a slice containing the entire vector, consuming `self`.
    ///
    /// Note that the returned slice's length will be the length of this vector,
    /// _not_ the length of the original backing storage.
    pub fn into_slice(self) -> &'a mut [T] {
        unimplemented!()
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.storage[0..self.len]
    }

    /// Extracts a mutable slice of the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.storage[0..self.len]
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            slice: &self.storage[0..self.len],
            index: 0
        }
    }

    /// Returns the number of elements in the vector, also referred to as its
    /// 'length'.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if the vector is at capacity.
    pub fn is_full(&self) -> bool {
        self.len == self.storage.len()
    }

    /// Appends `value` to the back of this vector if the vector is not full.
    ///
    /// # Error
    ///
    /// If this vector is full, an `Err` is returned. Otherwise, `Ok` is
    /// returned.
    pub fn push(&mut self, value: T) -> Result<(), ()> {
        if self.is_full() {
            Err(())
        }
        else {
            let index = self.len;
            self.storage[index] = value;
            self.len += 1;
            Ok(())
        }
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    /// If this vector is not empty, removes the last element from this vector
    /// by cloning it and returns it. Otherwise returns `None`.
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(self.storage[self.len].clone())
        }
        else {
            None
        }
    }
}

impl<'a, T: 'a> Index<usize> for StackVec<'a, T> {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        if index >= self.len() {
            panic!("Index out of range");
        }

        &self.storage[index]
    }
}

impl<'a, T: 'a> IndexMut<usize> for StackVec<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len() {
            panic!("Index out of range");
        }

        &mut self.storage[index]
    }
}

pub struct IntoIter<'a, T: 'a> {
    vec: StackVec<'a, T>,
    index: usize
}

impl<'a, T: 'a> Iterator for IntoIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        //let index = self.index;

        //if index < self.vec.len() {
        //    self.index += 1;
        //    let item = &self.vec[index];
        //    Some(&item)
        //}
        //else {
        //    None
        //}
        None
    }
}

impl<'a, T> IntoIterator for StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = IntoIter<'a, T>;

    fn into_iter(self) -> IntoIter<'a, T> {
        IntoIter {
            vec: self,
            index: 0
        }
    }
}

impl<'a, T> IntoIterator for &'a StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Iter<'a, T> {
        Iter {
            slice: self.as_slice(),
            index: 0
        }
    }
}

// FIXME: Implement `Deref`, `DerefMut`, and `IntoIterator` for `StackVec`.
// FIXME: Implement IntoIterator` for `&StackVec`.
