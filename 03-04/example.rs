fn main() {

	let mut n: u32 = 1; 
	
	let a = loop {
		println!("Ergebnis: {}", fib(n));
		if n >= 5 { break n; }
		n += 1;
	};
	println!("Ergebnis der Loop: {}", a);

}

fn fib(n: u32) -> u32 {

	if n <= 1 {
		return n;
	} else {
		return fib(n-1) + fib(n-2);
	}
}


