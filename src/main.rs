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
mod map_indexing_system;
pub use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
pub use melee_combat_system::MeleeCombatSystem;
mod damage_system;
pub use damage_system::DamageSystem;
mod gui;
pub use gui::draw_ui;
mod gamelog;
pub use gamelog::GameLog;
mod spawner;
//pub use spawner::player;
//pub use spawner::random_monster;
pub mod inventory_system;
pub use inventory_system::ItemCollectionSystem;



#[derive(PartialEq, Copy,Clone)]
//replace runstate with something more descriptive of each phase
pub enum RunState {AwaitingInput, PreRun, PlayerTurn, MonsterTurn}

pub struct State{
	pub ecs: World,
 //	pub runstate: RunState
}

impl GameState for State {
	fn tick(&mut self, ctx : &mut Rltk){
		ctx.cls();

		player_input(self,ctx);

		let mut newrunstate;
		{
			let runstate = self.ecs.fetch::<RunState>();
			newrunstate = *runstate;
		}

		match newrunstate {
			RunState::PreRun => {
				self.run_system();
				newrunstate = RunState::AwaitingInput;
			}
			RunState::AwaitingInput => {
				newrunstate = player_input(self,ctx);
			}
			RunState::PlayerTurn => {
				self.run_system();
				newrunstate = RunState::MonsterTurn;
			}
			RunState::MonsterTurn => {
				self.run_system();
				newrunstate =  RunState::AwaitingInput;
			}
		}

		{
			let mut runwriter = self.ecs.write_resource::<RunState>();
			*runwriter = newrunstate;
		}

		damage_system::delete_the_dead(&mut self.ecs);

		//let map = self.ecs.fetch::<Vec<TileType>>();
		draw_map(&self.ecs, ctx);

		let positions = self.ecs.read_storage::<Position>();
		let renderables = self.ecs.read_storage::<Renderable>();
		let map = self.ecs.fetch::<Map>();

		for (pos, render) in (&positions, &renderables).join(){
			let idx =  map.xy_idx(pos.x,pos.y);

			if map.visible_tiles[idx] {ctx.set(pos.x,pos.y,render.fg,render.bg, render.glyph)};
		}
		//implement health bar at each tick using ctx from draw_ui
		gui::draw_ui(&self.ecs, ctx);
	}
}


impl State{
	fn run_system(&mut self){
		let mut vis = VisibilitySystem{};
		vis.run_now(&self.ecs);
		let mut mob = MonsterAI{};
		mob.run_now(&self.ecs);
		let mut mapindex = MapIndexingSystem{};
		mapindex.run_now(&self.ecs);
		let mut melee = MeleeCombatSystem{};
		melee.run_now(&self.ecs);
		let mut damage = DamageSystem{};
		damage.run_now(&self.ecs);
		let mut pickup = ItemCollectionSystem{};
		pickup.run_now(&self.ecs);
		self.ecs.maintain();
	}
}

fn main() -> rltk::BError {
	use rltk::RltkBuilder;
	let mut context = RltkBuilder::simple80x50()
		.with_title("Roguish")
		.build()?;
	context.with_post_scanlines(true);
	let mut gs = State{
		ecs: World::new(),
		
	};

	gs.ecs.register::<Position>();
	gs.ecs.register::<Renderable>();
	gs.ecs.register::<Player>();
	gs.ecs.register::<Viewshed>();
	gs.ecs.register::<Monster>();
	gs.ecs.register::<Name>();
	gs.ecs.register::<BlocksTile>();
	gs.ecs.register::<CombatStats>();
	gs.ecs.register::<WantsToMelee>();
	gs.ecs.register::<SufferDamage>();
	gs.ecs.register::<Item>();
	gs.ecs.register::<Potion>();
	gs.ecs.register::<InBackpack>();
	gs.ecs.register::<WantsToPickupItem>();



	//let (rooms, map) = new_map_rooms_and_corridors();
	let map: Map = Map::new_map_rooms_and_corridors();
	let (player_x, player_y) = map.rooms[0].center();

	let mut rng = rltk::RandomNumberGenerator::new();

	let player_entity =  spawner::player(&mut gs.ecs, player_x, player_y);

	for room in map.rooms.iter().skip(1){

		spawner::spawn_room(&mut gs.ecs, room);
		//let (x,y) = room.center();
		//spawner::random_monster(&mut gs.ecs, x,y);
		
}
	//resource inserts - check why gs.ecs.insert cannot insert randnum from rltk error does not exist
	//gs.ecs.insert(rltk::RandomNumberGenerator::new());
	gs.ecs.insert(map);
	gs.ecs.insert(Point::new(player_x,player_y));
	gs.ecs.insert(player_entity);
	gs.ecs.insert(gamelog::GameLog{entries: vec!["Welcome to Roguish".to_string()]});
	gs.ecs.insert(RunState::PreRun);
	rltk::main_loop(context, gs)
}

