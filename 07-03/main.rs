


fn main() {


	let rect: geo::polygons::Rectangle = geo::polygons::Rectangle {
		height: 2,
		width: 3,
	};
	
	let result = rect.area();	
	println!("Flächeninhalt: {}", result);
	
}


