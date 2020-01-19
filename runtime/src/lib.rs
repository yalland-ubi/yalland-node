//! The Substrate Node Template runtime. This can be compiled with `#[no_std]`, ready for Wasm.

#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit="256"]

// Make the WASM binary available.
#[cfg(feature = "std")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));

use sp_std::prelude::*;
use sp_core::{OpaqueMetadata, U256, H160};
use sp_runtime::{
	ApplyExtrinsicResult, transaction_validity::TransactionValidity, generic, create_runtime_str,
	impl_opaque_keys, MultiSignature, curve::PiecewiseLinear,
};
use sp_runtime::traits::{
	BlakeTwo256, Block as BlockT, StaticLookup, Verify, ConvertInto, IdentifyAccount, OpaqueKeys,
	Extrinsic, SaturatedConversion
};
use im_online::sr25519::{AuthorityId as ImOnlineId};
use impls::{CurrencyToVoteHandler};
use grandpa::AuthorityList as GrandpaAuthorityList;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use pallet_transaction_payment_rpc_runtime_api::RuntimeDispatchInfo;
use pallet_contracts_rpc_runtime_api::ContractExecResult;
use sp_api::impl_runtime_apis;
use system::offchain::TransactionSubmitter;
use grandpa::fg_primitives;
use sp_staking::SessionIndex;
use sp_version::RuntimeVersion;
#[cfg(feature = "std")]
use sp_version::NativeVersion;

// A few exports that help ease life for downstream crates.
#[cfg(any(feature = "std", test))]
pub use sp_runtime::BuildStorage;
pub use timestamp::Call as TimestampCall;
pub use balances::Call as BalancesCall;
pub use sp_runtime::{Permill, Perbill};
pub use evm::{ConvertAccountId, FeeCalculator};
pub use frame_support::{
	StorageValue, construct_runtime, parameter_types, debug,
	traits::Randomness,
	weights::Weight,
};

pub use staking::StakerStatus;

/// An index to a block.
pub type BlockNumber = u32;

/// Alias to 512-bit hash when used in the context of a transaction signature on the chain.
pub type Signature = MultiSignature;

/// Some way of identifying an account on the chain. We intentionally make it equivalent
/// to the public key of our transaction signing scheme.
pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

/// The type for looking up accounts. We don't expect more than 4 billion of them, but you
/// never know...
pub type AccountIndex = u32;

/// Balance of an account.
pub type Balance = u128;

/// Index of a transaction in the chain.
pub type Index = u32;

/// A hash of some data used by the chain.
pub type Hash = sp_core::H256;

/// Digest item type.
pub type DigestItem = generic::DigestItem<Hash>;

/// Used for the module template in `./template.rs`
mod template;

pub mod constants;
pub mod impls;

use constants::{time::*, currency::*};

impl_opaque_keys! {
	pub struct SessionKeys {
		pub babe: Babe,
		pub im_online: ImOnline,
		pub grandpa: Grandpa,
		pub authority_discovery: AuthorityDiscovery,
	}
}

/// This runtime version.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: create_runtime_str!("yalland-node"),
	impl_name: create_runtime_str!("yalland-node"),
	authoring_version: 1,
	spec_version: 1,
	impl_version: 1,
	apis: RUNTIME_API_VERSIONS,
};

pub const MILLISECS_PER_BLOCK: u64 = 6000;

pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;

// These time units are defined in number of blocks.
pub const MINUTES: BlockNumber = 60_000 / (MILLISECS_PER_BLOCK as BlockNumber);
pub const HOURS: BlockNumber = MINUTES * 60;
pub const DAYS: BlockNumber = HOURS * 24;

/// The version infromation used to identify this runtime when compiled natively.
#[cfg(feature = "std")]
pub fn native_version() -> NativeVersion {
	NativeVersion {
		runtime_version: VERSION,
		can_author_with: Default::default(),
	}
}

parameter_types! {
	pub const BlockHashCount: BlockNumber = 250;
	pub const MaximumBlockWeight: Weight = 1_000_000;
	pub const AvailableBlockRatio: Perbill = Perbill::from_percent(75);
	pub const MaximumBlockLength: u32 = 5 * 1024 * 1024;
	pub const Version: RuntimeVersion = VERSION;
}

