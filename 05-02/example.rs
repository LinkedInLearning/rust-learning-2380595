struct Rectangle {
	height: u16,
	width: u16,
}

impl Rectangle {
	fn area(&self) -> u16 {
		self.height * self.width
	}
}

fn rect_print(r: &mut Rectangle) {
	println!("Höhe: {}", r.height);
	println!("Breite: {}", r.width);
}

fn main() {

	let mut r: Rectangle = Rectangle {
		height: 2,
		width: 3,
	};
	
	let r1 = &mut r;
	let r2 = &mut r;
	r1.height = 5;
	
	rect_print(&mut r);
	println!("Flächeninhalt: {}", r.area());
}


