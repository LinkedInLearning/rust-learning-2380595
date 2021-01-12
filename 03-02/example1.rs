fn main() {

	let r = fib(4);
	println!("Ergebnis: {}", r);
	
}

fn fib(n: u32) -> u32 {

	if n == 0 {
	
		return 0;
		
	} else {
	
		if n == 1 {
			return 1;
		} else {
			return fib(n-1) + fib(n-2);
		}
	}
}


