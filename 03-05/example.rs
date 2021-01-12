fn main() {

	let z: [u32; 5] = [1, 2, 3, 4, 5];
	
	for n in z.iter() {
		println!("Ergebnis: {}", fib(*n));
	}

}

fn fib(n: u32) -> u32 {

	if n <= 1 {
		return n;
	} else {
		return fib(n-1) + fib(n-2);
	}
}


