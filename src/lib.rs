// Similar to Vec<T>, except StackVec is backed by static memory storage.
// No memory allocater is needed.
// Works without `std`
#![no_std]

#[cfg(test)]
mod tests;

use core::ops::{Deref, DerefMut};

pub struct StackVec<'a, T: 'a> {
    storage: &'a mut [T],
    len: usize,
}

impl<'a, T: 'a> StackVec<'a, T> {
    pub fn new(storage: &'a mut [T]) -> StackVec<'a, T> {
        Self::with_len(storage, 0)
    }

    pub fn with_len(storage: &'a mut [T], len: usize) -> StackVec<'a, T> {
        if len > storage.len() {
            panic!("given `storage` cannot fit given `len`")
        }
        StackVec { storage, len }
    }

    pub fn capacity(&self) -> usize {
        self.storage.len()
    }

    pub fn truncate(&mut self, len: usize) {
        if len >= self.len {
            return;
        }
        self.len = len
    }

    pub fn into_slice(self) -> &'a mut [T] {
        &mut self.storage[..self.len]
    }

    pub fn as_slice(&self) -> &[T] {
        &self.storage[..self.len]
    }

    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.storage[..self.len]
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len >= self.storage.len()
    }

    pub fn push(&mut self, value: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("full");
        }
        self.storage[self.len] = value;
        self.len += 1;
        Ok(())
    }
}

impl<'a, T: Clone + 'a> StackVec<'a, T> {
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let x = self.storage[self.len - 1].clone();
        self.len -= 1;
        Some(x)
    }
}

impl<'a, T: 'a> Deref for StackVec<'a, T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        &self.storage[..self.len]
    }
}

impl<'a, T: 'a> DerefMut for StackVec<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.storage[..self.len]
    }
}

impl<'a, T: 'a> IntoIterator for StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            storage: self.storage,
            len: 0,
        }
    }
}

impl<'a, T: 'a> IntoIterator for &'a StackVec<'a, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            storage: self.storage,
            len: 0,
        }
    }
}

pub struct Iter<'a, T: 'a> {
    storage: &'a [T],
    len: usize,
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.len >= self.storage.len() {
            return None;
        } else {
            let x = &self.storage[self.len];
            self.len += 1;
            Some(x)
        }
    }
}
