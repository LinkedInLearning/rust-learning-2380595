fn main() {

	let r = fib(4);
	println!("Ergebnis: {}", r);
	
	let n = 34;
	let zahl = if n > 33 { 1 } else { 2 };
	println!("Zahl: {}", zahl);
}

fn fib(n: u32) -> u32 {

	if n <= 1 {
	
		return n;
		
	} else {
		return fib(n-1) + fib(n-2);
	}
}


