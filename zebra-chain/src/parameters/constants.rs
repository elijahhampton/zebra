//! Definitions of Zebra chain constants, including:
//! - slow start interval,
//! - slow start shift

use crate::block::Height;

/// An initial period from Genesis to this Height where the block subsidy is gradually incremented. [What is slow-start mining][slow-mining]
///
/// [slow-mining]: https://z.cash/support/faq/#what-is-slow-start-mining
pub const SLOW_START_INTERVAL: Height = Height(20_000);

/// `SlowStartShift()` as described in [protocol specification §7.8][7.8]
///
/// [7.8]: https://zips.z.cash/protocol/protocol.pdf#subsidies
///
/// This calculation is exact, because `SLOW_START_INTERVAL` is divisible by 2.
pub const SLOW_START_SHIFT: Height = Height(SLOW_START_INTERVAL.0 / 2);

/// ZIP208 block target intervals in seconds.
/// https://github.com/zcash/zcash/blob/master/src/consensus/params.h
/// Block target interval pre-Blossom upgrade in seconds.
const PRE_BLOSSOM_POW_TARGET_SPACING: u32 = 150;

/// Block target interval post-Blossom upgrade in seconds.
const POST_BLOSSOM_POW_TARGET_SPACING: u32 = 75;

/// Assert Blossom target spacing is less than pre-Blossom target spacing.
/// Ensures block times get faster after blossom upgrade.
const _: () = assert!(
    PRE_BLOSSOM_POW_TARGET_SPACING > POST_BLOSSOM_POW_TARGET_SPACING,
    "Blossom target spacing must be less than pre-Blossom target spacing."
);

/// Assert Blossom target spacing divides evenly into pre-Blossom spacing.
/// Ensures clean epoch boundaries when transitioning between the two intervals.
const _: () = assert!(
    PRE_BLOSSOM_POW_TARGET_SPACING % POST_BLOSSOM_POW_TARGET_SPACING == 0,
    "Blossom target spacing must exactly divide pre-Blossom target spacing."
);

/// The ratio between pre and post Blossom target spacing.
/// Used for calculations involving block timing across the Blossom boundary.
const BLOSSOM_POW_TARGET_SPACING_RATIO: u32 =
    PRE_BLOSSOM_POW_TARGET_SPACING / POST_BLOSSOM_POW_TARGET_SPACING;

/// Verify that BLOSSOM_POW_TARGET_SPACING_RATIO calculation is correct.
/// Ensures no rounding errors in integer division by checking ratio * new = old.
const _: () = assert!(
    BLOSSOM_POW_TARGET_SPACING_RATIO * POST_BLOSSOM_POW_TARGET_SPACING
        == PRE_BLOSSOM_POW_TARGET_SPACING,
    "Invalid BLOSSOM_POW_TARGET_SPACING_RATIO"
);

/// Default number of blocks, before blossom, after which a transaction expires.
pub const DEFAULT_PRE_BLOSSOM_TX_EXPIRY_DELTA: u32 = 20;
/// Default number of blocks, after blossom, after which a transaction expires.
pub const DEFAULT_POST_BLOSSOM_EXPIRY_DELTA: u32 =
    DEFAULT_PRE_BLOSSOM_TX_EXPIRY_DELTA * BLOSSOM_POW_TARGET_SPACING_RATIO;

/// Magic numbers used to identify different Zcash networks.
pub mod magics {
    use crate::parameters::network::magic::Magic;

    /// The production mainnet.
    pub const MAINNET: Magic = Magic([0x24, 0xe9, 0x27, 0x64]);
    /// The testnet.
    pub const TESTNET: Magic = Magic([0xfa, 0x1a, 0xf9, 0xbf]);
    /// The regtest, see <https://github.com/zcash/zcash/blob/master/src/chainparams.cpp#L716-L719>
    pub const REGTEST: Magic = Magic([0xaa, 0xe8, 0x3f, 0x5f]);
}
