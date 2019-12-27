use sp_core::{Pair, Public, sr25519, crypto::UncheckedInto};
use im_online::sr25519::AuthorityId as ImOnlineId;
use authority_discovery_primitives::AuthorityId as AuthorityDiscoveryId;
use yalland_node_runtime::{
	AccountId, BabeConfig, BalancesConfig, GenesisConfig, GrandpaConfig,
	SudoConfig, IndicesConfig, SystemConfig, WASM_BINARY, Signature,
	EVMConfig, ImOnlineConfig, AuthorityDiscoveryConfig, SessionConfig,
	SessionKeys, StakingConfig, StakerStatus
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
	PalmnickenTestnet
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
			Alternative::PalmnickenTestnet => {
				ChainSpec::from_genesis(
					"Palmnicken Testnet",
					"palmnicken_testnet",
					|| testnet_genesis(
						vec![(
							// 5DD7Q4VEfPTLEdn11CnThoHT5f9xKCrnofWJL5SsvpTghaAT
							hex!["62692ff7965729b21a10862da513beb42b7da54f321e35e2147983e1e53dc935"].into(),
							// 5GNzaEqhrZAtUQhbMe2gn9jBuNWfamWFZHULryFwBUXyd1cG
							hex!["3cd09eecf6faa579ff49a5bb8175c02244da1151cfa75b8b3fc9dcb15b4b281d"].into(),
							// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
							// 5EjvdwATjyFFikdZibVvx1q5uBHhphS2Mnsq5c7yfaYK25vm
							hex!["f3e9d049bf974441e6603e28d216b6f7b01addc12a987224f09f62ccd2df4c9f"].unchecked_into(),
							// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
							// 5FpewyS2VY8Cj3tKgSckq8ECkjd1HKHvBRnWhiHqRQsWfFC1
							hex!["5cd39f695f92f114d33f731283968eb5627d82b20cf9b2357798dbdb271bd01a"].unchecked_into(),
						),(
							// 5G9VGb8ESBeS8Ca4or43RfhShzk9y7T5iTmxHk5RJsjZwsRx
							hex!["a83f9b156daa23ac07dd3361514d1b9f1674904d35ce8ab422bc8e1e12dac70b"].into(),
							// 5F7V9Y5FcxKXe1aroqvPeRiUmmeQwTFcL3u9rrPXcMuMiCNx
							hex!["a224f6e5a1d971019c3d3d012a6980ff6cd20686a345d121a2373029ef014898"].into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
							// 5HBDAaybNqjmY7ww8ZcZZY1L5LHxvpnyfqJwoB7HhR6raTmG
							hex!["a9bc13a350d35a666ab5007f09a90bc2e7f4bcd58b0be8208a03347e46c9395d"].unchecked_into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
							// 5GvuM53k1Z4nAB5zXJFgkRSHv4Bqo4BsvgbQWNWkiWZTMwWY
							hex!["206ff0c892b4b1faa0e4051476a997e9b31a28957aff53281a52f43967cf200f"].unchecked_into(),
						),(
							// 5FzwpgGvk2kk9agow6KsywLYcPzjYc8suKej2bne5G5b9YU3
							hex!["5e25b78d7ef73fb03c48b5550c7762f2fffaff54ef6cac0d670157cf2ba18563"].into(),
							// 5EqoZhVC2BcsM4WjvZNidu2muKAbu5THQTBKe3EjvxXkdP7A
							hex!["1897739a555a3ffc548045b2d3580510e9d30e4529d7b92bc35da4421200d160"].into(),
							// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
							hex!["5ac8b49bc092ecf7fe6d5ba8d44af9bfd15d93c2c99cbba5a98caad2f661642e"].unchecked_into(),
							// 5E8ULLQrDAtWhfnVfZmX41Yux86zNAwVJYguWJZVWrJvdhBe
							hex!["91795288ab550bbb972627fff63567fc1d1d6f1ecfbcec33afe64560186418c4"].unchecked_into(),
							// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
							hex!["5ac8b49bc092ecf7fe6d5ba8d44af9bfd15d93c2c99cbba5a98caad2f661642e"].unchecked_into(),
							// 5CXNq1mSKJT4Sc2CbyBBdANeSkbUvdWvE4czJjKXfBHi9sX5
							hex!["5ac8b49bc092ecf7fe6d5ba8d44af9bfd15d93c2c99cbba5a98caad2f661642e"].unchecked_into(),
						)],
						hex!["fe67fbbb51d35b1528f30a74ffd735bdf6caa63b45fb65ebeaa87217418ed154"].into(),
						vec![
							// 5CVFESwfkk7NmhQ6FwHCM9roBvr9BGa4vJHFYU8DnGQxrXvz
							hex!["fe67fbbb51d35b1528f30a74ffd735bdf6caa63b45fb65ebeaa87217418ed154"].into(),
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
			"palm" => Some(Alternative::PalmnickenTestnet),
			"" | "local" => Some(Alternative::LocalTestnet),
			_ => None,
		}
	}
}

fn testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ImOnlineId, AuthorityDiscoveryId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	_enable_println: bool) -> GenesisConfig {

	const ENDOWMENT: u128 = 1_000_000 * YNT;
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
			validator_count: 50,
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
	}
}
