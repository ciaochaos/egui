#![deny(warnings)]

extern crate rusttype;
extern crate serde;

#[macro_use] // TODO: get rid of this
extern crate serde_derive;

mod emigui;
mod font;
mod layout;
pub mod math;
mod painter;
mod style;
mod texture_atlas;
pub mod types;
pub mod widgets;

pub use crate::{
    emigui::Emigui,
    font::Font,
    layout::LayoutOptions,
    layout::Region,
    painter::{Frame, Painter, Vertex},
    style::Style,
    texture_atlas::TextureAtlas,
    types::RawInput,
};
