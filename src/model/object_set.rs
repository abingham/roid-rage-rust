use super::objects::bullet::Bullet;
use super::objects::fragment::Fragment;
use super::objects::roid::Roid;

/// A simple collection of all object types
/// 
/// This is primarily used as a way of communicating change sets and so forth in the system.
pub struct ObjectSet {
    pub bullets: Vec<Bullet>,
    pub fragments: Vec<Fragment>,
    pub roids: Vec<Roid>,
}

impl ObjectSet {
    pub fn new() -> ObjectSet {
        ObjectSet {
            bullets: vec![],
            fragments: vec![],
            roids: vec![],
        }
    }

    pub fn extend(&mut self, other: ObjectSet) {
        self.bullets.extend(other.bullets);
        self.fragments.extend(other.fragments);
        self.roids.extend(other.roids);
    }
}