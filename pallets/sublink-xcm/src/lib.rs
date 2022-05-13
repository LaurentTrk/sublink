#![cfg_attr(not(feature = "std"), no_std)]

use cumulus_pallet_xcm::{ensure_sibling_para, Origin as CumulusOrigin};
use cumulus_primitives_core::ParaId;
use frame_system::Config as SystemConfig;
use sp_std::prelude::*;
use xcm::latest::prelude::*;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use pallet_chainlink_feed::pallet::{ FeedOracle, RoundData, FeedInterface };

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// The module configuration trait.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Origin: From<<Self as SystemConfig>::Origin>
			+ Into<Result<CumulusOrigin, <Self as Config>::Origin>>;

		/// The overarching call type; we assume sibling chains use the same type.
		type Call: From<Call<Self>> + Encode;

		type XcmSender: SendXcm;

		type Oracle: FeedOracle<Self>;
	}

	/// The total number of pings sent.
	#[pallet::storage]
	pub(super) type SubLinkParaId<T: Config> = StorageValue<_, ParaId, ValueQuery>;

	/// The target parachains to ping.
	#[pallet::storage]
	pub(super) type Targets<T: Config> = StorageValue<_, Vec<(ParaId, Vec<u8>)>, ValueQuery>;

	/// The total number of pings sent.
	#[pallet::storage]
	pub(super) type PingCount<T: Config> = StorageValue<_, u32, ValueQuery>;

	/// The sent pings.
	#[pallet::storage]
	pub(super) type Pings<T: Config> =
		StorageMap<_, Blake2_128Concat, u32, T::BlockNumber, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		SendRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		ReceiveRequestForLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		SendLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId, <<T::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value),
		ReceiveLatestPriceValue(ParaId, <T::Oracle as FeedOracle<T>>::FeedId, <<T::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value),
		ErrorSendingRequest(SendError, ParaId, <T::Oracle as FeedOracle<T>>::FeedId),
		ErrorSendingLatestPriceValue(SendError, ParaId, <T::Oracle as FeedOracle<T>>::FeedId, <<T::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value),
	}

	#[pallet::error]
	pub enum Error<T> {
		FeedMissing,		
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		// LTK : send updated price value to registered parachains ?

		// fn on_finalize(n: T::BlockNumber) {
		// 	for (para, payload) in Targets::<T>::get().into_iter() {
		// 		let seq = PingCount::<T>::mutate(|seq| {
		// 			*seq += 1;
		// 			*seq
		// 		});
		// 		match T::XcmSender::send_xcm(
		// 			(1, Junction::Parachain(para.into())),
		// 			Xcm(vec![Transact {
		// 				origin_type: OriginKind::Native,
		// 				require_weight_at_most: 1_000,
		// 				call: <T as Config>::Call::from(Call::<T>::ping {
		// 					seq,
		// 					payload: payload.clone(),
		// 				})
		// 				.encode()
		// 				.into(),
		// 			}]),
		// 		) {
		// 			Ok(()) => {
		// 				Pings::<T>::insert(seq, n);
		// 				Self::deposit_event(Event::PingSent(para, seq, payload));
		// 			},
		// 			Err(e) => {
		// 				Self::deposit_event(Event::ErrorSendingPing(e, para, seq, payload));
		// 			},
		// 		}
		// 	}
		// }
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {

		#[pallet::weight(0)]
		pub fn ask_latest_data(_origin: OriginFor<T>, feed_id: <T::Oracle as FeedOracle<T>>::FeedId) -> DispatchResult {
			log::info!("***** Sublink XCM ask_latest_data called");

			// TODO LTK : need to register SubLink parachain id
			let para: ParaId = 2000.into();
			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::Native,
					require_weight_at_most: 1_000,
					call: <T as Config>::Call::from(Call::<T>::get_latest_data {
						feed_id: feed_id.clone(),
					})
					.encode()
					.into(),
				}]),
			) {
				Ok(()) => {
					log::info!("***** Sublink XCM ask_latest_data called get_latest_data");
					Self::deposit_event(Event::SendRequestForLatestPriceValue(para, feed_id))
				},
				Err(e) => {
					log::error!("***** Sublink XCM ask_latest_data cannot called get_latest_data");
					Self::deposit_event(Event::ErrorSendingRequest(e, para, feed_id))
				},
			}
			Ok(())
		}

		#[pallet::weight(0)]
		pub fn get_latest_data(origin: OriginFor<T>, feed_id: <T::Oracle as FeedOracle<T>>::FeedId) -> DispatchResult {
			log::info!("***** Sublink XCM get_latest_data called");
			// Only accept pings from other chains.
			let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			Self::deposit_event(Event::ReceiveRequestForLatestPriceValue(para, feed_id.clone()));
			let feed = T::Oracle::feed(feed_id.clone()).ok_or(Error::<T>::FeedMissing) ?;
			let RoundData { answer,..} = feed.latest_data();

			log::info!("***** Sublink XCM latest_value = {:?}", answer.clone());

			match T::XcmSender::send_xcm(
				(1, Junction::Parachain(para.into())),
				Xcm(vec![Transact {
					origin_type: OriginKind::Native,
					require_weight_at_most: 1_000,
					call: <T as Config>::Call::from(Call::<T>::store_latest_data {
						feed_id: feed_id.clone(),
						latest_value: answer.clone()
					})
					.encode()
					.into(),
				}]),
			) {
				Ok(()) => {
					log::info!("***** Sublink XCM get_latest_data called store_latest_data");
					Self::deposit_event(Event::SendLatestPriceValue(para, feed_id, answer))
				},
				Err(e) => {
					log::error!("***** Sublink XCM get_latest_data cannot called store_latest_data");
					Self::deposit_event(Event::ErrorSendingLatestPriceValue(e, para, feed_id, answer))
				},
			}
			Ok(())

		}

		#[pallet::weight(0)]
		pub fn store_latest_data(origin: OriginFor<T>, feed_id: <T::Oracle as FeedOracle<T>>::FeedId, latest_value: <<T::Oracle as FeedOracle<T>>::Feed as FeedInterface<T>>::Value) -> DispatchResult {
			log::info!("***** Sublink XCM store_latest_data called");
			// Only accept pings from other chains.
			let para = ensure_sibling_para(<T as Config>::Origin::from(origin))?;

			log::info!("***** Sublink XCM Received latest_value = {:?}", latest_value.clone());
			Self::deposit_event(Event::ReceiveLatestPriceValue(para, feed_id, latest_value));
			Ok(())

		}
	}
}