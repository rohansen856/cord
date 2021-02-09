// Copyright 2019-2021 Dhiway.
// This file is part of CORD Platform.

// derived from kilt project

//! #MARK Types: Handles #MARK Types,
//! adding #MARK Types.
#![cfg_attr(not(feature = "std"), no_std)]

/// Test module for MTYPEs
#[cfg(test)]
mod tests;

use frame_support::{
	debug, decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, 
	ensure, StorageMap,
};
use frame_system::{self, ensure_signed};

/// The #MARK TYPE trait
pub trait Trait: frame_system::Config {
	/// #MARK TYPE specific event type
	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_event!(
	/// Events for #MARK TYPEs
	pub enum Event<T> where <T as frame_system::Config>::AccountId, <T as frame_system::Config>::Hash {
		/// A new #MARK Schema has been anchored
		MTypeAnchored(AccountId, Hash),
	}
);

// The pallet's errors
decl_error! {
	pub enum Error for Module<T: Trait> {
		NotFound,
		AlreadyExists,
	}
}

decl_module! {
	/// The #MARK TYPE runtime module
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {

		/// Deposit events
		fn deposit_event() = default;
		
		// Initializing errors
		// this includes information about your errors in the node's metadata.
		// it is needed only if you are using errors in your pallet
		type Error = Error<T>;

		/// Anchors a new #MARK TYPE on chain
		/// origin is the signed sender account, and
		/// hash is the hash of the anchored Type Schema
		#[weight = 1]
		pub fn anchor(origin, hash: T::Hash) -> DispatchResult {
			// origin of the transaction needs to be a signed sender account
			let sender = ensure_signed(origin)?;

			// check if #MARK TYPE already exists
			ensure!(!<MTYPEs<T>>::contains_key(hash), Error::<T>::AlreadyExists);

			// Anchors a new #MARK Schema
			debug::print!("insert MTYPE");
			<MTYPEs<T>>::insert(hash, sender.clone());
			// deposit event - #MARK TYPE has been anchored
			Self::deposit_event(RawEvent::MTypeAnchored(sender, hash));
			Ok(())
		}
	}
}

decl_storage! {
	trait Store for Module<T: Trait> as Mtype {
		// MTYPEs: mtype-hash -> account-id?
		pub MTYPEs get(fn mtypes):map hasher(opaque_blake2_256) T::Hash => Option<T::AccountId>;
	}
}
