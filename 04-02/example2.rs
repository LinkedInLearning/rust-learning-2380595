
struct Rectangle (u16, u16);

fn main() {

	let rect: Rectangle = Rectangle(2,3);
	
	println!("Höhe: {}", rect.0);
	println!("Breite: {}", rect.1);
	
}


