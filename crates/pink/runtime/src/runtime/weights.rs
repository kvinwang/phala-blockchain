#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

use pallet_contracts::weights::WeightInfo;

pub struct PinkWeights<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for PinkWeights<T> {
    /// Storage: Contracts DeletionQueueCounter (r:1 w:0)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    fn on_process_deletion_queue_batch() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `142`
        //  Estimated: `1627`
        // Minimum execution time: 2_519_000 picoseconds.
        Weight::from_parts(2_660_000, 1627)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `k` is `[0, 1024]`.
    fn on_initialize_per_trie_key(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `451 + k * (69 ±0)`
        //  Estimated: `441 + k * (70 ±0)`
        // Minimum execution time: 13_096_000 picoseconds.
        Weight::from_parts(13_395_000, 441)
            // Standard Error: 1_046
            .saturating_add(Weight::from_parts(1_246_238, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(k.into())))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_parts(0, 70).saturating_mul(k.into()))
    }
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:2 w:1)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:2 w:1)
    /// The range of component `c` is `[0, 125952]`.
    fn v9_migration_step(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `211 + c * (1 ±0)`
        //  Estimated: `6149 + c * (1 ±0)`
        // Minimum execution time: 8_409_000 picoseconds.
        Weight::from_parts(9_006_438, 6149)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_345, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts ContractInfoOf (r:2 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    fn v10_migration_step() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `510`
        //  Estimated: `6450`
        // Minimum execution time: 16_962_000 picoseconds.
        Weight::from_parts(17_716_000, 6450)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: Contracts DeletionQueue (r:1 w:1025)
    /// Proof: Contracts DeletionQueue (max_values: None, max_size: Some(142), added: 2617, mode: Measured)
    /// Storage: Contracts DeletionQueueCounter (r:0 w:1)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// The range of component `k` is `[0, 1024]`.
    fn v11_migration_step(k: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `171 + k * (1 ±0)`
        //  Estimated: `3635 + k * (1 ±0)`
        // Minimum execution time: 3_763_000 picoseconds.
        Weight::from_parts(2_401_625, 3635)
            // Standard Error: 2_827
            .saturating_add(Weight::from_parts(1_201_671, 0).saturating_mul(k.into()))
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(k.into())))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(k.into()))
    }
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc553053f13fd319a03c211337c76e0fe776df` (r:2 w:0)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc553053f13fd319a03c211337c76e0fe776df` (r:2 w:0)
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:1 w:1)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc553022fca90611ba8b7942f8bdb3b97f6580` (r:1 w:1)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:0 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn v12_migration_step(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `325 + c * (1 ±0)`
        //  Estimated: `6263 + c * (1 ±0)`
        // Minimum execution time: 17_490_000 picoseconds.
        Weight::from_parts(17_712_278, 6263)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(427, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(4_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:1)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    fn migration_noop() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `142`
        //  Estimated: `1627`
        // Minimum execution time: 3_282_000 picoseconds.
        Weight::from_parts(3_536_000, 1627)
            .saturating_add(T::DbWeight::get().reads(1_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:1)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:1)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:1)
    fn migrate() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `166`
        //  Estimated: `3631`
        // Minimum execution time: 12_973_000 picoseconds.
        Weight::from_parts(13_366_000, 3631)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(2_u64))
    }
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    fn on_runtime_upgrade_noop() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `142`
        //  Estimated: `3607`
        // Minimum execution time: 4_764_000 picoseconds.
        Weight::from_parts(5_000_000, 3607)
            .saturating_add(T::DbWeight::get().reads(1_u64))
    }
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    fn on_runtime_upgrade_in_progress() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `167`
        //  Estimated: `3632`
        // Minimum execution time: 6_616_000 picoseconds.
        Weight::from_parts(6_935_000, 3632)
            .saturating_add(T::DbWeight::get().reads(2_u64))
    }
    /// Storage: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    /// Proof Skipped: unknown `0x4342193e496fab7ec59d615ed0dc55304e7b9012096b41c4eb3aaf947f6ea429` (r:1 w:0)
    /// Storage: Contracts MigrationInProgress (r:1 w:1)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    fn on_runtime_upgrade() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `142`
        //  Estimated: `3607`
        // Minimum execution time: 6_953_000 picoseconds.
        Weight::from_parts(7_440_000, 3607)
            .saturating_add(T::DbWeight::get().reads(2_u64))
            .saturating_add(T::DbWeight::get().writes(1_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn call_with_code_per_byte(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `786`
        //  Estimated: `6735 + c * (1 ±0)`
        // Minimum execution time: 302_714_000 picoseconds.
        Weight::from_parts(271_320_595, 6735)
            // Standard Error: 72
            .saturating_add(Weight::from_parts(38_474, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(c.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate_with_code(c: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `303`
        //  Estimated: `8745`
        // Minimum execution time: 4_506_957_000 picoseconds.
        Weight::from_parts(643_316_921, 8745)
            // Standard Error: 278
            .saturating_add(Weight::from_parts(112_835, 0).saturating_mul(c.into()))
            // Standard Error: 33
            .saturating_add(Weight::from_parts(1_830, 0).saturating_mul(i.into()))
            // Standard Error: 33
            .saturating_add(Weight::from_parts(2_022, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().writes(9_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:2 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    /// The range of component `s` is `[0, 1048576]`.
    fn instantiate(i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `523`
        //  Estimated: `6513`
        // Minimum execution time: 2_103_482_000 picoseconds.
        Weight::from_parts(316_666_183, 6513)
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_933, 0).saturating_mul(i.into()))
            // Standard Error: 7
            .saturating_add(Weight::from_parts(1_803, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().writes(7_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System Account (r:1 w:1)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn call() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `820`
        //  Estimated: `6760`
        // Minimum execution time: 207_530_000 picoseconds.
        Weight::from_parts(217_243_000, 6760)
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// The range of component `c` is `[0, 125952]`.
    fn upload_code(c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `142`
        //  Estimated: `3607`
        // Minimum execution time: 246_381_000 picoseconds.
        Weight::from_parts(242_933_576, 3607)
            // Standard Error: 100
            .saturating_add(Weight::from_parts(74_645, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: System EventTopics (r:1 w:1)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts PristineCode (r:0 w:1)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    fn remove_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `255`
        //  Estimated: `3720`
        // Minimum execution time: 35_519_000 picoseconds.
        Weight::from_parts(36_813_000, 3720)
            .saturating_add(T::DbWeight::get().reads(3_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:2 w:2)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    fn set_code() -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `575`
        //  Estimated: `8990`
        // Minimum execution time: 37_769_000 picoseconds.
        Weight::from_parts(39_349_000, 8990)
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `860 + r * (6 ±0)`
        //  Estimated: `6801 + r * (6 ±0)`
        // Minimum execution time: 273_355_000 picoseconds.
        Weight::from_parts(280_115_308, 6801)
            // Standard Error: 662
            .saturating_add(Weight::from_parts(351_066, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_is_contract(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `918 + r * (240 ±0)`
        //  Estimated: `6822 + r * (2715 ±0)`
        // Minimum execution time: 264_066_000 picoseconds.
        Weight::from_parts(103_474_597, 6822)
            // Standard Error: 7_010
            .saturating_add(Weight::from_parts(3_917_988, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2715).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1601 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `910 + r * (244 ±0)`
        //  Estimated: `6826 + r * (2719 ±0)`
        // Minimum execution time: 275_726_000 picoseconds.
        Weight::from_parts(111_512_451, 6826)
            // Standard Error: 6_673
            .saturating_add(Weight::from_parts(4_626_511, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 2719).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_own_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `867 + r * (6 ±0)`
        //  Estimated: `6809 + r * (6 ±0)`
        // Minimum execution time: 274_377_000 picoseconds.
        Weight::from_parts(286_299_699, 6809)
            // Standard Error: 521
            .saturating_add(Weight::from_parts(419_417, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller_is_origin(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `857 + r * (3 ±0)`
        //  Estimated: `6802 + r * (3 ±0)`
        // Minimum execution time: 265_297_000 picoseconds.
        Weight::from_parts(283_474_927, 6802)
            // Standard Error: 376
            .saturating_add(Weight::from_parts(186_214, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_caller_is_root(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `747 + r * (3 ±0)`
        //  Estimated: `6687 + r * (3 ±0)`
        // Minimum execution time: 258_385_000 picoseconds.
        Weight::from_parts(269_869_790, 6687)
            // Standard Error: 334
            .saturating_add(Weight::from_parts(164_806, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(7_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `861 + r * (6 ±0)`
        //  Estimated: `6803 + r * (6 ±0)`
        // Minimum execution time: 271_351_000 picoseconds.
        Weight::from_parts(286_390_305, 6803)
            // Standard Error: 628
            .saturating_add(Weight::from_parts(339_374, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_gas_left(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `857 + r * (6 ±0)`
        //  Estimated: `6798 + r * (6 ±0)`
        // Minimum execution time: 273_060_000 picoseconds.
        Weight::from_parts(285_959_049, 6798)
            // Standard Error: 813
            .saturating_add(Weight::from_parts(544_941, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:2 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1001 + r * (6 ±0)`
        //  Estimated: `6925 + r * (6 ±0)`
        // Minimum execution time: 273_717_000 picoseconds.
        Weight::from_parts(301_053_119, 6925)
            // Standard Error: 3_314
            .saturating_add(Weight::from_parts(1_645_480, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_value_transferred(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `871 + r * (6 ±0)`
        //  Estimated: `6820 + r * (6 ±0)`
        // Minimum execution time: 273_480_000 picoseconds.
        Weight::from_parts(284_751_212, 6820)
            // Standard Error: 501
            .saturating_add(Weight::from_parts(334_063, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_minimum_balance(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `869 + r * (6 ±0)`
        //  Estimated: `6818 + r * (6 ±0)`
        // Minimum execution time: 278_938_000 picoseconds.
        Weight::from_parts(284_829_302, 6818)
            // Standard Error: 488
            .saturating_add(Weight::from_parts(338_782, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_block_number(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `866 + r * (6 ±0)`
        //  Estimated: `6816 + r * (6 ±0)`
        // Minimum execution time: 276_799_000 picoseconds.
        Weight::from_parts(290_353_700, 6816)
            // Standard Error: 675
            .saturating_add(Weight::from_parts(323_565, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_now(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `857 + r * (6 ±0)`
        //  Estimated: `6802 + r * (6 ±0)`
        // Minimum execution time: 267_740_000 picoseconds.
        Weight::from_parts(287_560_339, 6802)
            // Standard Error: 479
            .saturating_add(Weight::from_parts(329_276, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: TransactionPayment NextFeeMultiplier (r:1 w:0)
    /// Proof: TransactionPayment NextFeeMultiplier (max_values: Some(1), max_size: Some(16), added: 511, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_weight_to_fee(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `931 + r * (14 ±0)`
        //  Estimated: `6864 + r * (14 ±0)`
        // Minimum execution time: 275_471_000 picoseconds.
        Weight::from_parts(297_332_107, 6864)
            // Standard Error: 2_230
            .saturating_add(Weight::from_parts(1_484_476, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 14).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_input(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `859 + r * (6 ±0)`
        //  Estimated: `6803 + r * (6 ±0)`
        // Minimum execution time: 255_279_000 picoseconds.
        Weight::from_parts(282_649_020, 6803)
            // Standard Error: 429
            .saturating_add(Weight::from_parts(290_527, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 6).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_input_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `863`
        //  Estimated: `6803`
        // Minimum execution time: 268_029_000 picoseconds.
        Weight::from_parts(231_474_232, 6803)
            // Standard Error: 23
            .saturating_add(Weight::from_parts(1_050, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_return(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `847 + r * (45 ±0)`
        //  Estimated: `6787 + r * (45 ±0)`
        // Minimum execution time: 252_126_000 picoseconds.
        Weight::from_parts(277_677_710, 6787)
            // Standard Error: 770_704
            .saturating_add(Weight::from_parts(2_678_989, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 45).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_return_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `857`
        //  Estimated: `6810`
        // Minimum execution time: 271_967_000 picoseconds.
        Weight::from_parts(282_988_484, 6810)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(387, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts DeletionQueueCounter (r:1 w:1)
    /// Proof: Contracts DeletionQueueCounter (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// Storage: Contracts DeletionQueue (r:0 w:1)
    /// Proof: Contracts DeletionQueue (max_values: None, max_size: Some(142), added: 2617, mode: Measured)
    /// The range of component `r` is `[0, 1]`.
    fn seal_terminate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `889 + r * (300 ±0)`
        //  Estimated: `6829 + r * (7725 ±0)`
        // Minimum execution time: 257_246_000 picoseconds.
        Weight::from_parts(280_196_561, 6829)
            // Standard Error: 815_845
            .saturating_add(Weight::from_parts(127_831_338, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((5_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((8_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 7725).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: RandomnessCollectiveFlip RandomMaterial (r:1 w:0)
    /// Proof: RandomnessCollectiveFlip RandomMaterial (max_values: Some(1), max_size: Some(2594), added: 3089, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_random(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `938 + r * (10 ±0)`
        //  Estimated: `6879 + r * (10 ±0)`
        // Minimum execution time: 270_074_000 picoseconds.
        Weight::from_parts(292_298_331, 6879)
            // Standard Error: 2_123
            .saturating_add(Weight::from_parts(2_089_487, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_deposit_event(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `857 + r * (10 ±0)`
        //  Estimated: `6802 + r * (10 ±0)`
        // Minimum execution time: 267_080_000 picoseconds.
        Weight::from_parts(298_470_496, 6802)
            // Standard Error: 3_004
            .saturating_add(Weight::from_parts(3_898_460, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 10).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:6 w:6)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 4]`.
    /// The range of component `n` is `[0, 16384]`.
    fn seal_deposit_event_per_topic_and_byte(t: u32, n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `876 + t * (32 ±0)`
        //  Estimated: `6823 + t * (2508 ±0)`
        // Minimum execution time: 277_152_000 picoseconds.
        Weight::from_parts(290_745_178, 6823)
            // Standard Error: 88_577
            .saturating_add(Weight::from_parts(2_476_405, 0).saturating_mul(t.into()))
            // Standard Error: 24
            .saturating_add(Weight::from_parts(702, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2508).saturating_mul(t.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_debug_message(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `856 + r * (7 ±0)`
        //  Estimated: `6800 + r * (7 ±0)`
        // Minimum execution time: 168_782_000 picoseconds.
        Weight::from_parts(179_694_331, 6800)
            // Standard Error: 338
            .saturating_add(Weight::from_parts(246_541, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 7).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: MaxEncodedLen)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: MaxEncodedLen)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: MaxEncodedLen)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: MaxEncodedLen)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `i` is `[0, 1048576]`.
    fn seal_debug_message_per_byte(i: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `125807`
        //  Estimated: `131749`
        // Minimum execution time: 428_673_000 picoseconds.
        Weight::from_parts(398_928_494, 131749)
            // Standard Error: 12
            .saturating_add(Weight::from_parts(1_106, 0).saturating_mul(i.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_set_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `924 + r * (292 ±0)`
        //  Estimated: `922 + r * (293 ±0)`
        // Minimum execution time: 271_384_000 picoseconds.
        Weight::from_parts(147_677_611, 922)
            // Standard Error: 13_371
            .saturating_add(Weight::from_parts(7_085_478, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 293).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_new_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1383`
        //  Estimated: `1359`
        // Minimum execution time: 279_587_000 picoseconds.
        Weight::from_parts(335_690_918, 1359)
            // Standard Error: 57
            .saturating_add(Weight::from_parts(708, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().writes(6_u64))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_set_storage_per_old_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1246 + n * (1 ±0)`
        //  Estimated: `1246 + n * (1 ±0)`
        // Minimum execution time: 275_572_000 picoseconds.
        Weight::from_parts(300_309_544, 1246)
            // Standard Error: 35
            .saturating_add(Weight::from_parts(299, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_clear_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `920 + r * (288 ±0)`
        //  Estimated: `924 + r * (289 ±0)`
        // Minimum execution time: 271_875_000 picoseconds.
        Weight::from_parts(153_680_437, 924)
            // Standard Error: 13_050
            .saturating_add(Weight::from_parts(6_892_925, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_clear_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1242 + n * (1 ±0)`
        //  Estimated: `1242 + n * (1 ±0)`
        // Minimum execution time: 272_682_000 picoseconds.
        Weight::from_parts(301_025_128, 1242)
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_get_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `914 + r * (296 ±0)`
        //  Estimated: `919 + r * (297 ±0)`
        // Minimum execution time: 271_796_000 picoseconds.
        Weight::from_parts(183_856_480, 919)
            // Standard Error: 10_064
            .saturating_add(Weight::from_parts(5_660_636, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_get_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1258 + n * (1 ±0)`
        //  Estimated: `1258 + n * (1 ±0)`
        // Minimum execution time: 273_102_000 picoseconds.
        Weight::from_parts(297_455_692, 1258)
            // Standard Error: 35
            .saturating_add(Weight::from_parts(868, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_contains_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `935 + r * (288 ±0)`
        //  Estimated: `936 + r * (289 ±0)`
        // Minimum execution time: 271_323_000 picoseconds.
        Weight::from_parts(190_080_834, 936)
            // Standard Error: 9_143
            .saturating_add(Weight::from_parts(5_488_362, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 289).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_contains_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1245 + n * (1 ±0)`
        //  Estimated: `1245 + n * (1 ±0)`
        // Minimum execution time: 270_399_000 picoseconds.
        Weight::from_parts(296_679_410, 1245)
            // Standard Error: 34
            .saturating_add(Weight::from_parts(161, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_take_storage(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `908 + r * (296 ±0)`
        //  Estimated: `915 + r * (297 ±0)`
        // Minimum execution time: 271_645_000 picoseconds.
        Weight::from_parts(147_320_521, 915)
            // Standard Error: 13_502
            .saturating_add(Weight::from_parts(7_074_778, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 297).saturating_mul(r.into()))
    }
    /// Storage: Skipped Metadata (r:0 w:0)
    /// Proof Skipped: Skipped Metadata (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 16384]`.
    fn seal_take_storage_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1259 + n * (1 ±0)`
        //  Estimated: `1259 + n * (1 ±0)`
        // Minimum execution time: 280_680_000 picoseconds.
        Weight::from_parts(304_043_474, 1259)
            // Standard Error: 29
            .saturating_add(Weight::from_parts(644, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1602 w:1601)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_transfer(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1452 + r * (45 ±0)`
        //  Estimated: `7349 + r * (2520 ±0)`
        // Minimum execution time: 274_928_000 picoseconds.
        Weight::from_parts(192_111_339, 7349)
            // Standard Error: 42_436
            .saturating_add(Weight::from_parts(40_323_660, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2520).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:2 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:2 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:803 w:803)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1296 + r * (276 ±0)`
        //  Estimated: `9481 + r * (2752 ±0)`
        // Minimum execution time: 275_293_000 picoseconds.
        Weight::from_parts(278_243_000, 9481)
            // Standard Error: 119_869
            .saturating_add(Weight::from_parts(245_114_905, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(11_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2752).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:736 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:736 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:737 w:737)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 800]`.
    fn seal_delegate_call(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (572 ±0)`
        //  Estimated: `6806 + r * (2633 ±3)`
        // Minimum execution time: 271_857_000 picoseconds.
        Weight::from_parts(278_276_000, 6806)
            // Standard Error: 152_056
            .saturating_add(Weight::from_parts(243_744_830, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 2633).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:3 w:2)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:2 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:2 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:4 w:4)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `c` is `[0, 1048576]`.
    fn seal_call_per_transfer_clone_byte(t: u32, c: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1328 + t * (310 ±0)`
        //  Estimated: `12218 + t * (5260 ±0)`
        // Minimum execution time: 463_865_000 picoseconds.
        Weight::from_parts(70_396_050, 12218)
            // Standard Error: 11_489_598
            .saturating_add(Weight::from_parts(359_195_747, 0).saturating_mul(t.into()))
            // Standard Error: 16
            .saturating_add(Weight::from_parts(1_090, 0).saturating_mul(c.into()))
            .saturating_add(T::DbWeight::get().reads(13_u64))
            .saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(6_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 5260).saturating_mul(t.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1602 w:1602)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:801 w:801)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:801 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:801 w:800)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:802 w:802)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[1, 800]`.
    fn seal_instantiate(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1383 + r * (251 ±0)`
        //  Estimated: `7207 + r * (5202 ±0)`
        // Minimum execution time: 660_947_000 picoseconds.
        Weight::from_parts(668_346_000, 7207)
            // Standard Error: 357_950
            .saturating_add(Weight::from_parts(397_202_020, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(10_u64))
            .saturating_add(T::DbWeight::get().reads((6_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(6_u64))
            .saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 5202).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:4 w:4)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:2 w:2)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:2 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:2 w:1)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:3 w:3)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `t` is `[0, 1]`.
    /// The range of component `i` is `[0, 983040]`.
    /// The range of component `s` is `[0, 983040]`.
    fn seal_instantiate_per_transfer_input_salt_byte(t: u32, i: u32, s: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `1232 + t * (156 ±0)`
        //  Estimated: `9662 + t * (2578 ±2)`
        // Minimum execution time: 2_419_720_000 picoseconds.
        Weight::from_parts(1_328_224_119, 9662)
            // Standard Error: 17
            .saturating_add(Weight::from_parts(1_171, 0).saturating_mul(i.into()))
            // Standard Error: 17
            .saturating_add(Weight::from_parts(1_263, 0).saturating_mul(s.into()))
            .saturating_add(T::DbWeight::get().reads(15_u64))
            .saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(t.into())))
            .saturating_add(T::DbWeight::get().writes(10_u64))
            .saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(t.into())))
            .saturating_add(Weight::from_parts(0, 2578).saturating_mul(t.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_sha2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `856 + r * (8 ±0)`
        //  Estimated: `6797 + r * (8 ±0)`
        // Minimum execution time: 263_620_000 picoseconds.
        Weight::from_parts(285_686_431, 6797)
            // Standard Error: 605
            .saturating_add(Weight::from_parts(393_863, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_sha2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `864`
        //  Estimated: `6804`
        // Minimum execution time: 271_378_000 picoseconds.
        Weight::from_parts(266_737_832, 6804)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(1_124, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_keccak_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `858 + r * (8 ±0)`
        //  Estimated: `6800 + r * (8 ±0)`
        // Minimum execution time: 269_277_000 picoseconds.
        Weight::from_parts(282_723_951, 6800)
            // Standard Error: 577
            .saturating_add(Weight::from_parts(808_522, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_keccak_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `866`
        //  Estimated: `6808`
        // Minimum execution time: 254_252_000 picoseconds.
        Weight::from_parts(277_589_498, 6808)
            // Standard Error: 1
            .saturating_add(Weight::from_parts(3_394, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_256(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `858 + r * (8 ±0)`
        //  Estimated: `6803 + r * (8 ±0)`
        // Minimum execution time: 254_411_000 picoseconds.
        Weight::from_parts(283_572_987, 6803)
            // Standard Error: 549
            .saturating_add(Weight::from_parts(455_436, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_256_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `866`
        //  Estimated: `6812`
        // Minimum execution time: 264_371_000 picoseconds.
        Weight::from_parts(269_330_603, 6812)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(1_249, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_hash_blake2_128(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `858 + r * (8 ±0)`
        //  Estimated: `6804 + r * (8 ±0)`
        // Minimum execution time: 257_896_000 picoseconds.
        Weight::from_parts(286_738_151, 6804)
            // Standard Error: 680
            .saturating_add(Weight::from_parts(459_525, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 8).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 1048576]`.
    fn seal_hash_blake2_128_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `866`
        //  Estimated: `6806`
        // Minimum execution time: 272_952_000 picoseconds.
        Weight::from_parts(271_516_361, 6806)
            // Standard Error: 0
            .saturating_add(Weight::from_parts(1_242, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `n` is `[0, 125697]`.
    fn seal_sr25519_verify_per_byte(n: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `991 + n * (1 ±0)`
        //  Estimated: `6928 + n * (1 ±0)`
        // Minimum execution time: 351_363_000 picoseconds.
        Weight::from_parts(356_558_856, 6928)
            // Standard Error: 10
            .saturating_add(Weight::from_parts(6_085, 0).saturating_mul(n.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 1).saturating_mul(n.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_sr25519_verify(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `801 + r * (112 ±0)`
        //  Estimated: `6745 + r * (112 ±0)`
        // Minimum execution time: 261_688_000 picoseconds.
        Weight::from_parts(338_043_015, 6745)
            // Standard Error: 13_532
            .saturating_add(Weight::from_parts(56_420_806, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 112).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_recover(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `901 + r * (76 ±0)`
        //  Estimated: `6795 + r * (77 ±0)`
        // Minimum execution time: 267_401_000 picoseconds.
        Weight::from_parts(345_773_771, 6795)
            // Standard Error: 14_486
            .saturating_add(Weight::from_parts(46_180_739, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 77).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 160]`.
    fn seal_ecdsa_to_eth_address(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `871 + r * (42 ±0)`
        //  Estimated: `6810 + r * (42 ±0)`
        // Minimum execution time: 277_890_000 picoseconds.
        Weight::from_parts(319_211_194, 6810)
            // Standard Error: 9_132
            .saturating_add(Weight::from_parts(12_128_696, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 42).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1536 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1536 w:1536)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:1538 w:1538)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_set_code_hash(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0 + r * (961 ±0)`
        //  Estimated: `6801 + r * (3087 ±10)`
        // Minimum execution time: 259_692_000 picoseconds.
        Weight::from_parts(278_327_000, 6801)
            // Standard Error: 60_024
            .saturating_add(Weight::from_parts(25_758_805, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(r.into())))
            .saturating_add(Weight::from_parts(0, 3087).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `852 + r * (3 ±0)`
        //  Estimated: `6802 + r * (3 ±0)`
        // Minimum execution time: 258_907_000 picoseconds.
        Weight::from_parts(285_755_890, 6802)
            // Standard Error: 378
            .saturating_add(Weight::from_parts(179_649, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_account_reentrance_count(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `2092 + r * (39 ±0)`
        //  Estimated: `7919 + r * (40 ±0)`
        // Minimum execution time: 260_415_000 picoseconds.
        Weight::from_parts(363_871_048, 7919)
            // Standard Error: 2_010
            .saturating_add(Weight::from_parts(317_607, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(8_u64))
            .saturating_add(T::DbWeight::get().writes(3_u64))
            .saturating_add(Weight::from_parts(0, 40).saturating_mul(r.into()))
    }
    /// Storage: Contracts MigrationInProgress (r:1 w:0)
    /// Proof: Contracts MigrationInProgress (max_values: Some(1), max_size: Some(1026), added: 1521, mode: Measured)
    /// Storage: System Account (r:1 w:0)
    /// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: Measured)
    /// Storage: Contracts ContractInfoOf (r:1 w:1)
    /// Proof: Contracts ContractInfoOf (max_values: None, max_size: Some(290), added: 2765, mode: Measured)
    /// Storage: Contracts PristineCode (r:1 w:0)
    /// Proof: Contracts PristineCode (max_values: None, max_size: Some(125988), added: 128463, mode: Measured)
    /// Storage: Contracts CodeInfoOf (r:1 w:0)
    /// Proof: Contracts CodeInfoOf (max_values: None, max_size: Some(89), added: 2564, mode: Measured)
    /// Storage: Timestamp Now (r:1 w:0)
    /// Proof: Timestamp Now (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: Contracts Nonce (r:1 w:1)
    /// Proof: Contracts Nonce (max_values: Some(1), max_size: Some(8), added: 503, mode: Measured)
    /// Storage: System EventTopics (r:2 w:2)
    /// Proof Skipped: System EventTopics (max_values: None, max_size: None, mode: Measured)
    /// The range of component `r` is `[0, 1600]`.
    fn seal_instantiation_nonce(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `855 + r * (3 ±0)`
        //  Estimated: `6802 + r * (3 ±0)`
        // Minimum execution time: 257_725_000 picoseconds.
        Weight::from_parts(283_441_372, 6802)
            // Standard Error: 371
            .saturating_add(Weight::from_parts(157_674, 0).saturating_mul(r.into()))
            .saturating_add(T::DbWeight::get().reads(9_u64))
            .saturating_add(T::DbWeight::get().writes(4_u64))
            .saturating_add(Weight::from_parts(0, 3).saturating_mul(r.into()))
    }
    /// The range of component `r` is `[0, 5000]`.
    fn instr_i64const(r: u32, ) -> Weight {
        // Proof Size summary in bytes:
        //  Measured:  `0`
        //  Estimated: `0`
        // Minimum execution time: 1_635_000 picoseconds.
        Weight::from_parts(2_990_110, 0)
            // Standard Error: 31
            .saturating_add(Weight::from_parts(10_213, 0).saturating_mul(r.into()))
    }
}
