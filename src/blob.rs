// Copyright 2022 The peniko authors.
// SPDX-License-Identifier: Apache-2.0 OR MIT

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

/// Shared data with an associated unique identifier.
pub struct Blob<T> {
    data: Arc<dyn AsRef<[T]> + Send + Sync>,
    id: u64,
}

impl<T> fmt::Debug for Blob<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Blob")
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<T> PartialEq for Blob<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Clone for Blob<T> {
    fn clone(&self) -> Self {
        Self {
            data: Arc::clone(&self.data),
            id: self.id,
        }
    }
}

impl<T> AsRef<[T]> for Blob<T> {
    fn as_ref(&self) -> &[T] {
        self.data()
    }
}

impl<T> From<Vec<T>> for Blob<T>
where
    T: 'static + Send + Sync,
{
    fn from(vec: Vec<T>) -> Self {
        let boxed: Box<[T]> = vec.into();
        Self::new(Arc::new(boxed))
    }
}

static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

impl<T> Blob<T> {
    /// Creates a new blob from the given data and generates a unique
    /// identifier.
    pub fn new(data: Arc<dyn AsRef<[T]> + Send + Sync>) -> Self {
        Self {
            data,
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Creates a new blob from the given data and identifier.
    ///
    /// Note that while this function is not unsafe, usage of this in combination
    /// with `new` (or with identifiers that are not uniquely associated with the given data)
    /// can lead to inconsistencies.
    ///
    /// This is primarily for libraries that wish to interop with vello but are
    /// unable to depend on our resource types.
    pub fn from_raw_parts(data: Arc<dyn AsRef<[T]> + Send + Sync>, id: u64) -> Self {
        Self { data, id }
    }

    /// Consumes self and returns the inner components of the blob.
    #[must_use]
    pub fn into_raw_parts(self) -> (Arc<dyn AsRef<[T]> + Send + Sync>, u64) {
        (self.data, self.id)
    }

    /// Returns the length of the data.
    #[must_use]
    pub fn len(&self) -> usize {
        self.data().len()
    }

    /// Returns true if the blob is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the underlying data.
    #[must_use]
    pub fn data(&self) -> &[T] {
        self.data.as_ref().as_ref()
    }

    /// Returns the unique identifier associated with the data.
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Downgrades the shared blob to a weak reference.
    #[must_use]
    pub fn downgrade(&self) -> WeakBlob<T> {
        WeakBlob {
            data: Arc::downgrade(&self.data),
            id: self.id,
        }
    }
}

/// Weak reference to a shared [blob](Blob).
pub struct WeakBlob<T> {
    data: Weak<dyn AsRef<[T]> + Send + Sync>,
    id: u64,
}

impl<T> Clone for WeakBlob<T> {
    fn clone(&self) -> Self {
        Self {
            data: Weak::clone(&self.data),
            id: self.id,
        }
    }
}

impl<T> WeakBlob<T> {
    /// Returns the unique identifier associated with the data.
    #[must_use]
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Upgrades the weak reference. Returns `None` if the inner value has been
    /// dropped.
    #[must_use]
    pub fn upgrade(&self) -> Option<Blob<T>> {
        Some(Blob {
            data: self.data.upgrade()?,
            id: self.id,
        })
    }
}
