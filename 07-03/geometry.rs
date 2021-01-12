
	pub mod polygons {
		pub struct Rectangle {
			pub height: u16,
			pub width: u16,
		}

		impl Rectangle {
			pub fn area(&self) -> u16 {
				self.height * self.width
			}
		}
	}


