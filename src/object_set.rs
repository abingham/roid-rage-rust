use crate::objects::roid::Roid;
use crate::objects::bullet::Bullet;
use crate::traits::{Renderable, Updateable};

/// The state of the game: the field of play and the objects on it.
pub struct ObjectSet {
    pub roids: Vec<Roid>,
    pub bullets: Vec<Bullet>
}

impl ObjectSet {
    pub fn new() -> ObjectSet {
        ObjectSet {
            roids: vec![],
            bullets: vec![]
        }
    }

    pub fn from_objects(roids: Vec<Roid>, bullets: Vec<Bullet>) -> ObjectSet {
        ObjectSet {
            roids: roids,
            bullets: bullets
        }
    }

    pub fn extend(&mut self, other: ObjectSet) {
        self.roids.extend(other.roids);
        self.bullets.extend(other.bullets);
    }

    /// All renderable objects
    pub fn renderables(&self) -> Vec<&dyn Renderable> {
        let result: Vec<&dyn Renderable> = self
            .roids
            .iter()
            .map(|r| r as &dyn Renderable)
            .chain(self.bullets.iter().map(|b| b as &dyn Renderable))
            .collect();

        result
    }

    /// All updateable objects
    pub fn updateables(&mut self) -> Vec<&mut dyn Updateable> {
        let result: Vec<&mut dyn Updateable> = self
            .roids
            .iter_mut()
            .map(|r| r as &mut dyn Updateable)
            .chain(self.bullets.iter_mut().map(|b| b as &mut dyn Updateable))
            .collect();

        result
    }
}



