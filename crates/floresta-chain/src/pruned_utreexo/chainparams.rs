//! This module provides configuration and parameters for different Bitcoin networks (mainnet,
//! testnet, signet, and regtest).
//!
//! It includes:
//! - Network-specific parameters like block reward halving intervals and maturity periods
//! - DNS seeds for peer discovery
//! - Assumable validation states for Utreexo
//! - Block verification flag exceptions
//!
//! The main struct [`ChainParams`] encapsulates all chain-specific parameters while
//! [`DnsSeed`] handles peer discovery through DNS.

extern crate alloc;
use alloc::vec::Vec;
use core::ffi::c_uint;

use bitcoin::blockdata::constants::genesis_block;
use bitcoin::p2p::ServiceFlags;
use bitcoin::params::Params;
use bitcoin::Block;
use bitcoin::BlockHash;
use bitcoin::Network;
use floresta_common::acchashes;
use floresta_common::bhash;
use rustreexo::accumulator::node_hash::BitcoinNodeHash;

use crate::prelude::*;
use crate::AssumeValidArg;
use crate::BlockchainError;

#[derive(Clone, Debug)]
/// This struct encapsulates all chain-specific parameters.
pub struct ChainParams {
    /// Field to store parameters related to the chain consensus.
    pub params: Params,

    /// The network's first block, also called genesis block.
    pub genesis: Block,

    /// Interval of blocks until the block reward halves
    pub subsidy_halving_interval: u64,

    /// When we retarget we expect this many seconds to be elapsed since last time. If
    /// it's more, we decrease difficulty, if it's less we increase difficulty
    pub pow_target_timespan: u64,

    /// We wait this many blocks before a coinbase output can be spent
    pub coinbase_maturity: u32,

    /// The height at which segwit is activated
    pub segwit_activation_height: u32,

    /// The height at which csv(CHECK_SEQUENCE_VERIFY) is activated
    pub csv_activation_height: u32,

    /// A list of exceptions to the rules, where the key is the block hash and the value is the
    /// verification flags
    pub exceptions: HashMap<BlockHash, c_uint>,

    /// The network this chain params is for
    pub network: Network,

    /// Whether we should enforce BIP-094 "Testnet 4" rules
    pub enforce_bip94: bool,
}

/// A dns seed is a authoritative DNS server that returns the IP addresses of nodes that are
/// likely to be accepting incoming connections. This is our preferred way of finding new peers
/// on the first startup, as peers returned by seeds are likely to be online and accepting
/// connections. We may use this as a fallback if we don't have any peers to connect in
/// subsequent startups.
///
/// Some seeds allow filtering by service flags, so we may use this to find peers that are
/// likely to be running Utreexo, for example.
pub struct DnsSeed {
    /// The network this peer supports (e.g, mainnet, testnet, etc)
    pub network: Network,

    /// The domain name of the seed
    pub seed: &'static str,

    /// Useful filters we can use to find relevant peers
    pub filters: ServiceFlags,
}

/// This functionality is used to create a new DNS seed with possible filters.
impl DnsSeed {
    /// Create a new DNS seed
    pub fn new(network: Network, seed: &'static str, filters: ServiceFlags) -> Self {
        DnsSeed {
            network,
            seed,
            filters,
        }
    }
}

/// If enabled, the node will assume that the provided Utreexo state is valid, and will
/// start running from there. You may use this to make your node start faster, but you
/// should be sure that the provided state is valid. You may or not verify the state,
/// by downloading all blocks on background, and then verifying the final Utreexo state.
#[derive(Debug, Clone)]
pub struct AssumeUtreexoValue {
    /// The latest block assumed to be valid. This acc is the roots at this block
    pub block_hash: BlockHash,

    /// Same as block_hash, but in height
    pub height: u32,

    /// The roots of the Utreexo accumulator at this block
    pub roots: Vec<BitcoinNodeHash>,

    /// The number of leaves in the Utreexo accumulator at this block
    pub leaves: u64,
}

