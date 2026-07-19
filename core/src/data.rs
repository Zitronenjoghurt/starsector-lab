pub mod ship;
pub mod weapon;

#[derive(Debug, Default)]
pub struct Data {
    pub core_dir: Option<std::path::PathBuf>,
    pub ships: Vec<ship::Ship>,
    pub weapons: Vec<weapon::Weapon>,
}
