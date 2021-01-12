

enum Color { Red(u16), Yellow, Green }

fn main() {

	let mut trafficlight = Color::Red(8);
	
	match trafficlight {
		Color::Red(8) => println!("Rot"),
		Color::Yellow => println!("Gelb"),
		Color::Green => println!("Grün"),
		_ => println!("Unbekannte Ampelschaltung")
	}
	
}