impl ChainParams {
    /// This method is called when Assume Utreexo is set to true. It means that the user will accept the hardcoded utreexo state for the specified block, if it is found in the best chain. We can then sync rapidly from this state.
    pub fn get_assume_utreexo(network: Network) -> Result<AssumeUtreexoValue, BlockchainError> {
        let genesis = genesis_block(Params::new(network));
        match network {
            Network::Bitcoin => Ok(AssumeUtreexoValue {
                block_hash: bhash!(
                    "00000000000000000000569f4d863c27e667cbee8acc8da195e7e5551658e6e9"
                ),
                height: 855571,
                roots: acchashes![
                    "4dcc014cc23611dda2dcf0f34a3e62e7d302146df4b0b01ac701d440358c19d6",
                    "988e0a883e4ad0c5559432f4747395115112755ec1138dcdd62e2f5741c31c2c",
                    "49ecba683e12823d44f2ad190120d3028386d8bb7860a3eea62a250a1f293c60",
                    "7c02e55ae35f12501134f0b81a351abb6c5e7a2529641d0c537a7534a560c770",
                    "59cb07c73d71164ce1a4f953cfd01ef0e3269080e29d34022d4251523cb1e8ac",
                    "ff96c9983b6765092403f8089fe5d0cdd6a94c58e4dcd14e77570c8b10c17628",
                    "47ed934529b2ea03a7382febcf0c05e0bfc5884cc1235c2ad42624a56234b9a6",
                    "d5c9373ed35de281d426888bd656f04a36623197a33706932ab82014d67f26ae",
                    "05de50991df991f0b78d9166d06ce3c61cb29e07dc7c53ba75d75df6455e6967",
                    "ebfdaf53b7240e9cd25d7c63b35d462763253f9282cc97d8d0c92ea9ade6aa02",
                    "c349b6850f75346224cf7cf1e0a69e194306c59489017cd4f4a045c001f1fefc",
                    "7edfd925905e88fd14c47edaaf09606cf0ae19f3b898239a2feb607d175d9a90",
                    "442dadd38fd16949d2ef03d799aa6b61ad8c0b7c611aaa5e218bc6360c4f41ce",
                    "2a57b73e540c7a72cb44fdc4ab7fcc3f0f148be7885667f07fce345430f08a15",
                    "66dc66000a8baaacacef280783a0245b4d33bd7eba5f1f14b939bd3a54e135cb",
                    "67ba89afe6bce9bafbf0b88013e4446c861e6c746e291c3921e0b65c93671ba3",
                    "972ea2c7472c22e4eab49e9c2db5757a048b271b6251883ce89ccfeaa38b47ab",
                ]
                .to_vec(),
                leaves: 2587882501,
            }),
            Network::Testnet => Ok(AssumeUtreexoValue {
                block_hash: genesis.block_hash(),
                height: 0,
                leaves: 0,
                roots: Vec::new(),
            }),
            Network::Testnet4 => Ok(AssumeUtreexoValue {
                block_hash: genesis.block_hash(),
                height: 0,
                leaves: 0,
                roots: Vec::new(),
            }),
            Network::Signet => Ok(AssumeUtreexoValue {
                block_hash: genesis.block_hash(),
                height: 0,
                leaves: 0,
                roots: Vec::new(),
            }),
            Network::Regtest => Ok(AssumeUtreexoValue {
                block_hash: genesis.block_hash(),
                height: 0,
                leaves: 0,
                roots: Vec::new(),
            }),
            network => Err(BlockchainError::UnsupportedNetwork(network)),
        }
    }

    /// This method is used to assume all the scripts up to a specific block in the chain as valid. It can be None (we will verify all the scripts), user input or hardcoded.
    pub fn get_assume_valid(
        network: Network,
        arg: AssumeValidArg,
    ) -> Result<Option<BlockHash>, BlockchainError> {
        match arg {
            AssumeValidArg::Disabled => Ok(None),
            AssumeValidArg::UserInput(hash) => Ok(Some(hash)),
            AssumeValidArg::Hardcoded => match network {
                Network::Bitcoin => Ok(Some(bhash!(
                    "00000000000000000000569f4d863c27e667cbee8acc8da195e7e5551658e6e9"
                ))),
                Network::Testnet => Ok(Some(bhash!(
                    "000000000000001142ad197bff16a1393290fca09e4ca904dd89e7ae98a90fcd"
                ))),
                Network::Testnet4 => Ok(Some(bhash!(
                    "0000000006af13c1117f3e2eb14f10eb9736e255713118cf7eb6659b1448efc1"
                ))),
                Network::Signet => Ok(Some(bhash!(
                    "0000003ed17b9c93954daab00d73ccbd0092074c4ebfc751c7458d58b827dfea"
                ))),
                Network::Regtest => Ok(Some(bhash!(
                    "0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206"
                ))),
                network => Err(BlockchainError::UnsupportedNetwork(network)),
            },
        }
    }
}

