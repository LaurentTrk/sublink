//! Chainlink Chain Extension
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use frame_support::{
	log::{error},
};
pub use frame_support::dispatch::DispatchError;
use frame_support::dispatch::Encode;
use log;

use pallet_contracts::chain_extension::{
    RetVal, ChainExtension, Environment, Ext, InitState, SysConfig, UncheckedFrom,
};

use pallet_chainlink_feed::{FeedOracle, FeedInterface, RoundData};

/// The chain Extension for ChainLink
pub struct ChainlinkExtension<Runtime>(sp_std::marker::PhantomData<Runtime>);


impl<Runtime> ChainExtension<Runtime> for ChainlinkExtension<Runtime> 
where   Runtime: pallet_contracts::Config,
        Runtime: pallet_chainlink_feed::Config,
    {
    fn call<E: Ext>(
        func_id: u32,
        env: Environment<E, InitState>,
    ) -> Result<RetVal, DispatchError>
    where
        <E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
    {
        log::info!("***** Chainlink extension called {:?}", func_id);
        match func_id {
            // latest_data by id
            70930000 => {
				let mut env = env.buf_in_buf_out();
				let feed_id: <Runtime as pallet_chainlink_feed::Config>::FeedId =
					env.read_as_unbounded(env.in_len())?;
                let feed = pallet_chainlink_feed::Pallet::<Runtime>::feed(feed_id.into()).unwrap();
                let RoundData { answer,..} = feed.latest_data();
                log::info!("called latest_data extension with feed_id {:?} = {:?}", feed_id, answer);
                let r = answer.encode();
				env.write(&r, false, None).map_err(|_| {
                    log::info!("Error when writing result");
					DispatchError::Other(
						"ChainlinkExtension failed to return result",
					)
				})?;
            }

            _ => {
                error!("Called an unregistered `func_id`: {:}", func_id);
                return Err(DispatchError::Other("Unimplemented func_id"))
            }
        }

        Ok(RetVal::Converging(0))
    }
}
