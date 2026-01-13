use specs::{Component, VecStorage};

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
    type Storage = VecStorage<Self>;
}
