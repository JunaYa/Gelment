#![allow(non_snake_case, dead_code)]

pub mod components;
pub mod size;
pub mod color;

pub mod prelude {
    pub use crate::size::*;
    pub use crate::color::*;

    pub use crate::components::button::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::radio::*;
    pub use crate::components::divider::*;
}
