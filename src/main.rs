use rltk::{Rltk,GameState,RGB, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max,min};
use specs_derive::Component;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;


pub struct State{
	pub ecs: World
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk){
		ctx.cls();


		player_input(self,ctx);
		self.run_system();

		let map = self.ecs.fetch::<Vec<TileType>>();
		draw_map(&map, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();


		for (pos, render) in (&positions, &renderables).join(){
			ctx.set(pos.x,pos.y,render.fg,render.bg, render.glyph);
		}
	}
}

impl State{
	fn run_system(&mut self){
	
		self.ecs.maintain();
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
	//gs.ecs.register::<LeftMover>();
	gs.ecs.register::<Player>();
	gs.ecs.insert(new_map_rooms_and_corridors());


		gs.ecs
			.create_entity()
			.with(Position{ x: 40, y: 25})
			.with(Renderable{
				glyph: rltk::to_cp437('♦'),
				fg: RGB::named(rltk::YELLOW),
				bg: RGB::named(rltk::BLACK),
			})
			.with(Player{})
			.build();

	rltk::main_loop(context, gs)
}

