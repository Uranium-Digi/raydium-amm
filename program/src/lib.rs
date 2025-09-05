// #![deny(missing_docs)]

//! An Uniswap-like program for the Solana blockchain.
#[macro_use]
pub mod log;

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod invokers;
pub mod math;
pub mod processor;
pub mod state;

// Export current solana-sdk types for downstream users who may also be building with a different solana-sdk version
pub use solana_program;

#[cfg(not(feature = "no-entrypoint"))]
solana_security_txt::security_txt! {
    name: "raydium-amm",
    project_url: "https://raydium.io",
    contacts: "link:https://immunefi.com/bounty/raydium",
    policy: "https://immunefi.com/bounty/raydium",
    source_code: "https://github.com/raydium-io/raydium-amm",
    preferred_languages: "en",
    auditors: "https://github.com/raydium-io/raydium-docs/blob/master/audit/MadSheild%20Q2%202023/Raydium%20updated%20orderbook%20AMM%20program%20%26%20OpenBook%20migration.pdf"
}

// #[cfg(feature = "mainnet")]
// solana_program::declare_id!("2Eb6JxPEizUJraUKCDuYxHQN7t2exHVdc8ec5apYNFmG");
// #[cfg(feature = "devnet")]
// solana_program::declare_id!("2Eb6JxPEizUJraUKCDuYxHQN7t2exHVdc8ec5apYNFmG");
// #[cfg(feature = "localnet")]
solana_program::declare_id!("ESzFnFJt9gvE24HZSccA9js6L1NAUgvQLJexKxrmTXgC");
