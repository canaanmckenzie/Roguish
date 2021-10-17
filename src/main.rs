use rltk::{Rltk,GameState,RGB, VirtualKeyCode,Point};
use specs::prelude::*;
//use std::cmp::{max,min};
//use specs_derive::Component;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
pub use visibility_system::VisibilitySystem;
mod monster_ai_system;
pub use monster_ai_system::MonsterAI;


#[derive(PartialEq, Copy,Clone)]
pub enum RunState {Paused, Running}

pub struct State{
	pub ecs: World,
	pub runstate: RunState
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk){
		ctx.cls();

		player_input(self,ctx);


		//add runstate conditions for pausing
		if self.runstate == RunState::Running{
			self.run_system();
			self.runstate = RunState::Paused;
		} else {
			self.runstate = player_input(self,ctx);
		}

		//let map = self.ecs.fetch::<Vec<TileType>>();
		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();



		for (pos, render) in (&positions, &renderables).join(){
			let idx =  map.xy_idx(pos.x,pos.y);

			if map.visible_tiles[idx] {ctx.set(pos.x,pos.y,render.fg,render.bg, render.glyph)};
		}
	}
}


impl State{
	fn run_system(&mut self){
		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
		let mut mob = MonsterAI{};
		mob.run_now(&self.ecs);
		self.ecs.maintain();
	}
}

fn main() -> rltk::BError {
	use rltk::RltkBuilder;
	let context = RltkBuilder::simple80x50()
		.with_title("Roguish")
		.build()?;

	let mut gs = State{
		ecs: World::new(),
		runstate: RunState::Running
	};

	gs.ecs.register::<Position>();
	gs.ecs.register::<Renderable>();
	gs.ecs.register::<Player>();
	gs.ecs.register::<Viewshed>();
	gs.ecs.register::<Monster>();
	gs.ecs.register::<Name>();

	//let (rooms, map) = new_map_rooms_and_corridors();
	let map: Map = Map::new_map_rooms_and_corridors();
	let (player_x, player_y) = map.rooms[0].center();

	let mut rng = rltk::RandomNumberGenerator::new();

	for room in map.rooms.iter().skip(1){

		let (x,y) = room.center();
		let glyph : rltk::FontCharType;
		let fg : rltk::RGB;
		let name : String;

		let roll = rng.roll_dice(1,5);
		match roll {
			1 => {glyph = rltk::to_cp437('♥'); fg =  RGB::named(rltk::RED); name = "Heart Henchman".to_string();}
			2 => {glyph = rltk::to_cp437('♣'); fg = RGB::named(rltk::GREEN); name = "Club Combatant".to_string();}
			3 => {glyph = rltk::to_cp437('♠'); fg = RGB::named(rltk::BLUE); name = "Spade Soldier".to_string();}
			_ => {glyph = rltk::to_cp437('♦'); fg = RGB::named(rltk::YELLOW); name = "Diamond Duelist".to_string();}
		}
		gs.ecs
			.create_entity()
			.with(Position{x,y})
			.with(Renderable{
				glyph: glyph,
				fg: fg,
				bg: RGB::named(rltk::BLACK),
			})
			.with(Viewshed{visible_tiles:Vec::new(),range:6,dirty:true})
			.with(Monster{})
			.with(Name{ name: format!("{}",&name)})
			.build();
	}

	gs.ecs.insert(map);

		gs.ecs
			.create_entity()
			.with(Position { x: player_x, y: player_y})
			.with(Renderable{
				glyph: rltk::to_cp437('@'),
				fg: RGB::named(rltk::PURPLE),
				bg: RGB::named(rltk::BLACK),
			})
			.with(Player{})
			.with(Viewshed{visible_tiles:Vec::new(),range: 6, dirty: true}) //change hard code later
			.with(Name{name: "Player".to_string()})
			.build();

	gs.ecs.insert(Point::new(player_x,player_y));
	rltk::main_loop(context, gs)
}