#[cfg(feature = "bitcoinconsensus")]
/// We use an inverse logic to pick validation flags.
/// When we call verify_script we need to tell what to validate (taproot, segwit, CSV, P2SH...).
/// Although those features were added later in the protocol, their exact template would rarely appear in a transaction.
/// There's almost no transactions in the chain that "looks like segwit but are not segwit".
/// We pretend segwit was enabled since genesis, and only skip this for blocks that have such transactions using hardcoded values.
fn get_exceptions() -> HashMap<BlockHash, c_uint> {
    use bitcoinconsensus::VERIFY_NONE;
    use bitcoinconsensus::VERIFY_P2SH;
    use bitcoinconsensus::VERIFY_WITNESS;
    let mut exceptions = HashMap::new();
    exceptions.insert(
        bhash!("00000000000002dc756eebf4f49723ed8d30cc28a5f108eb94b1ba88ac4f9c22"),
        VERIFY_NONE,
    ); // BIP16 exception on main net
    exceptions.insert(
        bhash!("0000000000000000000f14c35b2d841e986ab5441de8c585d5ffe55ea1e395ad"),
        VERIFY_P2SH | VERIFY_WITNESS,
    ); // Taproot exception on main net
    exceptions.insert(
        bhash!("00000000dd30457c001f4095d208cc1296b0eed002427aa599874af7a432b105"),
        VERIFY_NONE,
    ); // BIP16 exception on test net
    exceptions
}

#[cfg(not(feature = "bitcoinconsensus"))]
fn get_exceptions() -> HashMap<BlockHash, c_uint> {
    HashMap::new()
}

impl AsRef<Params> for ChainParams {
    fn as_ref(&self) -> &Params {
        &self.params
    }
}

impl TryFrom<Network> for ChainParams {
    type Error = BlockchainError;

    fn try_from(network: Network) -> Result<Self, Self::Error> {
        let genesis = genesis_block(Params::new(network));
        let exceptions = get_exceptions();

        match network {
            Network::Bitcoin => Ok(ChainParams {
                params: Params::new(network),
                network,
                genesis,
                pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
                subsidy_halving_interval: 210_000,
                coinbase_maturity: 100,
                segwit_activation_height: 481_824,
                csv_activation_height: 419_328,
                exceptions,
                enforce_bip94: false,
            }),
            Network::Testnet => Ok(ChainParams {
                params: Params::new(network),
                network,
                genesis,
                pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
                subsidy_halving_interval: 210_000,
                coinbase_maturity: 100,
                segwit_activation_height: 834_624,
                csv_activation_height: 770_112,
                exceptions,
                enforce_bip94: false,
            }),
            Network::Testnet4 => Ok(ChainParams {
                params: Params::new(network),
                network,
                genesis,
                pow_target_timespan: 14 * 24 * 60 * 60,
                subsidy_halving_interval: 210_000,
                coinbase_maturity: 100,
                segwit_activation_height: 1,
                csv_activation_height: 1,
                exceptions,
                enforce_bip94: true,
            }),
            Network::Signet => Ok(ChainParams {
                params: Params::new(network),
                network,
                genesis,
                pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
                subsidy_halving_interval: 210_000,
                coinbase_maturity: 100,
                csv_activation_height: 1,
                segwit_activation_height: 1,
                exceptions,
                enforce_bip94: false,
            }),
            Network::Regtest => Ok(ChainParams {
                params: Params::new(network),
                network,
                genesis,
                pow_target_timespan: 14 * 24 * 60 * 60, // two weeks
                subsidy_halving_interval: 150,
                coinbase_maturity: 100,
                csv_activation_height: 0,
                segwit_activation_height: 0,
                exceptions,
                enforce_bip94: false,
            }),
            network => Err(BlockchainError::UnsupportedNetwork(network)),
        }
    }
}

