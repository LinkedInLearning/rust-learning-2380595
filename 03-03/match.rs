
fn bonus(points: u8) -> bool {
	match points {
		100 => true,
		_ => false,
	}
}


fn main() {

	if bonus(100) { println!("Bonus!"); }
	
}


