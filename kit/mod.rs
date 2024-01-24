#[macro_use]
mod policies;
#[macro_use]
mod opengl;

pub mod sync {
	pub use super::sync_pre::*;
	pub use chan::{Receiver, Sender};
	pub use crossbeam_channel as chan;
	pub use std::sync::{Barrier, Mutex};
	pub use std::{fs, io};
}
pub mod asyn {
	pub use super::sync_pre::*;
	pub use chan::{Receiver, Sender};
	pub use flume as chan;
	pub use smol::lock::{Barrier, Mutex};
	pub use smol::Unblock;
	pub use smol::{fs, io};
}
mod sync_pre {
	pub mod task {
		pub use smol::{block_on, future::poll_once, spawn, Timer};
	}
	pub use smol::{future, prelude::*, stream, Task};
	pub use std::sync::{atomic::*, Arc, OnceLock};
	pub use std::thread::{self, JoinHandle};
}
#[cfg(feature = "adv_fs")]
pub mod ser {
	pub mod SERDE {
		pub use bincode::{deserialize as FromVec, serialize as ToVec};
		pub use serde_json::{from_str as FromStr, to_string as ToStr};
	}
	pub use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
	pub use std::fmt;
}
pub mod stdlib {
	pub use std::cell::{Cell, RefCell, UnsafeCell};
	pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
	pub use std::{char, fmt::Debug, iter, mem, ops::Range, ptr, rc::Rc, rc::Weak, slice, time};
	pub use std::{cmp::Ordering as ord, marker::PhantomData as Dummy};
}
pub mod lib {
	pub use super::policies::ext::{cached::*, cached_str::*, n_iter::*, *};
	pub use super::policies::{chksum, derives::*, index::*, logging, math::*, pointer::*, result::*, type_tools};
	pub use super::{stdlib::*, GL::types::*};
	pub use {bitflags::bitflags, const_format, num_cpus};
}
pub mod math {
	pub use super::policies::math::{ext::*, la::na, tuple::*, *};
}
pub mod GL {
	pub use super::opengl::opengl::*;
	pub use super::policies::window;
}

pub use policies::{event, ext::lazy, ext::prefetch, file as FS, logging, profiling, rand, slicing};
