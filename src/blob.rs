// Copyright 2022 The peniko authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// Also licensed under MIT license, at your choice.

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Weak};

/// Shared data with an associated unique identifier.
pub struct Blob<T> {
    data: Arc<dyn AsRef<[T]>>,
    id: u64,
}

impl<T> fmt::Debug for Blob<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Blob").field("id", &self.id).finish()
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

impl<T> Blob<T> {
    /// Creates a new blob from the given data and generates a unique
    /// identifier.
    pub fn new(data: Arc<dyn AsRef<[T]>>) -> Self {
        static ID_COUNTER: AtomicU64 = AtomicU64::new(0);
        Self {
            data,
            id: ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    /// Returns the length of the data.
    pub fn len(&self) -> usize {
        self.data().len()
    }

    /// Returns true if the blob is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a reference to the underlying data.
    pub fn data(&self) -> &[T] {
        self.data.as_ref().as_ref()
    }

    /// Returns the unique identifier associated with the data.
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Downgrades the shared blob to a weak reference.
    pub fn downgrade(&self) -> WeakBlob<T> {
        WeakBlob {
            data: Arc::downgrade(&self.data),
            id: self.id,
        }
    }
}

/// Weak reference to a shared blob.
pub struct WeakBlob<T> {
    data: Weak<dyn AsRef<[T]>>,
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
    pub fn id(&self) -> u64 {
        self.id
    }

    /// Upgrades the weak reference. Returns `None` if the inner value has been
    /// dropped.
    pub fn upgrade(&self) -> Option<Blob<T>> {
        Some(Blob {
            data: self.data.upgrade()?,
            id: self.id,
        })
    }
}
