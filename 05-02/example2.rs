

fn main() {

	let a = 1;
	let refa = &a;
	let mut b = *refa;
	b = 2;
	
	println!("b ist: {}", b);
	
}


