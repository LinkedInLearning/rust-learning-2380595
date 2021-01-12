

fn main() {

	let a: [f32; 3] = [1.0, 2.0, 3.0];

	println!("Ergebnis: {}", first(&a));
}

fn first<T>(list: &[T]) -> &T {
	let r: &T = &list[0];
	return r;
}



