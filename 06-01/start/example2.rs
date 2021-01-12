struct Rectangle<T> {
	height: T,
	width: T,
}

impl<T> Rectangle<T> {
	fn height(&self) -> &T {
		&self.height
	}
}

impl Rectangle<u16> {
	fn area(&self) -> u16 {
		self.height * self.width
	}
}

fn main() {

	let rect_u = Rectangle { height: 2, width: 3 };
	let rect_f = Rectangle { height: 4.0, width: 5.0 };
	
	let rect_u16 = Rectangle { height: 2 as u16, width: 3 as u16 };
	println!("Flächeninhalt: {}", rect_u16.area());
	
}


