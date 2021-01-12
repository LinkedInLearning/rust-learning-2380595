
mod geometry {
	pub mod polygons {
		pub struct Rectangle {
			pub height: u16,
			pub width: u16,
		}

		impl Rectangle {
			pub fn area(&self) -> u16 {
				self.height * self.width
			}
		}
	}
}

fn main() {

	use geometry::polygons as geo;

	let rect: geo::Rectangle = geometry::polygons::Rectangle {
		height: 2,
		width: 3,
	};
	
	let result = rect.area();	
	println!("Flächeninhalt: {}", result);
	
}


