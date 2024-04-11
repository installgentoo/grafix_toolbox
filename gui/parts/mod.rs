pub use {frame9::*, obj::*, rect::*, sprite::*, sprite9::*, string::*};

pub fn hex_to_rgba(c: u32) -> Color {
	Color::to(((c & 0xff000000) >> 24, (c & 0xff0000) >> 16, (c & 0xff00) >> 8, c & 0xff)).div(255)
}

mod frame9;
mod obj;
mod rect;
mod sprite;
mod sprite9;
mod string;

use super::*;
