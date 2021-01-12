
fn main() {

	let hello = "Hallo Welt!";
	
	// String:
	
	let mut s1 = String::from("Hallo");
	s1.push_str(" Welt");
	s1.push('!');
	
	let s2 = " Willkommen".to_string();
	let s3 = String::new();
	
	let s4 = s1 + &s2 + &s3 + "!";
	
	println!("{}", &s4[0..11] );

	
	// Vector:
	
	let mut v: Vec<u16> = Vec::new();
	v.push(5);
	v.push(6);
	
	println!("Das zweite Elemente ist: {}", &v[1] );
	
	let v2 = vec![1,2,3];
	
	for i in v2.iter() {
		println!("{}", i);
	}
	
	let mut it = v2.iter();
	match it.next() {
		Some(&z) => println!("Nächstes Element: {}", z),
		_ => println!("Kein weiteres Element vorhanden.")
	}
	
	
	// Hash Map:
	
	use std::collections::HashMap;
	
	let mut phonebook = HashMap::new();
	
	phonebook.insert( String::from("Tim Schürmann"), 12345678 );
	
	let tim = String::from("Tim Schürmann");
	match phonebook.get(&tim) {
		Some(&number) => println!("Die Telefonnummer lautet: {}", number),
		_ => println!("Es gibt keine Telefonnummer.")
	}
	
	for (key, value) in phonebook.iter() {
		println!("{} hat die Telefonnummer: {}", key, value);
	}
}








