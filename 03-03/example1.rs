fn main() {

	let error = 404;
	
	if error == 400 {
		println!("Bad Request");
	} else if error == 404 {
		println!("Not Found");
	} else if error == 500 {
		println!("Internal Server Error");
	} else if error == 307 || error == 308 {
		println!("Redirect");
	} else if error == 200 {
		println!("OK");
	} else if error == 404 {
		println!("Unknown error");
	}
	
}


