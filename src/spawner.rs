use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats,Player,Renderable,Name,Position,Viewshed,Monster,BlocksTile, map::MAPWIDTH, Rect};


//control ratio of monsters to items
const MAX_MONSTERS: i32 = 4;
const MAX_ITEMS: i32 = 2;

//move spawning of entities to separate file to include items monsters and player
//spawns player and returns entity object
pub fn player(ecs: &mut World, player_x: i32, player_y: i32) -> Entity {
	ecs
		.create_entity()
		.with(Position { x: player_x, y: player_y})
		.with(Renderable{
			glyph: rltk::to_cp437('@'),
			fg: RGB::named(rltk::PURPLE),
			bg: RGB::named(rltk::BLACK),
		})
		.with(Player{})
		.with(CombatStats{max_hp: 100, hp: 100, defense: 2, power: 20})
		.with(Viewshed{visible_tiles:Vec::new(),range: 6, dirty: true}) //change hard code later
		.with(Name{name: "Player".to_string()})
		.build()
}

//spawn a random monster at a given location
pub fn random_monster(ecs: &mut World, x: i32, y:i32){
	let roll: i32;

	{
		let mut rng = rltk::RandomNumberGenerator::new();
		roll = rng.roll_dice(1,2);
	} 

	match roll {
		1 => {orc(ecs,x,y)}
		_ => {goblin(ecs,x,y)}
	}
}

fn orc(ecs: &mut World, x:i32, y:i32){ 
	monster(ecs, x, y, rltk::to_cp437('o'), "Orc"); 
}

fn goblin(ecs: &mut World, x:i32, y:i32){
	monster(ecs,x,y, rltk::to_cp437('g'), "Goblin");
}

fn monster<S : ToString>(ecs: &mut World, x:i32, y:i32, glyph: rltk::FontCharType, name: S){
	ecs
			.create_entity()
			.with(Position{x,y})
			.with(Renderable{
				glyph,
				fg: RGB::named(rltk::RED),
				bg: RGB::named(rltk::BLACK),
			})
			.with(Viewshed{visible_tiles:Vec::new(),range:6,dirty:true})
			.with(Monster{})
			.with(CombatStats{max_hp: 13, hp: 13, defense: 1, power: 4})
			.with(Name{ name: name.to_string()})
			.with(BlocksTile{})
			.build();
}

//fills a room with items or monsters
pub fn spawn_room(ecs: &mut World, room: &Rect){
	let mut monster_spawn_points: Vec<usize> =  Vec::new();

	//scope to keep the borrow checker happy - mutable rng then passing ecs, drops rng when passed to ecs
	{
		//seeding another rng, will this end up being a problem later because of randgenerator not in scope, rltk problem
		let mut rng = rltk::RandomNumberGenerator::new();
		let num_monsters = rng.roll_dice(1, MAX_MONSTERS+2) -  3;

		for _i in 0..num_monsters {
			let mut added = false;

			while !added{
				let x = (room.x1 + rng.roll_dice(1, i32::abs(room.x2 - room.x1))) as usize;
				let y = (room.y1 + rng.roll_dice(1, i32::abs(room.y2 - room.y1))) as usize;
				let idx = (y * MAPWIDTH) + x;
				if !monster_spawn_points.contains(&idx) {
					monster_spawn_points.push(idx);
					added = true;
				}
			}
		}
	}

	//spawn monster
	for idx in monster_spawn_points.iter(){
		let x = *idx % MAPWIDTH;
		let y = *idx / MAPWIDTH;
		random_monster(ecs, x as i32, y as i32);
	}
}