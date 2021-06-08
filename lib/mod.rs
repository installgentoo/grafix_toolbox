#[macro_use]
mod policies;
#[macro_use]
mod opengl;
#[macro_use]
mod utility;

pub mod uses {
	pub use hashbrown::{HashMap, HashSet};
	pub use std::borrow::{self, Borrow, ToOwned};
	pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
	pub use std::marker::PhantomData as Dummy;
	pub use std::{cell::Cell, cell::UnsafeCell, char, cmp, convert::TryInto, fmt, fmt::Debug, hash, io, iter, mem, ops, path, path::Path, ptr, rc::Rc, rc::Weak, slice, time};
	pub mod ord {
		pub use std::cmp::Ordering::*;
	}
	pub mod Sync {
		pub mod sync {
			pub use std::sync::*;
		}
		pub mod chan {
			pub use chan::{Receiver, Sender};
			pub use std::sync::mpsc as chan;
		}
	}
	pub mod Async {
		pub mod sync {
			pub use smol::lock::*;
			pub use std::sync::{atomic, Once};
		}
		pub mod chan {
			pub use chan::{Receiver, Sender};
			pub use smol::channel as chan;
		}
		pub mod pre {
			pub use smol::{prelude::*, unblock, Unblock};
			pub use std::marker::{Send, Unpin};
		}
		pub mod task {
			pub use smol::Task;
			pub mod task {
				pub use smol::{block_on, future::poll_once, spawn};
			}
		}
		pub use smol::{fs, io};
	}
	pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
	pub mod SERDE {
		pub use bincode::{deserialize as FromVec, serialize as ToVec};
		pub use serde_json::{from_str as FromStr, to_string as ToStr};
	}
	pub mod serde_impl {
		pub use serde::{de::*, ser::*, *};
		pub use std::fmt::Formatter;
		pub use std::fmt::Result as FmtRes;
	}
	pub use super::{
		policies::{adapters, chksum, files as FS, logging},
		utility::profiling,
	};
	pub mod math {
		pub use super::super::utility::tuple::*;
	}
	pub use super::policies::{casts::cast::Cast, math::*, rand, type_tools};
	pub use super::utility::{cached_str::CachedStr, ext::*, prefetch, slicing};
	pub use super::{GL, GL::opengl, GL::types::*};
	pub use bitflags::bitflags;
	pub use nalgebra as na;
	pub use nalgebra_glm as glm;
}

pub mod events {
	pub use super::policies::events::*;
}

pub mod GL {
	pub use super::{opengl::opengl::*, policies::window};
}
