
mod geometry;

fn main() {


	let rect: geometry::polygons::Rectangle = geometry::polygons::Rectangle {
		height: 2,
		width: 3,
	};
	
	let result = rect.area();	
	println!("Flächeninhalt: {}", result);
	
}


