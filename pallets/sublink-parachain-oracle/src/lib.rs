#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*
	};
	use pallet_chainlink_feed::pallet::{ FeedOracle, RoundData, FeedInterface, RoundId, MutableFeedInterface };
	use sublink_xcm::{FeedRequester, FeedReceiver, FeedId, FeedValue, RoundDataOf};

	use frame_support::{
		dispatch::DispatchResult,
		require_transactional,
	};
	use sp_runtime::traits::Zero;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + sublink_xcm::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// Need for types :(
		type Oracle: FeedOracle<Self>;

		type FeedRequester: FeedRequester<Self>;
	}


	/// The configuration for an oracle feed.
	#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	pub struct FeedConfig{
		/// Represents the number of decimals with which the feed is configured
		pub decimals: u8,
		/// The id of the latest round
		pub latest_round: RoundId,
		/// The id of the first round that contains non-default data
		pub first_valid_round: Option<RoundId>,
	}

	pub struct Feed<T: Config> {
		pub id: FeedId<T>,
		pub config: FeedConfig
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn feed_config)]
	pub type Feeds<T: Config> =
		StorageMap<_, Twox64Concat, FeedId<T>, FeedConfig, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn feed_value)]
	pub type Rounds<T: Config> =
		StorageMap<_, Twox64Concat, FeedId<T>, RoundData<T::BlockNumber, FeedValue<T>>, OptionQuery>;
	
	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	// #[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		NotImplemented,
	}


	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
	}

	impl<T: Config> Feed<T> {
		// --- constructors ---

		/// Create a new feed with the given id and config.
		/// Will store the config when dropped.
		pub fn new(id: FeedId<T>, config: FeedConfig) -> Self {
			Self {
				id,
				config,
			}
		}

		/// Load the feed with the given id for reading.
		/// Will not store the config when dropped.
		/// -> Don't mutate the feed object.
		pub fn read_only_from(id: FeedId<T>) -> Option<Self> {
			let config = Feeds::<T>::get(id.clone())?;
			Some(Self {
				id,
				config,
			})
		}

	}

	impl<T: Config> FeedOracle<T> for Pallet<T> {
		type FeedId = FeedId<T>;
		type Feed = Feed<T>;
		type MutableFeed = Feed<T>;

		/// Return a transient feed proxy object for interacting with the feed
		/// given by the id. Provides read-only access.
		fn feed(id: Self::FeedId) -> Option<Self::Feed> {
			let feed = Feed::read_only_from(id.clone());
			if feed.is_none(){
				T::FeedRequester::request_latest_data(id);
			}
			feed
		}

		/// Return a transient feed proxy object for interacting with the feed
		/// given by the id. Provides read-write access.
		fn feed_mut(_id: Self::FeedId) -> Option<Self::MutableFeed> {
			None
		}
	}


	impl<T: Config> FeedInterface<T> for Feed<T> {
		type Value = FeedValue<T>;

		/// Returns the id of the first round that contains non-default data.
		fn first_valid_round(&self) -> Option<RoundId> {
			self.config.first_valid_round
		}

		/// Returns the id of the latest oracle round.
		fn latest_round(&self) -> RoundId {
			self.config.latest_round
		}

		/// Returns the data for a given round.
		fn data_at(&self, _round: RoundId) -> Option<RoundData<T::BlockNumber, FeedValue<T>>> {
			None
		}

		/// Returns the latest data for the feed.
		fn latest_data(&self) -> RoundData<T::BlockNumber, FeedValue<T>>  {
			Rounds::<T>::get(self.id.clone()).unwrap_or_else(|| {
				log::info!("***** Sublink Parachain Oracle no round for {:?}", self.id);
				RoundData {
					started_at: T::BlockNumber::default(),
					answer: 0.into(),
					updated_at: T::BlockNumber::default(),
					answered_in_round: 0
				}
			})
		}

		/// Returns the configured decimals
		fn decimals(&self) -> u8 {
			self.config.decimals
		}
	}

	impl<T: Config> MutableFeedInterface<T> for Feed<T> {
		/// Requests that a new round be started for the feed.
		///
		/// Returns `Ok` on success and `Err` in case the round could not be
		/// started.
		#[require_transactional]
		fn request_new_round(&mut self, _requester: T::AccountId) -> DispatchResult {
			Err(Error::<T>::NotImplemented.into())
		}
	}

	impl<T: Config> FeedReceiver<T> for Pallet<T> {
		fn receive_latest_data(feed_id: FeedId<T>, latest_round_data: RoundDataOf<T>) {
			log::info!("***** Sublink Parachain Oracle receiving {:?}={:?}", feed_id, latest_round_data);
			let feed = Feed::<T>::read_only_from(feed_id.clone());
			if feed.is_none(){
				let new_config = FeedConfig {
					decimals: 3, 
					latest_round: Zero::zero(),
					first_valid_round: None,
				};
				log::info!("***** Sublink Parachain Oracle inserting new feed");
				Feeds::<T>::insert(feed_id.clone(), &new_config);
			}
			let current_block_number = <frame_system::Pallet<T>>::block_number();
			// TODO LTK : started_at is relative to sublink parachain block numbers
			// How to convert to the receiver parachain block ?
			let new_round = RoundData {
				started_at: latest_round_data.started_at,
				answer: latest_round_data.answer,
				updated_at: current_block_number,
				answered_in_round: latest_round_data.answered_in_round
			};			
			log::info!("***** Sublink Parachain Oracle inserting new round");
			Rounds::<T>::insert(feed_id.clone(), &new_round);
		}
	}	
}
