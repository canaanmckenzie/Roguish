use rltk::{Rltk,GameState,RGB};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;

#[derive(Component)]
struct Position {
	x : i32, //32 bit integers
	y : i32,
}

#[derive(Component)]
struct Renderable {
	glyph: rltk::FontCharType,
	fg: RGB,
	bg: RGB,
}

struct State{
	ecs: World
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk){
		ctx.cls();
		ctx.print(1,1, "Canaan's rogue")
	}
}

fn main() -> rltk::BError {

	use rltk::RltkBuilder;
	let context = RltkBuilder::simple80x50()
		.with_title("Roguish")
		.build()?;

	let mut gs = State{

		ecs: World::new()
	};
	gs.ecs.register::<Position>();
	gs.ecs.register::<Renderable>();


	rltk::main_loop(context, gs)
}