impl system::Trait for Runtime {
	/// The identifier used to distinguish between accounts.
	type AccountId = AccountId;
	/// The aggregated dispatch type that is available for extrinsics.
	type Call = Call;
	/// The lookup mechanism to get account ID from whatever is passed in dispatchers.
	type Lookup = Indices;
	/// The index type for storing how many extrinsics an account has signed.
	type Index = Index;
	/// The index type for blocks.
	type BlockNumber = BlockNumber;
	/// The type for hashing blocks and tries.
	type Hash = Hash;
	/// The hashing algorithm used.
	type Hashing = BlakeTwo256;
	/// The header type.
	type Header = generic::Header<BlockNumber, BlakeTwo256>;
	/// The ubiquitous event type.
	type Event = Event;
	/// The ubiquitous origin type.
	type Origin = Origin;
	/// Maximum number of block number to block hash mappings to keep (oldest pruned first).
	type BlockHashCount = BlockHashCount;
	/// Maximum weight of each block.
	type MaximumBlockWeight = MaximumBlockWeight;
	/// Maximum size of all encoded transactions (in bytes) that are allowed in one block.
	type MaximumBlockLength = MaximumBlockLength;
	/// Portion of the block weight that is available to all normal transactions.
	type AvailableBlockRatio = AvailableBlockRatio;
	/// Version of the runtime.
	type Version = Version;
	type ModuleToIndex = ModuleToIndex;
}

parameter_types! {
	pub const EpochDuration: u64 = EPOCH_DURATION_IN_BLOCKS as u64;
	pub const ExpectedBlockTime: u64 = MILLISECS_PER_BLOCK;
}

impl babe::Trait for Runtime {
	type EpochDuration = EpochDuration;
	type ExpectedBlockTime = ExpectedBlockTime;
	type EpochChangeTrigger = babe::SameAuthoritiesForever;
}

impl grandpa::Trait for Runtime {
	type Event = Event;
}

impl indices::Trait for Runtime {
	/// The type for recording indexing into the account enumeration. If this ever overflows, there
	/// will be problems!
	type AccountIndex = AccountIndex;
	/// Use the standard means of resolving an index hint from an id.
	type ResolveHint = indices::SimpleResolveHint<Self::AccountId, Self::AccountIndex>;
	/// Determine whether an account is dead.
	type IsDeadAccount = Balances;
	/// The ubiquitous event type.
	type Event = Event;
}

parameter_types! {
	pub const MinimumPeriod: u64 = SLOT_DURATION / 2;
}

impl timestamp::Trait for Runtime {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = u64;
	type OnTimestampSet = Babe;
	type MinimumPeriod = MinimumPeriod;
}

parameter_types! {
	pub const ExistentialDeposit: u128 = 500;
	pub const TransferFee: u128 = 0;
	pub const CreationFee: u128 = 0;
}

impl balances::Trait for Runtime {
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// What to do if an account's free balance gets zeroed.
	type OnFreeBalanceZero = ();
	/// What to do if a new account is created.
	type OnNewAccount = Indices;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type TransferPayment = ();
	type ExistentialDeposit = ExistentialDeposit;
	type TransferFee = TransferFee;
	type CreationFee = CreationFee;
	type OnReapAccount = ();
}

parameter_types! {
	pub const TransactionBaseFee: Balance = 0;
	pub const TransactionByteFee: Balance = 1;
}

impl transaction_payment::Trait for Runtime {
	type Currency = balances::Module<Runtime>;
	type OnTransactionPayment = ();
	type TransactionBaseFee = TransactionBaseFee;
	type TransactionByteFee = TransactionByteFee;
	type WeightToFee = ConvertInto;
	type FeeMultiplierUpdate = ();
}

impl sudo::Trait for Runtime {
	type Event = Event;
	type Proposal = Call;
}

/// Used for the module template in `./template.rs`
impl template::Trait for Runtime {
	type Event = Event;
}

