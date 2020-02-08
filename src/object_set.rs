use crate::objects::roid::Roid;
use crate::objects::bullet::Bullet;
use crate::objects::fragment::Fragment;
use crate::game_object::GameObject;
use crate::collide::collide;
use std::collections::HashSet;

/// The state of the game: the field of play and the objects on it.
pub struct ObjectSet {
    roids: Vec<Roid>,
    bullets: Vec<Bullet>,
    fragments: Vec<Fragment>
}

impl ObjectSet {
    pub fn new() -> ObjectSet {
        ObjectSet {
            roids: vec![],
            bullets: vec![],
            fragments: vec![],
        }
    }

    pub fn from_objects(
            roids: Vec<Roid>, 
            bullets: Vec<Bullet>,
            fragments: Vec<Fragment>) -> ObjectSet {
        ObjectSet {
            roids: roids,
            bullets: bullets,
            fragments: fragments
        }
    }

    // pub fn remove_dead(&mut self) {
    //     self.roids.retain(|r| r.alive());
    //     self.bullets.retain(|b| b.alive());
    //     self.fragments.retain(|b| b.alive());
    // }

    pub fn extend(&mut self, other: ObjectSet) {
        self.roids.extend(other.roids);
        self.bullets.extend(other.bullets);
        self.fragments.extend(other.fragments);
    }

    pub fn collisions(&self, time_delta: f64) -> HashSet<uuid::Uuid> {
        collide(&self.roids, &self.bullets, time_delta)
            .iter()
            .fold(HashSet::new(), |mut acc, x| {
                acc.insert(x.0.id());
                acc.insert(x.1.id());
                acc
            })
    }

    /// All GameObjects
    pub fn iter<'a>(&'a self) ->impl Iterator<Item = &'a dyn GameObject> {
        self
            .roids
            .iter()
            .map(|r| r as &dyn GameObject)
            .chain(self.bullets.iter().map(|b| b as &dyn GameObject))
            .chain(self.fragments.iter().map(|f| f as &dyn GameObject))
    }

    pub fn iter_mut<'a>(&'a mut self) ->impl Iterator<Item = &'a mut dyn GameObject> {
        self
            .roids
            .iter_mut()
            .map(|r| r as &mut dyn GameObject)
            .chain(self.bullets.iter_mut().map(|b| b as &mut dyn GameObject))
            .chain(self.fragments.iter_mut().map(|f| f as &mut dyn GameObject))
    }

    pub fn roids<'a>(&'a self) ->impl Iterator<Item = &'a Roid> {
        self.roids.iter()
    }


}



