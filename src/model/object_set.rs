use super::objects::bullet::Bullet;
use super::objects::fragment::Fragment;
use super::objects::roid::Roid;

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
}