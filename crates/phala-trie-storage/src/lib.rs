#![no_std]

extern crate alloc;

#[cfg(feature = "serde")]
pub mod ser;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use core::iter::FromIterator;

use alloc::vec::Vec;

use parity_scale_codec::Codec;
use sp_core::storage::ChildInfo;
use sp_core::Hasher;
use sp_state_machine::{Backend, TrieBackend};
use sp_trie::{trie_types::TrieDBMutV0 as TrieDBMut, MemoryDB, TrieMut};

use sp_trie::HashDBT as _;

/// Storage key.
pub type StorageKey = Vec<u8>;

/// Storage value.
pub type StorageValue = Vec<u8>;

/// In memory array of storage values.
pub type StorageCollection = Vec<(StorageKey, Option<StorageValue>)>;

/// In memory arrays of storage values for multiple child tries.
pub type ChildStorageCollection = Vec<(StorageKey, StorageCollection)>;

pub struct TrieStorage<H: Hasher>(TrieBackend<MemoryDB<H>, H>);

impl<H: Hasher> Default for TrieStorage<H>
where
    H::Out: Codec,
{
    fn default() -> Self {
        Self(TrieBackend::new(Default::default(), Default::default()))
    }
}

pub fn load_trie_backend<H: Hasher>(
    pairs: impl Iterator<Item = (impl AsRef<[u8]>, impl AsRef<[u8]>)>,
) -> TrieBackend<MemoryDB<H>, H>
where
    H::Out: Codec,
{
    let mut root = Default::default();
    let mut mdb = Default::default();
    {
        let mut trie_db = TrieDBMut::new(&mut mdb, &mut root);
        for (key, value) in pairs {
            if trie_db.insert(key.as_ref(), value.as_ref()).is_err() {
                panic!("Insert item into trie DB should not fail");
            }
        }
    }
    TrieBackend::new(mdb, root)
}

#[cfg(feature = "serde")]
pub fn serialize_trie_backend<H: Hasher, S>(
    trie: &TrieBackend<MemoryDB<H>, H>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    H::Out: Codec + Serialize,
    S: Serializer,
{
    let root = trie.root();
    let kvs: Vec<_> = trie
        .backend_storage()
        .clone()
        .drain()
        .into_iter()
        .map(|it| it.1)
        .collect();
    (root, kvs).serialize(serializer)
}

#[cfg(feature = "serde")]
pub fn deserialize_trie_backend<'de, H: Hasher, De>(
    deserializer: De,
) -> Result<TrieBackend<MemoryDB<H>, H>, De::Error>
where
    H::Out: Codec + Deserialize<'de>,
    De: Deserializer<'de>,
{
    let (root, kvs): (H::Out, Vec<(Vec<u8>, i32)>) = Deserialize::deserialize(deserializer)?;
    let mut mdb = MemoryDB::default();
    for value in kvs {
        for _ in 0..value.1 {
            mdb.insert((&[], None), &value.0);
        }
    }
    let backend = TrieBackend::new(mdb, root);
    Ok(backend)
}

pub fn clone_trie_backend<H: Hasher>(
    trie: &TrieBackend<MemoryDB<H>, H>,
) -> TrieBackend<MemoryDB<H>, H>
where
    H::Out: Codec,
{
    let root = trie.root();
    let kvs: Vec<_> = trie
        .backend_storage()
        .clone()
        .drain()
        .into_iter()
        .map(|it| it.1)
        .collect();
    let mut mdb = MemoryDB::default();
    for value in kvs {
        for _ in 0..value.1 {
            mdb.insert((&[], None), &value.0);
        }
    }
    TrieBackend::new(mdb, *root)
}

impl<H: Hasher> TrieStorage<H>
where
    H::Out: Codec + Ord,
{
    /// Overwrite all data in the trie DB with given key/value pairs.
    pub fn load(&mut self, pairs: impl Iterator<Item = (impl AsRef<[u8]>, impl AsRef<[u8]>)>) {
        let trie = load_trie_backend(pairs);
        let _ = core::mem::replace(&mut self.0, trie);
    }

    /// Calculate the new state root given storage changes. Returns the new root and a transaction to apply.
    #[allow(clippy::ptr_arg)]
    pub fn calc_root_if_changes<'a>(
        &self,
        delta: &'a StorageCollection,
        child_deltas: &'a ChildStorageCollection,
    ) -> (H::Out, MemoryDB<H>) {
        let child_deltas: Vec<(ChildInfo, &StorageCollection)> = child_deltas
            .iter()
            .map(|(k, v)| {
                let chinfo = ChildInfo::new_default(k);
                (chinfo, v)
            })
            .collect();
        self.0.full_storage_root(
            delta
                .iter()
                .map(|(k, v)| (k.as_ref(), v.as_ref().map(|v| v.as_ref()))),
            child_deltas.iter().map(|(k, v)| {
                (
                    k,
                    v.iter()
                        .map(|(k, v)| (k.as_ref(), v.as_ref().map(|v| v.as_ref()))),
                )
            }),
            sp_core::storage::StateVersion::V0,
        )
    }

    /// Apply storage changes calculated from `calc_root_if_changes`.
    pub fn apply_changes(&mut self, root: H::Out, transaction: MemoryDB<H>) {
        let mut storage = core::mem::take(self).0.into_storage();
        storage.consolidate(transaction);
        storage.purge();
        let _ = core::mem::replace(&mut self.0, TrieBackend::new(storage, root));
    }

    /// Return the state root hash
    pub fn root(&self) -> &H::Out {
        self.0.root()
    }

    /// Given storage key return storage value
    pub fn get(&self, key: impl AsRef<[u8]>) -> Option<Vec<u8>> {
        self.0.storage(key.as_ref()).ok().flatten()
    }

    /// Return storage pairs which start with given storage key prefix
    pub fn pairs(&self, prefix: impl AsRef<[u8]>) -> Vec<(Vec<u8>, Vec<u8>)> {
        self.pairs_into(prefix)
    }

    fn pairs_into<R: FromIterator<(Vec<u8>, Vec<u8>)>>(&self, prefix: impl AsRef<[u8]>) -> R {
        self.0
            .keys(prefix.as_ref())
            .into_iter()
            .map(|key| {
                let value = self.get(&key).expect("Reflected key should exists");
                (key, value)
            })
            .collect()
    }
}

#[cfg(feature = "serde")]
const _: () = {
    impl<H: Hasher> Serialize for TrieStorage<H>
    where
        H::Out: Codec + Serialize + Ord,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serialize_trie_backend(&self.0, serializer)
        }
    }

    impl<'de, H: Hasher> Deserialize<'de> for TrieStorage<H>
    where
        H::Out: Codec + Deserialize<'de> + Ord,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            Ok(Self(deserialize_trie_backend(deserializer)?))
        }
    }
};
