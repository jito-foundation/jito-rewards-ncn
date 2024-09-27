#[repr(u8)]
pub enum Discriminators {
    EpochPriceTable = 0,
    EpochRewardMerkleRoot = 1,
    EpochRewardMerkleRootTicket = 2,
    EpochRewardDistributionMarker = 3,
    RewardDropbox = 4,
    RewardConfig = 5,
}
