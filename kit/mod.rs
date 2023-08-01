#[macro_use]
mod policies;
#[macro_use]
mod opengl;
#[macro_use]
mod utility;

pub mod uses {
	pub use hashbrown::{HashMap, HashSet};
	pub use std::borrow::{self, Borrow, ToOwned};
	pub use std::cell::{Cell, RefCell, UnsafeCell};
	pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
	pub use std::marker::PhantomData as Dummy;
	pub use std::{char, cmp, fmt, fmt::Debug, hash, mem, ops, path, path::Path, ptr, rc::Rc, rc::Weak, slice, time};
	pub mod iter {
		pub use std::iter::*;
		pub fn counter() -> impl Iterator<Item = usize> {
			successors(Some(0), |n| Some(n + 1))
		}
	}
	pub mod ord {
		pub use std::cmp::Ordering::*;
	}
	pub mod sync {
		pub use super::sync_pre::*;
		pub use std::thread::{self, JoinHandle};
		pub use std::{fs, io};
	}
	pub mod asyn {
		pub use super::sync_pre::*;
		pub use smol::{future, stream, Task};
		pub mod task {
			pub use smol::{block_on, future::poll_once, spawn};
		}
		pub use smol::{fs, io};
		pub use smol::{prelude::*, unblock, Unblock};
	}
	mod sync_pre {
		pub use chan::{Receiver, Sender};
		pub use flume as chan;
		pub use std::sync::{atomic::*, Arc, Barrier, Mutex, OnceLock};
	}
	#[cfg(feature = "adv_fs")]
	pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
	#[cfg(feature = "adv_fs")]
	pub mod SERDE {
		pub use bincode::{deserialize as FromVec, serialize as ToVec};
		pub use serde_json::{from_str as FromStr, to_string as ToStr};
		pub mod uses {
			pub use serde::*;
			pub use std::fmt::Formatter;
			pub use std::fmt::Result as FmtRes;
		}
	}
	pub use super::{
		policies::{chksum, file as FS, logging},
		utility::profiling,
	};
	pub mod math {
		pub use super::super::utility::tuple::*;
	}
	pub use super::policies::{casts::*, derives::TrivialBound, math::*, rand, type_tools, unsafe_static::*};
	pub use super::utility::{cached::Cached, cached_str::CachedStr, coroutine as coro, ext::*, prefetch, slicing};
	pub use super::{GL, GL::types::*};
	pub use {bitflags::bitflags, const_format, num_cpus, trait_set::trait_set}; //TODO replace with trait aliases
	pub use {nalgebra as na, nalgebra_glm as glm};
}

pub mod event {
	pub use super::policies::event::*;
}

pub mod GL {
	pub use super::{opengl::opengl::*, policies::window};
}
