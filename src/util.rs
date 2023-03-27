// SPDX-License-Identifier: LGPL-3.0-or-later OR MPL-2.0
// This file is a part of `ui-theme`.
//
// `ui-theme` is free software: you can redistribute it and/or modify it under the terms of
// either:
//
// * GNU Lesser General Public License as published by the Free Software Foundation, either
// version 3 of the License, or (at your option) any later version.
// * Mozilla Public License as published by the Mozilla Foundation, version 2.
//
// `ui-theme` is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
// without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
// See the GNU Lesser General Public License or the Mozilla Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License and the Mozilla
// Public License along with `ui-theme`. If not, see <https://www.gnu.org/licenses/> or
// <https://www.mozilla.org/en-US/MPL/2.0/>.

//! Defines a hash map using `ahash`, using `fastrand` to seed it.

use ahash::RandomState;
use core::hash::BuildHasher;
use hashbrown::HashMap as HashbrownHashMap;

pub(super) type HashMap<K, V> = HashbrownHashMap<K, V, PsuedoRandom>;

pub(super) trait HashMapExt {
    fn with_capacity(capacity: usize) -> Self;
}

impl<K, V> HashMapExt for HashMap<K, V> {
    fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_and_hasher(capacity, PsuedoRandom::default())
    }
}

#[derive(Clone)]
pub(super) struct PsuedoRandom(RandomState);

impl Default for PsuedoRandom {
    fn default() -> Self {
        #[cfg(feature = "std")]
        let rng = fastrand::Rng::new();

        #[cfg(not(feature = "std"))]
        let rng = fastrand::Rng::with_seed(0x4815162342);

        Self(RandomState::with_seeds(
            rng.u64(..),
            rng.u64(..),
            rng.u64(..),
            rng.u64(..),
        ))
    }
}

impl BuildHasher for PsuedoRandom {
    type Hasher = ahash::AHasher;

    fn build_hasher(&self) -> Self::Hasher {
        self.0.build_hasher()
    }
}
