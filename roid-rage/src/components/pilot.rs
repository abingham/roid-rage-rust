use specs::{Component, HashMapStorage};

pub struct Pilot {
    /// Address for communicating with pilot process with grpc.
    pub url: String,
}

impl Pilot {
    pub fn new(url: &str) -> Pilot {
        Pilot {
            url: url.to_string(),
        }
    }
}

impl Component for Pilot {
    // TODO: Is this the right storage type?
    type Storage = HashMapStorage<Self>;
}
