use crate as pallet_kitties;
use sp_core::H256;
use frame_support::parameter_types;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup}, testing::Header,
};
use sp_runtime::BuildStorage;
use frame_system as system;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        RandomnessCollectiveFlip: pallet_randomness_collective_flip::{Pallet, Storage},
		SubstrateKitties: pallet_kitties::{Pallet, Call, Storage, Config<T>, Event<T>},
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	// type BaseCallFilter = ();
	// type BlockWeights = ();
	// type BlockLength = ();
	// type DbWeight = ();
	// type Origin = Origin;
	// type Call = Call;
	// type Index = u64;
	// type BlockNumber = u64;
	// type Hash = H256;
	// type Hashing = BlakeTwo256;
	// type AccountId = u64;
	// type Lookup = IdentityLookup<Self::AccountId>;
	// type Header = Header;
	// type Event = Event;
	// type BlockHashCount = BlockHashCount;
	// type Version = ();
	// type PalletInfo = PalletInfo;
	// type AccountData = ();
	// type OnNewAccount = ();
	// type OnKilledAccount = ();
	// type SystemWeightInfo = ();
	// type SS58Prefix = SS58Prefix;
	// type OnSetCode = ();
	type AccountData = pallet_balances::AccountData<u64>;
	type AccountId = u64;
	type BaseCallFilter = ();
	// type BaseCallFilter = Type;
	type BlockHashCount = BlockHashCount;
	type BlockLength = ();
	type BlockNumber = u64;
	type BlockWeights = ();
	type Call = Call;
	type DbWeight = ();
	type Event = Event;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Index = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type OnKilledAccount = ();
	type OnNewAccount = ();
	type OnSetCode = ();
	type Origin = Origin;
	type PalletInfo = PalletInfo;
	type SS58Prefix = SS58Prefix;
	type SystemWeightInfo = ();
	type Version = ();
}

impl pallet_kitties::Config for Test {
	type Event = Event;
    type Randomness = RandomnessCollectiveFlip;
	type Currency = Balances;

}

impl pallet_randomness_collective_flip::Config for Test {}

parameter_types!{
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
	type AccountStore = System;
	type Balance = u64;
	type DustRemoval = ();
	type Event = Event;
	type ExistentialDeposit = ExistentialDeposit;
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	type WeightInfo = ();
}


// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisConfig {
		balances: BalancesConfig {
			balances: vec![(1,100), (2,100)]
		},
		substrate_kitties: SubstrateKittiesConfig {
			kitties: vec![
				(1, [0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0]),
				(2, [1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0])
			]
		},
		..Default::default()
	}
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(||System::set_block_number(1));
	ext
}
