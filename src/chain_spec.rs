use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};
use im_online::sr25519::AuthorityId as ImOnlineId;
use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use yalland_node_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY, Signature,
	EVMConfig, ImOnlineConfig, AuthorityDiscoveryConfig, SessionConfig,
	SessionKeys, StakingConfig, StakerStatus, ContractsConfig
};
use yalland_node_runtime::constants::currency::*;
use sc_consensus_babe::{AuthorityId as BabeId};
use grandpa_primitives::{AuthorityId as GrandpaId};
use sc_service;
use sp_runtime::traits::{Verify, IdentifyAccount};
use sp_runtime::Perbill;
use hex_literal::hex;

// Note this is the URL for the telemetry server
//const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Specialized `ChainSpec`. This is a specialization of the general Substrate ChainSpec type.
pub type ChainSpec = sc_service::ChainSpec<GenesisConfig>;

/// The chain specification option. This is expected to come in from the CLI and
/// is little more than one of a number of alternatives which can easily be converted
/// from a string (`--chain=...`) into a `ChainSpec`.
#[derive(Clone, Debug)]
pub enum Alternative {
	/// Whatever the current runtime is, with just Alice as an auth.
	Development,
	/// Whatever the current runtime is, with simple Alice/Bob auths.
	LocalTestnet,
	YallandCom
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

type AccountPublic = <Signature as Verify>::Signer;

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
	seed: &str
) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ImOnlineId,
	AuthorityDiscoveryId
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

fn session_keys(
	babe: BabeId,
	grandpa: GrandpaId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { babe, grandpa, im_online, authority_discovery }
}


impl Alternative {
	/// Get an actual chain config from one of the alternatives.
	pub(crate) fn load(self) -> Result<ChainSpec, String> {
		Ok(match self {
			Alternative::Development => ChainSpec::from_genesis(
				"Development",
				"dev",
				|| testnet_genesis(vec![
					get_authority_keys_from_seed("Alice"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::LocalTestnet => ChainSpec::from_genesis(
				"Local Testnet",
				"local_testnet",
				|| testnet_genesis(vec![
					get_authority_keys_from_seed("Alice"),
					get_authority_keys_from_seed("Bob"),
				],
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
					get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
					get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
					get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
					get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
					get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
				],
				true),
				vec![],
				None,
				None,
				None,
				None
			),
			Alternative::YallandCom => {
				ChainSpec::from_genesis(
					"yalland.com",
					"yalland",
					// WARNING: SS58 public addresses are not in sync with public addresses
					|| testnet_genesis(
						vec![(
							// Stash - 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
							hex!["62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935"].into(),
							// Controller - 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
							hex!["3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d"].into(),
							// Babe - 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
							// Grandpa - 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
							hex!["f3e9d049bf974441e6603e28d216b6f7b01addc12a987224f09f62ccd2df4c9f"].unchecked_into(),
							// ImOnline - 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
							// AuthorityDiscovery - 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
						),(
							// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
							hex!["a83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b"].into(),
							// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
							hex!["b819d8c01cbc46e23d9b79f7654f704a828fa1946bc8a97f56889daade1ced4e"].into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
							// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
							hex!["a9bc13a350d35a666ab5007f09a90bc2e7f4bcd58b0be8208a03347e46c9395d"].unchecked_into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
						)],
						hex!["2ae01eb7c88dfb7d0b1654eada34b948838d8123e067bbf43ae6615f2f2e2119"].into(),
						vec![
							// 5D2vSp8hqLtPfB5DRwP1BasLT5j186XPm6GzHoR4V1PqU7tt
							hex!["2ae01eb7c88dfb7d0b1654eada34b948838d8123e067bbf43ae6615f2f2e2119"].into()
						],
						true
					),
					vec![],
					None,
					None,
					None,
					None,
				)
			},
		})
	}

	pub(crate) fn from(s: &str) -> Option<Self> {
		match s {
			"dev" => Some(Alternative::Development),
			"local" => Some(Alternative::LocalTestnet),
			"" | "yalland" => Some(Alternative::YallandCom),
			_ => None,
		}
	}
}

fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	enable_println: bool) -> GenesisConfig {

	const ENDOWMENT: u128 = 21_000_000 * YNT;
	const STASH: u128 = 100 * YNT;

	GenesisConfig {
		system: Some(SystemConfig {
			code: WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(IndicesConfig {
			ids: endowed_accounts.clone(),
		}),
		balances: Some(BalancesConfig {
			balances: endowed_accounts.iter().cloned()
				.map(|k| (k, ENDOWMENT))
				.chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
				.collect(),
			vesting: vec![],
		}),
		sudo: Some(SudoConfig {
			key: root_key,
		}),
		session: Some(SessionConfig {
			keys: initial_authorities.iter().map(|x| {
				(x.0.clone(), session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()))
			}).collect::<Vec<_>>(),
		}),
		staking: Some(StakingConfig {
			current_era: 0,
			validator_count: 20,
			minimum_validator_count: 2,
			stakers: initial_authorities.iter().map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator)).collect(),
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			..Default::default()
		}),
		babe: Some(BabeConfig {
			authorities: vec![],
		}),
		grandpa: Some(GrandpaConfig {
			authorities: vec![],
		}),
		im_online: Some(ImOnlineConfig {
			keys: vec![],
		}),
		authority_discovery: Some(AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		evm: Some(EVMConfig::default()),
		pallet_contracts: Some(ContractsConfig {
			current_schedule: pallet_contracts::Schedule {
				enable_println, // this should only be enabled on development chains
				..Default::default()
			},
			gas_price: 1 * MILLICENTS,
		}),
	}
}
