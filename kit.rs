pub mod sync {
	pub use std::sync::mpsc::{Receiver, SyncSender as Sender};
	pub mod chan {
		pub use std::sync::mpsc::sync_channel as bounded;
	}
}
pub mod asyn {
	pub mod chan {
		pub use tokio::sync::mpsc::unbounded_channel as unbounded;
	}
	pub use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
}
mod sync_pre {
	pub mod arc {
		pub use std::sync::Weak;
	}
	pub mod task {
		pub async fn sleep_ms(ms: u64) {
			sleep(std::time::Duration::from_millis(ms)).await
	}
		pub use super::super::policies::task::{GLRuntime, Runtime};
		pub use futures_lite::future::block_on;
		pub use tokio::{sync::Mutex, task::*, time::sleep};
	}
	pub use super::policies::task::{Task, pre::*};
	pub use parking_lot::{Mutex, MutexGuard, RwLock, RwLockUpgradableReadGuard};
	pub use std::sync::{Arc, Barrier, OnceLock, atomic::*};
	pub use std::thread::{self, JoinHandle};
}
pub mod stdlib {
	pub use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
	pub use std::fmt::{Debug, Display, Formatter, Result as fmtRes};
	pub use std::{borrow::Borrow, cell::Cell, iter, mem, ops, ptr, rc, rc::Rc, time};
	pub use std::{cmp::Ordering as ord, marker::PhantomData as Dummy, mem::size_of as type_size};
}
pub mod lib {
	pub use super::{GL, GL::types::*, policies::ext::*, policies::pre::*, stdlib::*, sync_pre::*};
	#[cfg(feature = "adv_fs")]
	pub use serde;
	pub use {bitflags::bitflags, grafix_toolbox_macros::*};
}
pub mod math {
	pub use super::policies::math::{ext::*, la, la::na};
}
pub mod GL {
	pub use super::opengl::{event, pre::*, window};
}
pub mod text_color {
	pub mod term {
		pub mod term_color {
			pub use yansi::disable;
			pub use yansi::enable;
		}
		pub use yansi::Paint as text_color_prelude;
	}
}

pub use policies::ext::*;

#[macro_use]
mod policies;
#[macro_use]
mod opengl;
