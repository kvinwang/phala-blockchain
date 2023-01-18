use phactory_api::{
    blocks::{BlockHeader, StorageProof},
    pruntime_client,
};
use phaxt::subxt::rpc::types::ChainBlockResponse;
use serde::{Deserialize, Serialize};
use sp_core::sr25519;
use sp_runtime::{generic::SignedBlock as SpSignedBlock, OpaqueExtrinsic};

pub use sp_core::storage::{StorageData, StorageKey};

pub use phaxt::{self, *};
pub use subxt::rpc::types::NumberOrHex;

use codec::{Decode, Encode};

pub type PrClient = pruntime_client::PRuntimeClient;
pub type SrSigner = subxt::tx::PairSigner<Config, sr25519::Pair>;

pub type SignedBlock<Hdr, Ext> = SpSignedBlock<sp_runtime::generic::Block<Hdr, Ext>>;

pub type Block = SignedBlock<Header, OpaqueExtrinsic>;
// API: notify

#[derive(Serialize, Deserialize, Debug)]
pub struct NotifyReq {
    pub headernum: BlockNumber,
    pub blocknum: BlockNumber,
    pub pruntime_initialized: bool,
    pub pruntime_new_init: bool,
    pub initial_sync_finished: bool,
}

pub mod utils {
    use super::StorageProof;
    use phaxt::subxt::rpc::types::ReadProof;
    pub fn raw_proof<T>(read_proof: ReadProof<T>) -> StorageProof {
        read_proof.proof.into_iter().map(|p| p.0).collect()
    }
}

pub trait ConvertTo<T> {
    fn convert_to(&self) -> T;
}

impl<H> ConvertTo<BlockHeader> for H
where
    H: subxt::config::Header,
{
    fn convert_to(&self) -> BlockHeader {
        Decode::decode(&mut &self.encode()[..]).expect("Failed to convert block header")
    }
}

impl<T> ConvertTo<Block> for ChainBlockResponse<T> {
    fn convert_to(&self) -> Block {
        Block {
            block: sp_runtime::generic::Block {
                header: self.block.header.convert_to(),
                extrinsics: vec![],
            },
            justifications: todo!(),
        }
    }
}