pub struct FixedGasPrice;
impl FeeCalculator for FixedGasPrice {
	fn min_gas_price() -> U256 {
		// The gas price is always 1 GWei XTH per 1 unit of gas.
		// Todo: allow users to dynamically adjust the gas price.
		// https://github.com/dothereum/dothereum/issues/52
		1_000_000_000.into()
	}
}

// Use truncated account IDs to convert for EVM accounts.
pub struct TruncatedAccountId;
impl<AccountId> ConvertAccountId<AccountId> for TruncatedAccountId {
	fn convert_account_id(_account_id: &AccountId) -> H160 {
		// Todo: this needs a proper implementation.
		// Either truncate the first several bits and return the resulting H160;
		// Or hashing directly is easier to figure this out.
		// https://github.com/dothereum/dothereum/issues/53
		unimplemented!();
	}
}

// Implement the EVM Trait for the Dothereum Runtime.
impl evm::Trait for Runtime {
	type FeeCalculator = FixedGasPrice;
	type ConvertAccountId = TruncatedAccountId;
	type Currency = Balances;
	type Event = Event;
	type Precompiles = ();
}

parameter_types! {
	pub const ContractTransferFee: Balance = 1 * CENTS;
	pub const ContractCreationFee: Balance = 1 * CENTS;
	pub const ContractTransactionBaseFee: Balance = 1 * CENTS;
	pub const ContractTransactionByteFee: Balance = 10 * MILLICENTS;
	pub const ContractFee: Balance = 1 * CENTS;
	pub const TombstoneDeposit: Balance = 1 * DOLLARS;
	pub const RentByteFee: Balance = 1 * DOLLARS;
	pub const RentDepositOffset: Balance = 1000 * DOLLARS;
	pub const SurchargeReward: Balance = 150 * DOLLARS;
}

impl pallet_contracts::Trait for Runtime {
	type Currency = Balances;
	type Time = Timestamp;
	type Randomness = RandomnessCollectiveFlip;
	type Call = Call;
	type Event = Event;
	type DetermineContractAddress = pallet_contracts::SimpleAddressDeterminator<Runtime>;
	type ComputeDispatchFee = pallet_contracts::DefaultDispatchFeeComputor<Runtime>;
	type TrieIdGenerator = pallet_contracts::TrieIdFromParentCounter<Runtime>;
	type GasPayment = ();
	type RentPayment = ();
	type SignedClaimHandicap = pallet_contracts::DefaultSignedClaimHandicap;
	type TombstoneDeposit = TombstoneDeposit;
	type StorageSizeOffset = pallet_contracts::DefaultStorageSizeOffset;
	type RentByteFee = RentByteFee;
	type RentDepositOffset = RentDepositOffset;
	type SurchargeReward = SurchargeReward;
	type TransferFee = ContractTransferFee;
	type CreationFee = ContractCreationFee;
	type TransactionBaseFee = ContractTransactionBaseFee;
	type TransactionByteFee = ContractTransactionByteFee;
	type ContractFee = ContractFee;
	type CallBaseFee = pallet_contracts::DefaultCallBaseFee;
	type InstantiateBaseFee = pallet_contracts::DefaultInstantiateBaseFee;
	type MaxDepth = pallet_contracts::DefaultMaxDepth;
	type MaxValueSize = pallet_contracts::DefaultMaxValueSize;
	type BlockGasLimit = pallet_contracts::DefaultBlockGasLimit;
}

parameter_types! {
	pub const SessionDuration: BlockNumber = EPOCH_DURATION_IN_SLOTS as _;
}

pub type SubmitTransaction = TransactionSubmitter<ImOnlineId, Runtime, UncheckedExtrinsic>;

impl im_online::Trait for Runtime {
	type AuthorityId = ImOnlineId;
	type Event = Event;
	type Call = Call;
	type SubmitTransaction = SubmitTransaction;
	type SessionDuration = SessionDuration;
	type ReportUnresponsiveness = Offences;
}

impl system::offchain::CreateTransaction<Runtime, UncheckedExtrinsic> for Runtime {
	type Public = <Signature as Verify>::Signer;
	type Signature = Signature;

