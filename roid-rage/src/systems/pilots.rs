/// System that associates pilots processes with ships
use specs::{Entities, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteExpect};

pub struct PilotsSystem;

impl<'s> System<'s> for PilotsSystem {
	type SystemData = (
		ReadStorage<'s, Pilot>,
		WriteStorage<'s, Ship>,
		Entities<'s>,
		// ReadExpect<'s, Settings>,
		// Read<'s, LazyUpdate>,
	);

	/// TODO: This should check for new pilot registrations and create a pilot/ship
	/// for them.
	fn run(&mut self, (pilots, mut ships, entities): Self::SystemData) {}
}
