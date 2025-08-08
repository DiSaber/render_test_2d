use std::{
    hash::{Hash, Hasher},
    marker::PhantomData,
};

/// Stores the dense storage index and generational index.
#[derive(Debug)]
pub struct DenseStorageIndex<T>(pub usize, pub u32, PhantomData<T>);

impl<T> DenseStorageIndex<T> {
    pub fn new(index: usize, generational_index: u32) -> Self {
        Self(index, generational_index, PhantomData)
    }
}

// Manual impl's needed: https://github.com/rust-lang/rust/issues/26925
impl<T> Clone for DenseStorageIndex<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for DenseStorageIndex<T> {}
impl<T> Eq for DenseStorageIndex<T> {}
impl<T> PartialEq for DenseStorageIndex<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}
impl<T> Hash for DenseStorageIndex<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
        state.write_u32(self.1);
    }
}

/// A generational index storage container.
#[derive(Debug, Clone)]
pub struct DenseStorage<T> {
    // (generation, value)
    storage: Vec<(u32, Option<T>)>,
    recycled_indices: Vec<usize>,
}

impl<T> DenseStorage<T> {
    /// Creates a new empty container.
    pub fn new() -> Self {
        Self {
            storage: Vec::new(),
            recycled_indices: Vec::new(),
        }
    }

    /// Pushes a new value into the container and returns a `DenseStorageIndex` that can be used to
    /// retrieve the value later.
    pub fn push(&mut self, value: T) -> DenseStorageIndex<T> {
        if let Some(i) = self.recycled_indices.pop() {
            let (generation, v) = &mut self.storage[i];
            *v = Some(value);
            DenseStorageIndex(i, *generation, PhantomData)
        } else {
            self.storage.push((0, Some(value)));
            DenseStorageIndex(self.storage.len() - 1, 0, PhantomData)
        }
    }

    /// Gets a value from the container with the given index or `None` if the value doesn't
    /// exist.
    pub fn get(&self, index: DenseStorageIndex<T>) -> Option<&T> {
        self.storage
            .get(index.0)
            .filter(|(generation, _)| *generation == index.1)
            .and_then(|(_, value)| value.as_ref())
    }

    /// Removes a value from the container with the given index and returns the value if it exists.
    pub fn remove(&mut self, index: DenseStorageIndex<T>) -> Option<T> {
        if let Some((generation, value)) = self.storage.get_mut(index.0) {
            if value.is_some() {
                *generation += 1;
                self.recycled_indices.push(index.0);
            }

            value.take()
        } else {
            None
        }
    }

    pub fn iter(&self) -> DenseStorageIter<'_, T> {
        self.into_iter()
    }
}

type DenseStorageIter<'a, T> = std::iter::FilterMap<
    std::iter::Enumerate<std::slice::Iter<'a, (u32, Option<T>)>>,
    fn((usize, &(u32, Option<T>))) -> Option<(DenseStorageIndex<T>, &T)>,
>;

type DenseStorageIntoIter<T> = std::iter::FilterMap<
    std::iter::Enumerate<std::vec::IntoIter<(u32, Option<T>)>>,
    fn((usize, (u32, Option<T>))) -> Option<(DenseStorageIndex<T>, T)>,
>;

impl<'a, T> IntoIterator for &'a DenseStorage<T> {
    type Item = (DenseStorageIndex<T>, &'a T);
    type IntoIter = DenseStorageIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage
            .iter()
            .enumerate()
            .filter_map(|(i, (generation, value))| {
                value
                    .as_ref()
                    .map(|value| (DenseStorageIndex::new(i, *generation), value))
            })
    }
}

impl<T> IntoIterator for DenseStorage<T> {
    type Item = (DenseStorageIndex<T>, T);
    type IntoIter = DenseStorageIntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.storage
            .into_iter()
            .enumerate()
            .filter_map(|(i, (generation, value))| {
                value.map(|value| (DenseStorageIndex::new(i, generation), value))
            })
    }
}
