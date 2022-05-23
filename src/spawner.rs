use rltk::{RGB, RandomNumberGenerator};
use specs::prelude::*;
use super::{CombatStats,Player,Renderable,Name,Position,Viewshed,Monster,BlocksTile};

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