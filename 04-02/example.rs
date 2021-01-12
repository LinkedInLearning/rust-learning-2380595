
struct Rectangle {
	height: u16,
	width: u16,
}

fn main() {

	let rect: Rectangle = Rectangle {
		height: 2,
		width: 3,
	};
	
	let rect2: Rectangle = Rectangle {
		width: 5,
		..rect
	};
	
	println!("Höhe: {}", rect.height);
	println!("Breite: {}", rect.width);
	
}

fn big_rectangle(h: u16, width: u16) -> Rectangle {
	Rectangle {
		height: h * 2,
		width,
	}
}