	fn create_transaction<TSigner: system::offchain::Signer<Self::Public, Self::Signature>>(
		call: Call,
		public: Self::Public,
		account: AccountId,
		index: Index,
	) -> Option<(Call, <UncheckedExtrinsic as Extrinsic>::SignaturePayload)> {
		// take the biggest period possible.
		let period = BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let current_block = System::block_number()
			.saturated_into::<u64>()
			// The `System::block_number` is initialized with `n+1`,
			// so the actual block number is `n`.
			.saturating_sub(1);
		let tip = 0;
		let extra: SignedExtra = (
			system::CheckVersion::<Runtime>::new(),
			system::CheckGenesis::<Runtime>::new(),
			system::CheckEra::<Runtime>::from(generic::Era::mortal(period, current_block)),
			system::CheckNonce::<Runtime>::from(index),
			system::CheckWeight::<Runtime>::new(),
			transaction_payment::ChargeTransactionPayment::<Runtime>::from(tip),
			Default::default(),
		);
		let raw_payload = SignedPayload::new(call, extra).map_err(|e| {
			debug::warn!("Unable to create signed payload: {:?}", e);
		}).ok()?;
		let signature = TSigner::sign(public, &raw_payload)?;
		let address = Indices::unlookup(account);
		let (call, extra, _) = raw_payload.deconstruct();
		Some((call, (address, signature, extra)))
	}
}

pallet_staking_reward_curve::build! {
	const REWARD_CURVE: PiecewiseLinear<'static> = curve!(
		min_inflation: 0_025_000,
		max_inflation: 0_100_000,
		ideal_stake: 0_500_000,
		falloff: 0_050_000,
		max_piece_count: 40,
		test_precision: 0_005_000,
	);
}

parameter_types! {
	// Six sessions in an era (24 hours).
	pub const SessionsPerEra: SessionIndex = 6;
	// 28 eras for unbonding (28 days).
	// KUSAMA: This value is 1/4 of what we expect for the mainnet, however session length is also
	// a quarter, so the figure remains the same.
	pub const BondingDuration: staking::EraIndex = 28;
	pub const SlashDeferDuration: staking::EraIndex = 28;
	pub const RewardCurve: &'static PiecewiseLinear<'static> = &REWARD_CURVE;
}

impl staking::Trait for Runtime {
	type Currency = Balances;
	type Time = Timestamp;
	type CurrencyToVote = CurrencyToVoteHandler;
	type RewardRemainder = ();
	type Event = Event;
	type Slash = ();
	type Reward = ();
	type SessionsPerEra = SessionsPerEra;
	type BondingDuration = BondingDuration;
	type SlashDeferDuration = SlashDeferDuration;
	type SlashCancelOrigin = system::EnsureRoot<AccountId>;
	type SessionInterface = Self;
	type RewardCurve = RewardCurve;
}

impl authority_discovery::Trait for Runtime {}

parameter_types! {
	pub const WindowSize: BlockNumber = 101;
	pub const ReportLatency: BlockNumber = 1000;
}

impl finality_tracker::Trait for Runtime {
	type OnFinalizationStalled = Grandpa;
	type WindowSize = WindowSize;
	type ReportLatency = ReportLatency;
}

parameter_types! {
	pub const UncleGenerations: BlockNumber = 5;
}

impl authorship::Trait for Runtime {
	type FindAuthor = session::FindAccountFromAuthorIndex<Self, Babe>;
	type UncleGenerations = UncleGenerations;
	type FilterUncle = ();
	type EventHandler = (Staking, ImOnline);
}

impl offences::Trait for Runtime {
	type Event = Event;
	type IdentificationTuple = session::historical::IdentificationTuple<Self>;
	type OnOffenceHandler = Staking;
}

parameter_types! {
	pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(17);
}

impl session::Trait for Runtime {
	type Event = Event;
	type ValidatorId = <Self as system::Trait>::AccountId;
	type ValidatorIdOf = staking::StashOf<Self>;
	type ShouldEndSession = Babe;
	type OnSessionEnding = Staking;
	type SessionHandler = <SessionKeys as OpaqueKeys>::KeyTypeIdProviders;
	type Keys = SessionKeys;
	type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
	type SelectInitialValidators = Staking;
}

