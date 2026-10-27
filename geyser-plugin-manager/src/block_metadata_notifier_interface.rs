use {
    solana_clock::UnixTimestamp,
    solana_hash::Hash,
    solana_runtime::bank::{BankId, KeyedRewardsAndNumPartitions},
    std::sync::Arc,
};

/// Interface for notifying block metadata changes
pub trait BlockMetadataNotifier {
    /// Notify the block metadata
    #[allow(clippy::too_many_arguments)]
    fn notify_block_metadata(
        &self,
        parent_slot: u64,
        parent_blockhash: &str,
        slot: u64,
        bank_id: BankId,
        blockhash: &str,
        rewards: &KeyedRewardsAndNumPartitions,
        block_time: Option<UnixTimestamp>,
        block_height: Option<u64>,
        executed_transaction_count: u64,
        entry_count: u64,
        commission_rate_in_basis_points: bool,
        bank_hashes: &BankHashes,
    );
}

/// The hashes that identify a frozen bank's resulting state.
pub struct BankHashes {
    pub block_id: Option<Hash>,
    pub bank_hash: Hash,
    pub parent_bank_hash: Hash,
    pub accounts_lt_hash_checksum: [u8; 32],
    pub signature_count: u64,
}

pub type BlockMetadataNotifierArc = Arc<dyn BlockMetadataNotifier + Sync + Send>;
