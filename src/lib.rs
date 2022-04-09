#![allow(non_snake_case, dead_code)]

pub mod components;
pub mod size;
pub mod color;
pub mod constants;

pub mod prelude {
    pub use crate::size::*;
    pub use crate::color::*;
    pub use crate::constants::*;

    pub use crate::components::button::*;
    pub use crate::components::checkbox::*;
    pub use crate::components::radio::*;
    pub use crate::components::switch::*;
    pub use crate::components::number_input::*;
    pub use crate::components::text_input::*;
    pub use crate::components::rate::*;
    pub use crate::components::list::*;
    pub use crate::components::divider::*;
}
