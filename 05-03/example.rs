

fn main() {

	let a: [i16; 15] = [5,10,15,20,30,40,50,60,70,80,90,100,110,120,130];
	
	let b = &a[..6];

	for i in b.iter() {
		println!("{}", i);
	};
	
	let s = "Geschwindigkeit";
	
	print_string(s);
}

fn print_string(s: &str) {
	println!("{}", s);
}
