
struct Rectangle {
	height: u16,
	width: u16,
}

impl Rectangle {

	fn area(&self) -> u16 {
		self.height * self.width
	}
}

fn main() {

	let rect: Rectangle = Rectangle {
		height: 2,
		width: 3,
	};
	
	let result = rect.area();	
	println!("Flächeninhalt: {}", result);
	
}


