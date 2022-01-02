use specs::{Component, HashMapStorage};

pub struct Pilot {
	/// Address for communicating with pilot process with grpc.
	pub url: String,
}

impl Pilot {
	pub fn new() -> Pilot {
		Pilot {
			url: "http://[::1]:50051".to_string(),
		}
	}
}

impl Component for Pilot {
	// TODO: Is this the right storage type?
	type Storage = HashMapStorage<Self>;
}
