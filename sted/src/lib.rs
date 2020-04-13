mod bearing;
mod velocity;

pub use bearing::Bearing;
pub use velocity::Velocity;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
