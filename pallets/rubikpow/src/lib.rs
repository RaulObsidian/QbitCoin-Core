#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
    dispatch,
    pallet_prelude::*,
    traits::{Currency, ExistenceRequirement},
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Hash;
use sp_std::vec::Vec;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use qbitcoin_core::{Cube, Move, calculate_difficulty};

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
        type Currency: Currency<Self::AccountId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn difficulty)]
    pub type Difficulty<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn last_nonce)]
    pub type LastNonce<T: Config> = StorageValue<_, u64, ValueQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BlockMined { miner: T::AccountId, cube_size: u32 },
        Reward { miner: T::AccountId, amount: u32 },
        DifficultyAdjustment { new_difficulty: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidSolution,
        CubeTooSmall,
        CubeTooLarge,
        InvalidNonce,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))] // Adjust weight based on complexity
        pub fn submit_solution(
            origin: OriginFor<T>,
            cube_size: u32,
            moves: Vec<Move>,
            nonce: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(cube_size >= 2, Error::<T>::CubeTooSmall);
            ensure!(cube_size <= 16, Error::<T>::CubeTooLarge); // Limit cube size for performance

            // Ensure nonce is unique and increasing
            let last_nonce = Self::last_nonce();
            ensure!(nonce > last_nonce, Error::<T>::InvalidNonce);
            <LastNonce<T>>::put(nonce);

            // Create cube and scramble it with the nonce
            let mut cube = Cube::new(cube_size as usize);
            let block_header = b"mock_block_header"; // In a real implementation, this would be the actual block header
            let scramble = cube.scramble_deterministic(nonce, block_header);

            // Verify solution
            ensure!(cube.verify_solution(&moves), Error::<T>::InvalidSolution);

            // Check if the cube state meets the current difficulty target
            let difficulty = Self::difficulty();
            let target = difficulty; // Simplified: target is same as difficulty
            let hash = [0u8; 32]; // In a real implementation, this would be the cube's state hash
            ensure!(cube.meets_difficulty(hash, target), Error::<T>::InvalidSolution);

            let reward = Self::calculate_reward(cube_size);
            let new_difficulty = Self::adjust_difficulty(difficulty, cube_size);

            <Difficulty<T>>::put(new_difficulty);

            // Issue reward (simplified - in reality, would use T::Currency)
            // For now, we just deposit an event.
            Self::deposit_event(Event::BlockMined { miner: who.clone(), cube_size });
            Self::deposit_event(Event::Reward { miner: who, amount: reward });
            Self::deposit_event(Event::DifficultyAdjustment { new_difficulty });

            Ok(())
        }

        #[pallet::call_index(1)]
        #[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
        pub fn set_difficulty(origin: OriginFor<T>, new_difficulty: u32) -> DispatchResult {
            ensure_root(origin)?;
            <Difficulty<T>>::put(new_difficulty);
            Self::deposit_event(Event::DifficultyAdjustment { new_difficulty });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn calculate_reward(cube_size: u32) -> u32 {
            // Reward based on cube size and difficulty
            let base_reward = 1000;
            base_reward * cube_size
        }

        fn adjust_difficulty(current_difficulty: u32, cube_size: u32) -> u32 {
            // Simple difficulty adjustment based on cube size
            // In a real implementation, this would be based on block time
            let adjustment_factor = (cube_size * 100) / (current_difficulty.max(1));
            current_difficulty.saturating_add(adjustment_factor)
        }
    }
}