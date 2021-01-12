
struct Rectangle {
	height: u16,
	width: u16,
}

fn main() {

	let height: u16 = 2;
	let width: u16 = 3;
	
	let mut rect: Rectangle = Rectangle {
		height: 2,
		width: 3,
	};
	
	rect.width = 8;
	
	println!("Höhe: {}", rect.height);
	println!("Breite: {}", rect.width);
	
}


