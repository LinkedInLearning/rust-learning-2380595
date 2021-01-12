
macro_rules! error {
	(404) => { println!("Not Found"); };
	
	() => { println!("Unbekannter Fehler"); };
	( $e:literal ) => { println!("Fehler: {}", $e); };
}

fn main() {

	println!("Hallo Welt");
	
	error!(404);
}