/// This function returns the DNS seeds for the given network.
///
/// DNS seeds taken from Bitcoin Core at commit 382b692a503355df7347efd9c128aff465b5583e
///
/// Some dns seeds lets us filter the returned peers by advertised services. We are interested
/// in peers with: UTREEXO, COMPACT_FILTERS, WITNESS and NETWORK. Not all seeds supports all
/// bits, so from this list, we pick the ones they support, and ask for this.
pub fn get_chain_dns_seeds(network: Network) -> Result<Vec<DnsSeed>, BlockchainError> {
    let mut seeds = Vec::new();

    // x9 or 0x09 means NETWORK + WITNESS
    let x9: ServiceFlags = ServiceFlags::from(0x9);
    // this mean NETWORK + WITNESS + COMPACT_FILTERS
    let x49 = ServiceFlags::from(0x49);
    // this mean NETWORK + WITNESS + COMPACT_FILTERS + UTREEXO
    let x1000049 = ServiceFlags::from(0x1000049);
    // this means NETWORK + WITNESS + UTREEXO
    let x1000009 = ServiceFlags::from(0x1000009);
    // filters aren't supported (usually returns a static list of peers)
    let none = ServiceFlags::NONE;

    match network {
        Network::Bitcoin => {
            seeds.push(DnsSeed::new(
                Network::Bitcoin,
                "seed.calvinkim.info",
                x1000009,
            ));
            seeds.push(DnsSeed::new(
                Network::Bitcoin,
                "seed.bitcoin.luisschwab.com",
                x1000049,
            ));
            seeds.push(DnsSeed::new(
                Network::Bitcoin,
                "seed.bitcoin.sipa.be",
                x9, // no COMPACT_FILTERS
            ));
            seeds.push(DnsSeed::new(Network::Bitcoin, "dnsseed.bluematt.me", x49));
            seeds.push(DnsSeed::new(Network::Bitcoin, "seed.bitcoinstats.com", x49));
            seeds.push(DnsSeed::new(
                Network::Bitcoin,
                "seed.btc.petertodd.org",
                x49,
            ));
            seeds.push(DnsSeed::new(
                Network::Bitcoin,
                "seed.bitcoin.sprovoost.nl",
                x49,
            ));
            seeds.push(DnsSeed::new(Network::Bitcoin, "dnsseed.emzy.de", x49));
            seeds.push(DnsSeed::new(Network::Bitcoin, "seed.bitcoin.wiz.biz", x49));
        }
        Network::Testnet => {
            seeds.push(DnsSeed::new(
                Network::Testnet,
                "testnet-seed.bitcoin.jonasschnelli.ch",
                x49,
            ));
            seeds.push(DnsSeed::new(
                Network::Testnet,
                "seed.tbtc.petertodd.org",
                x49,
            ));
            seeds.push(DnsSeed::new(
                Network::Testnet,
                "seed.testnet.bitcoin.sprovoost.nl",
                x49,
            ));
            seeds.push(DnsSeed::new(
                Network::Testnet,
                "testnet-seed.bluematt.me",
                none,
            ));
        }
        Network::Testnet4 => {
            seeds.push(DnsSeed::new(
                Network::Testnet4,
                "seed.testnet4.bitcoin.sprovoost.nl",
                none,
            ));
            seeds.push(DnsSeed::new(
                Network::Testnet4,
                "seed.testnet4.wiz.biz",
                none,
            ));
        }
        Network::Signet => {
            seeds.push(DnsSeed::new(
                Network::Signet,
                "seed.signet.bitcoin.sprovoost.nl",
                x49,
            ));
            seeds.push(DnsSeed::new(Network::Signet, "seed.dlsouza.lol", x1000049));
        }
        Network::Regtest => {
            // No seeds for regtest
        }
        network => return Err(BlockchainError::UnsupportedNetwork(network)),
    };
    Ok(seeds)
}
