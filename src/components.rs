use specs::prelude::*;
use specs_derive::*;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
	pub x : i32, //32 bit integers
	pub y : i32,
}

#[derive(Component)]
pub struct Renderable {
	pub glyph: rltk::FontCharType,
	pub fg: RGB,
	pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player{}


#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component, Debug)]
pub struct Name{
	pub name: String
}

#[derive(Component, Debug)]
pub struct BlocksTile {}