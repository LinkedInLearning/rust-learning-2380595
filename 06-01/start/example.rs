

fn main() {

	let a: [f32; 3] = [1.0, 2.0, 3.0];

	println!("Ergebnis: {}", first(&a));
}

fn first(list: &[u16]) -> &u16 {
	&list[0]
}

fn first_u32(list: &[u32]) -> &u32 {
	&list[0]
}

fn first_f32(list: &[f32]) -> &f32 {
	&list[0]
}