impl session::historical::Trait for Runtime {
	type FullIdentification = staking::Exposure<AccountId, Balance>;
	type FullIdentificationOf = staking::ExposureOf<Runtime>;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = node_primitives::Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		// Basic stuff; balances is uncallable initially.
		System: system::{Module, Call, Storage, Config, Event},
		RandomnessCollectiveFlip: randomness_collective_flip::{Module, Storage},

		// Must be before session.
		Babe: babe::{Module, Call, Storage, Config, Inherent(Timestamp)},

		TemplateModule: template::{Module, Call, Storage, Event<T>},
		Timestamp: timestamp::{Module, Call, Storage, Inherent},
		Indices: indices,
		Balances: balances,
		TransactionPayment: transaction_payment::{Module, Storage},
		EVM: evm::{Module, Call, Storage, Config, Event},
		Contracts: pallet_contracts,
		Sudo: sudo,

		// Consensus
		Session: session::{Module, Call, Storage, Event, Config<T>},
		Grandpa: grandpa::{Module, Call, Storage, Config, Event},
		ImOnline: im_online::{Module, Call, Storage, Event<T>, ValidateUnsigned, Config<T>},
		AuthorityDiscovery: authority_discovery::{Module, Call, Config},
		FinalityTracker: finality_tracker::{Module, Call, Inherent},
		Authorship: authorship::{Module, Call, Storage, Inherent},
		Staking: staking,
		Offences: offences::{Module, Call, Storage, Event},
	}
);

/// The address format for describing accounts.
pub type Address = <Indices as StaticLookup>::Source;
/// Block header type as expected by this runtime.
pub type Header = generic::Header<BlockNumber, BlakeTwo256>;
/// Block type as expected by this runtime.
pub type Block = generic::Block<Header, UncheckedExtrinsic>;
/// A Block signed with a Justification
pub type SignedBlock = generic::SignedBlock<Block>;
/// BlockId type as expected by this runtime.
pub type BlockId = generic::BlockId<Block>;
/// The SignedExtension to the basic transaction logic.
pub type SignedExtra = (
	system::CheckVersion<Runtime>,
	system::CheckGenesis<Runtime>,
	system::CheckEra<Runtime>,
	system::CheckNonce<Runtime>,
	system::CheckWeight<Runtime>,
	transaction_payment::ChargeTransactionPayment<Runtime>,
	pallet_contracts::CheckBlockGasLimit<Runtime>,
);
/// Unchecked extrinsic type as expected by this runtime.
pub type UncheckedExtrinsic = generic::UncheckedExtrinsic<Address, Call, Signature, SignedExtra>;
/// The payload being signed in transactions.
pub type SignedPayload = generic::SignedPayload<Call, SignedExtra>;
/// Extrinsic type that has already been checked.
pub type CheckedExtrinsic = generic::CheckedExtrinsic<AccountId, Call, SignedExtra>;
/// Executive: handles dispatch to the various modules.
pub type Executive = frame_executive::Executive<Runtime, Block, system::ChainContext<Runtime>, Runtime, AllModules>;

