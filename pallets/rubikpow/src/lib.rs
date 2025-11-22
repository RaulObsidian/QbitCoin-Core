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

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        BlockMined { miner: T::AccountId, cube_size: u32 },
        Reward { miner: T::AccountId, amount: u32 },
    }

    #[pallet::error]
    pub enum Error<T> {
        InvalidSolution,
        CubeTooSmall,
        CubeTooLarge,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(10_000)] // TODO: adjust weight
        pub fn submit_solution(
            origin: OriginFor<T>,
            cube_size: u32,
            scramble: Vec<Move>,
            solution: Vec<Move>,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            ensure!(cube_size >= 3, Error::<T>::CubeTooSmall);
            ensure!(cube_size <= 16, Error::<T>::CubeTooLarge); // Limit cube size for performance

            let mut cube = Cube::new(cube_size as usize);
            for m in scramble.iter() {
                cube.apply_move(m);
            }

            ensure!(cube.verify_solution(&solution), Error::<T>::InvalidSolution);

            let difficulty = calculate_difficulty(cube_size as usize);
            let reward = difficulty * 10; // Example reward calculation

            // Issue reward
            // This is a simplified example. A real implementation would use a proper currency system.
            // For now, we just deposit an event.
            Self::deposit_event(Event::BlockMined { miner: who.clone(), cube_size });
            Self::deposit_event(Event::Reward { miner: who, amount: reward });

            Ok(())
        }
    }
}