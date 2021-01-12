struct Rectangle {
	height: u16,
	width: u16,
}

impl Rectangle {

	fn area(&self) -> u16 {
		self.height * self.width
	}
	
	fn clone(&self) -> Rectangle {
		Rectangle {
			height: self.height,
			width: self.width,
		}
	}
}

fn main() {

	let r: Rectangle = Rectangle {
		height: 2,
		width: 3,
	};
	
	let s = r.clone();	
	
	println!("Höhe: {}", r.height);
	
	let a = 5;
	let b = a;
	
	println!("b ist: {}", b);
}


