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
	pub use std::sync::{atomic::*, OnceLock};
	pub use std::thread::{self, JoinHandle};
}
#[cfg(not(feature = "adv_fs"))]
pub mod ser {}
#[cfg(feature = "adv_fs")]
pub mod ser {
	pub mod SERDE {
		pub use bincode::{deserialize as FromVec, serialize as ToVec};
		pub use serde_json::{from_str as FromStr, to_string as ToStr, to_vec as ToU8};
	}
	pub use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
	pub use std::fmt;
}
pub mod stdlib {
	pub use std::cell::{Cell, RefCell, UnsafeCell};
	pub use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};
	pub use std::{borrow::Borrow, char, fmt, fmt::Debug, iter, mem, ops, ops::Range, ptr, rc::Rc, rc::Weak, slice, sync::Arc, time};
	pub use std::{cmp::Ordering as ord, marker::PhantomData as Dummy, mem::size_of as type_size};
}
pub mod lib {
	pub use super::{policies::ext::*, policies::pre::*, stdlib::*, GL, GL::types::*};
	pub use {bitflags::bitflags, const_format, num_cpus};
}
pub mod math {
	pub use super::policies::math::{ext::*, la, la::na};
}
pub mod GL {
	pub use super::opengl::{event, pre::*, window};
}

pub use policies::ext::*;

#[macro_use]
mod policies;
#[macro_use]
mod opengl;
