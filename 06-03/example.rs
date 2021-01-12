struct Rectangle {
	height: u16,
	width: u16,
}

struct Square {
	width: u16,
}

trait Geometry {
	fn area(&self) -> u16;
	fn sides(&self) -> u16;
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
	
	fn sides(&self) -> u16 {
		return 4;
	}
}

fn calc_area<T: Geometry>(a: T) -> u16 
{
	a.area()
}

fn main() {

	let r = Rectangle { height: 2, width: 3 };
	let s = Square { width: 5 };
	
	println!("Flächeninhalt von r: {}", calc_area(r));
	println!("Flächeninhalt von s: {}", calc_area(s));
}


