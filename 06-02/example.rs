struct Rectangle {
	height: u16,
	width: u16,
}

struct Square {
	width: u16,
}

trait Geometry {
	fn area(&self) -> u16;
	fn sides(&self) -> u16 { return 0; }
}

impl Geometry for Rectangle {
	fn area(&self) -> u16 {
		self.height * self.width
	}
	
	fn sides(&self) -> u16 {
		return 4;
	}
}

impl Geometry for Square {
	fn area(&self) -> u16 {
		self.width * self.width
	}
}

fn main() {

	let r = Rectangle { height: 2, width: 3 };
	let s = Square { width: 5 };
	
	println!("Flächeninhalt Rechteck: {}", r.area());
	println!("Anzahl Seiten Rechteck: {}", r.sides());
	println!("Flächeninhalt Quadrat: {}", s.area());
	println!("Anzahl Seiten Quadrat: {}", s.sides());
	
}


