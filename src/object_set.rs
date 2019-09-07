use crate::objects::roid::Roid;
use crate::objects::bullet::Bullet;
use crate::game_object::GameObject;

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

    pub fn remove_dead(&mut self) {
        self.roids.retain(|r| r.alive());
        self.bullets.retain(|b| b.alive());
    }

    pub fn extend(&mut self, other: ObjectSet) {
        self.roids.extend(other.roids);
        self.bullets.extend(other.bullets);
    }

    /// All GameObjects
    pub fn iter<'a>(&'a self) ->impl Iterator<Item = &'a dyn GameObject> {
        self
            .roids
            .iter()
            .map(|r| r as &dyn GameObject)
            .chain(self.bullets.iter().map(|b| b as &dyn GameObject))
    }

    pub fn iter_mut<'a>(&'a mut self) ->impl Iterator<Item = &'a mut dyn GameObject> {
        self
            .roids
            .iter_mut()
            .map(|r| r as &mut dyn GameObject)
            .chain(self.bullets.iter_mut().map(|b| b as &mut dyn GameObject))
    }
}



