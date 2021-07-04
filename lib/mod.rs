#[macro_use]
mod policies;
#[macro_use]
mod opengl;
mod utility;

pub mod uses {
	pub use hashbrown::{HashMap, HashSet};
	pub use std::borrow::{self, Borrow, ToOwned};
	pub use std::collections::{BTreeMap, BTreeSet, VecDeque};
	pub use std::marker::PhantomData as Dummy;
	pub use std::{cell::Cell, cell::UnsafeCell, char, cmp, convert::TryInto, fmt, fmt::Debug, hash, iter, mem, ops, path, path::Path, ptr, rc::Rc, rc::Weak, slice, time};
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
		pub use smol::{prelude::Future, Task};
		pub mod task {
			pub use smol::{block_on, future::poll_once, spawn};
		}
		pub mod pre {
			pub use smol::{prelude::*, unblock, Unblock};
		}
		pub use smol::{fs, io};
	}
	mod sync_pre {
		pub use chan::{Receiver, Sender};
		pub use flume as chan;
		pub use std::sync::{atomic::*, Arc, Barrier, Mutex, Once};
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
	pub use super::policies::{casts::cast::Cast, math::*, rand, type_tools, unsafe_static::*};
	pub use super::utility::{cached_str::CachedStr, ext::*, prefetch, slicing};
	pub use super::{GL, GL::types::*};
	pub use {bitflags::bitflags, const_format, num_cpus};
	pub use {nalgebra as na, nalgebra_glm as glm};
}

pub mod events {
	pub use super::policies::events::*;
}

pub mod GL {
	pub use super::{opengl::opengl::*, policies::window};
}