impl_runtime_apis! {
	impl sp_api::Core<Block> for Runtime {
		fn version() -> RuntimeVersion {
			VERSION
		}

		fn execute_block(block: Block) {
			Executive::execute_block(block)
		}

		fn initialize_block(header: &<Block as BlockT>::Header) {
			Executive::initialize_block(header)
		}
	}

	impl sp_api::Metadata<Block> for Runtime {
		fn metadata() -> OpaqueMetadata {
			Runtime::metadata().into()
		}
	}

	impl sp_block_builder::BlockBuilder<Block> for Runtime {
		fn apply_extrinsic(extrinsic: <Block as BlockT>::Extrinsic) -> ApplyExtrinsicResult {
			Executive::apply_extrinsic(extrinsic)
		}

		fn finalize_block() -> <Block as BlockT>::Header {
			Executive::finalize_block()
		}

		fn inherent_extrinsics(data: sp_inherents::InherentData) -> Vec<<Block as BlockT>::Extrinsic> {
			data.create_extrinsics()
		}

		fn check_inherents(
			block: Block,
			data: sp_inherents::InherentData,
		) -> sp_inherents::CheckInherentsResult {
			data.check_extrinsics(&block)
		}

		fn random_seed() -> <Block as BlockT>::Hash {
			RandomnessCollectiveFlip::random_seed()
		}
	}

	impl sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block> for Runtime {
		fn validate_transaction(tx: <Block as BlockT>::Extrinsic) -> TransactionValidity {
			Executive::validate_transaction(tx)
		}
	}

	impl sp_offchain::OffchainWorkerApi<Block> for Runtime {
		fn offchain_worker(header: &<Block as BlockT>::Header) {
			Executive::offchain_worker(header)
		}
	}

	impl fg_primitives::GrandpaApi<Block> for Runtime {
		fn grandpa_authorities() -> GrandpaAuthorityList {
			Grandpa::grandpa_authorities()
		}
	}

	impl babe_primitives::BabeApi<Block> for Runtime {
		fn configuration() -> babe_primitives::BabeConfiguration {
			// The choice of `c` parameter (where `1 - c` represents the
			// probability of a slot being empty), is done in accordance to the
			// slot duration and expected target block time, for safely
			// resisting network delays of maximum two seconds.
			// <https://research.web3.foundation/en/latest/polkadot/BABE/Babe/#6-practical-results>
			babe_primitives::BabeConfiguration {
				slot_duration: Babe::slot_duration(),
				epoch_length: EpochDuration::get(),
				c: PRIMARY_PROBABILITY,
				genesis_authorities: Babe::authorities(),
				randomness: Babe::randomness(),
				secondary_slots: true,
			}
		}
	}

	impl sp_authority_discovery::AuthorityDiscoveryApi<Block> for Runtime {
		fn authorities() -> Vec<AuthorityDiscoveryId> {
			AuthorityDiscovery::authorities()
		}
	}

	impl frame_system_rpc_runtime_api::AccountNonceApi<Block, AccountId, Index> for Runtime {
		fn account_nonce(account: AccountId) -> Index {
			System::account_nonce(account)
		}
	}

	impl pallet_contracts_rpc_runtime_api::ContractsApi<Block, AccountId, Balance> for Runtime {
		fn call(
			origin: AccountId,
			dest: AccountId,
			value: Balance,
			gas_limit: u64,
			input_data: Vec<u8>,
		) -> ContractExecResult {
			let exec_result = Contracts::bare_call(
				origin,
				dest.into(),
				value,
				gas_limit,
				input_data,
			);
			match exec_result {
				Ok(v) => ContractExecResult::Success {
					status: v.status,
					data: v.data,
				},
				Err(_) => ContractExecResult::Error,
			}
		}

		fn get_storage(
			address: AccountId,
			key: [u8; 32],
		) -> pallet_contracts_rpc_runtime_api::GetStorageResult {
			Contracts::get_storage(address, key).map_err(|rpc_err| {
				use pallet_contracts::GetStorageError;
				use pallet_contracts_rpc_runtime_api::{GetStorageError as RpcGetStorageError};
				/// Map the contract error into the RPC layer error.
				match rpc_err {
					GetStorageError::ContractDoesntExist => RpcGetStorageError::ContractDoesntExist,
					GetStorageError::IsTombstone => RpcGetStorageError::IsTombstone,
				}
			})
		}
	}

	impl pallet_transaction_payment_rpc_runtime_api::TransactionPaymentApi<
		Block,
		Balance,
		UncheckedExtrinsic,
	> for Runtime {
		fn query_info(uxt: UncheckedExtrinsic, len: u32) -> RuntimeDispatchInfo<Balance> {
			TransactionPayment::query_info(uxt, len)
		}
	}

	impl sp_session::SessionKeys<Block> for Runtime {
		fn generate_session_keys(seed: Option<Vec<u8>>) -> Vec<u8> {
			SessionKeys::generate(seed)
		}
	}
}
