use polkadot_primitives::{parachain::{ValidatorId}, AccountId};
use polkadot_service::{
chain_spec::{get_account_id_from_seed, get_from_seed, Extensions}};
use babe_primitives::AuthorityId as BabeId;
use grandpa::AuthorityId as GrandpaId;
use polkadot_test_runtime::constants::currency::DOTS;
use sp_core::sr25519;
use sc_chain_spec::{ChainType};
use sp_runtime::{Perbill};
use pallet_staking::Forcing;

const DEFAULT_PROTOCOL_ID: &str = "dot";

/// The `ChainSpec parametrised for polkadot runtime`.
pub type PolkadotChainSpec = service::GenericChainSpec<
	polkadot_test_runtime::GenesisConfig,
	Extensions,
>;


/// Polkadot local testnet config (multivalidator Alice + Bob)
pub fn polkadot_local_testnet_config() -> PolkadotChainSpec {
	PolkadotChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		polkadot_local_testnet_genesis,
		vec![],
		None,
		Some(DEFAULT_PROTOCOL_ID),
		None,
		Default::default(),
	)
}

fn polkadot_local_testnet_genesis() -> polkadot_test_runtime::GenesisConfig {
	polkadot_testnet_genesis(
		vec![
			get_authority_keys_from_seed("Alice"),
			get_authority_keys_from_seed("Bob"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Helper function to generate stash, controller and session key from seed
fn get_authority_keys_from_seed(seed: &str) -> (
	AccountId,
	AccountId,
	BabeId,
	GrandpaId,
	ValidatorId,
) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<ValidatorId>(seed),
	)
}

fn testnet_accounts() -> Vec<AccountId> {
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
	]
}

/// Helper function to create polkadot GenesisConfig for testing
fn polkadot_testnet_genesis(
	initial_authorities: Vec<(AccountId, AccountId, BabeId, GrandpaId, ValidatorId)>,
	_root_key: AccountId, // TODO
	endowed_accounts: Option<Vec<AccountId>>,
) -> polkadot_test_runtime::GenesisConfig {
	use polkadot_test_runtime as polkadot;

	let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

	const ENDOWMENT: u128 = 1_000_000 * DOTS;
	const STASH: u128 = 100 * DOTS;

	polkadot::GenesisConfig {
		system: Some(polkadot::SystemConfig {
			code: polkadot::WASM_BINARY.to_vec(),
			changes_trie_config: Default::default(),
		}),
		indices: Some(polkadot::IndicesConfig {
			indices: vec![],
		}),
		balances: Some(polkadot::BalancesConfig {
			balances: endowed_accounts.iter().map(|k| (k.clone(), ENDOWMENT)).collect(),
		}),
		session: Some(polkadot::SessionConfig {
			keys: initial_authorities.iter().map(|x| (
						  x.0.clone(),
						  x.0.clone(),
						polkadot_test_runtime::SessionKeys {
							babe: x.2.clone(),
							grandpa: x.3.clone(),
							parachain_validator: x.4.clone(),
						},
				  )).collect::<Vec<_>>(),
		}),
		staking: Some(polkadot::StakingConfig {
			minimum_validator_count: 1,
			validator_count: 2,
			stakers: initial_authorities.iter()
				.map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
				.collect(),
				invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
				force_era: Forcing::NotForcing,
				slash_reward_fraction: Perbill::from_percent(10),
				.. Default::default()
		}),
		babe: Some(Default::default()),
		grandpa: Some(Default::default()),
		authority_discovery: Some(polkadot::AuthorityDiscoveryConfig {
			keys: vec![],
		}),
		parachains: Some(polkadot::ParachainsConfig {
			authorities: vec![],
		}),
		registrar: Some(polkadot::RegistrarConfig{
			parachains: vec![],
			_phdata: Default::default(),
		}),
		claims: Some(polkadot::ClaimsConfig {
			claims: vec![],
			vesting: vec![],
		}),
		vesting: Some(polkadot::VestingConfig {
			vesting: vec![],
		}),
		// TODO: it should have one?
		/*
		sudo: Some(polkadot::SudoConfig {
			key: root_key,
		}),
		*/
	}
}