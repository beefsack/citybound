#![feature(plugin)]
#![plugin(clippy)]
#![feature(core_intrinsics)]
#![feature(optin_builtin_traits)]
#![feature(specialization)]
#![feature(conservative_impl_trait)]
#![feature(box_syntax)]
#![allow(no_effect)]
extern crate random;
mod pointer_to_maybe_compact;
mod allocators;
#[macro_use]
mod compact;
mod compact_vec;
mod compact_dict;
//mod compact_sorted_dict;
mod chunked;
mod inbox;
mod slot_map;
mod swarm;
#[macro_use]
mod messaging;
mod type_registry;
mod actor_system;
pub use self::compact::{Compact};
pub use self::compact_vec::{CompactVec as CVec};
pub use self::compact_dict::{CompactDict as CDict};
pub use self::chunked::{MemChunker};
pub use self::swarm::{Swarm, Create, CreateWith, RecipientAsSwarm, ToRandom, RequestConfirmation, Confirmation};
pub use self::inbox::{Inbox};
pub use self::messaging::{Message, Packet, Actor, Recipient, Individual, Fate};
pub use self::actor_system::{THE_SYSTEM, ID, ActorSystem};