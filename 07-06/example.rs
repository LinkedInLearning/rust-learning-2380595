
use std::thread;
use std::time::Duration;

fn main() {

	let t = thread::spawn( || {
		for i in 1..11 {
			println!("Thread, Durchlauf: {}", i);
			thread::sleep( Duration::from_secs(1) );
		}
	});
	
	for i in 1..11 {
		println!("Hauptprogramm, Durchlauf: {}", i);
		thread::sleep( Duration::from_secs_f32(0.5));
	}

	t.join();

}



