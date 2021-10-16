use specs::prelude::*;
use super::{Viewshed,Position,Map, Player};
use rltk::{field_of_view,Point};


pub struct VisibilitySystem {}

impl <'a> System <'a> for VisibilitySystem {
	type SystemData = (ReadExpect<'a, Map>,
					   WriteStorage<'a, Viewshed>,
					   WriteStorage<'a, Position>);

	fn run(&mut self, data : Self::SystemData){
		let (map, mut viewshed, pos) = data;

		for (viewshed,pos) in (&mut viewshed, &pos).join(){
			viewshed.visibile_tiles.clear();
			viewshed.visibile_tiles = field_of_view(Point::new(pos.x,pos.y), viewshed.range,&*map); //field_of_view is rltk function
			viewshed.visibile_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >=0 && p.y < map.height);
		}
	}
}

