fn main() {

	let h: u16 = 2;
	let w: u16 = 3;
	
	let a = calc_area(h, w);
	
	println!("Fläche: {}", a);
	
}

fn calc_area(h: u16, w: u16) -> u16 {
	let r = h * w;
	return r;
}
