pub mod ship;

#[derive(Debug)]
pub struct Data {
    pub ships: Vec<ship::Ship>,
}
