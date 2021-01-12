
fn countup() -> impl FnMut() {
	let mut counter = 0;
	
	move || { counter += 1; println!("{}", counter); }
}

fn main() {

	let mut plus = countup();
	plus();
	plus();
	plus();
	
}


