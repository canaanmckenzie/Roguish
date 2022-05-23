use rltk::{RGB, Rltk, Console};
use specs::prelude::*;
use super::{CombatStats, Player, GameLog};

//add box below map for user input
pub fn draw_ui(ecs: &World, ctx: &mut Rltk){
	ctx.draw_box(0,43,79,6, RGB::named(rltk::GREY), RGB::named(rltk::BLACK));

	let combat_stats = ecs.read_storage::<CombatStats>();

	let players = ecs.read_storage::<Player>();

	//use ctx to print player health and update health bar at each tick
	for(_player, stats) in (&players, &combat_stats).join(){
		let health = format!("HP: {}/{}",stats.hp,stats.max_hp);
		ctx.print_color(12,43, RGB::named(rltk::PURPLE), RGB::named(rltk::BLACK), &health);
		ctx.draw_bar_horizontal(28,43,51,stats.hp,stats.max_hp,RGB::named(rltk::RED), RGB::named(rltk::BLACK));
	}

	let log = ecs.fetch::<GameLog>();
	let mut y = 45;
	for s in log.entries.iter().rev(){
		if y < 49 {ctx.print(2, y , s);}
		y +=1;
	}
}