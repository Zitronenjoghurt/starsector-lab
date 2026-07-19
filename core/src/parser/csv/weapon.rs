#[derive(Debug, serde::Deserialize)]
pub struct WeaponRow {
    /// In-game display name
    pub name: String,
    /// Unique weapon identifier
    pub id: String,
    /// Balance tier
    pub tier: Option<f32>,
    /// Base credit value
    #[serde(alias = "base value")]
    pub base_value: Option<f32>,
    /// Effective range
    pub range: Option<f32>,
    /// Sustained damage per second
    #[serde(alias = "damage/second")]
    pub damage_per_second: Option<f32>,
    /// Damage per individual shot
    #[serde(alias = "damage/shot")]
    pub damage_per_shot: Option<f32>,
    /// EMP damage
    pub emp: Option<f32>,
    /// Hit strength against armor
    pub impact: Option<f32>,
    /// Turret turn rate
    #[serde(alias = "turn rate")]
    pub turn_rate: Option<f32>,
    /// Ordnance points to mount
    #[serde(alias = "OPs")]
    pub ordnance_points: Option<f32>,
    /// Ammo capacity
    pub ammo: Option<f32>,
    /// Ammo regenerated per second
    #[serde(alias = "ammo/sec")]
    pub ammo_per_sec: Option<f32>,
    /// Ammo restored per reload
    #[serde(alias = "reload size")]
    pub reload_size: Option<f32>,
    /// Damage type: KINETIC, HIGH_EXPLOSIVE, ENERGY or FRAGMENTATION
    #[serde(rename = "type")]
    pub damage_type: Option<String>,
    /// Flux cost per shot
    #[serde(alias = "energy/shot")]
    pub energy_per_shot: Option<f32>,
    /// Flux cost per second while firing
    #[serde(alias = "energy/second")]
    pub energy_per_second: Option<f32>,
    /// Seconds to charge before firing
    pub chargeup: Option<f32>,
    /// Seconds to cool down after firing
    pub chargedown: Option<f32>,
    /// Number of shots in a burst
    #[serde(alias = "burst size")]
    pub burst_size: Option<f32>,
    /// Delay between shots within a burst
    #[serde(alias = "burst delay")]
    pub burst_delay: Option<f32>,
    /// Projectile travel speed
    #[serde(alias = "proj speed")]
    pub proj_speed: Option<f32>,
    /// Projectile lifetime in seconds
    #[serde(alias = "flight time")]
    pub flight_time: Option<f32>,
    /// Projectile hit points
    #[serde(alias = "proj hitpoints")]
    pub proj_hp: Option<f32>,
    /// Tech tier or manufacturer faction
    #[serde(alias = "tech/manufacturer")]
    pub tech_manufacturer: Option<String>,
    /// Primary role label shown in the tooltip
    #[serde(alias = "primaryRoleStr")]
    pub primary_role: Option<String>,
    /// Behavior and UI hint flags
    pub hints: Option<String>,
    /// Free-form classification tags
    pub tags: Option<String>,
}
