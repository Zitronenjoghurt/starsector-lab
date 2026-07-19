use crate::parser::csv::weapon::WeaponRow;
use crate::parser::wpn::WeaponSpec;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Weapon {
    /// Unique weapon identifier
    pub id: String,
    /// In-game display name
    pub name: String,
    /// Mount size the weapon fits: SMALL, MEDIUM or LARGE
    pub mount_size: String,
    /// Mount type the weapon fits: BALLISTIC, ENERGY, MISSILE, etc.
    pub mount_type: String,
    /// Projectile behavior class, e.g. projectile, beam, missile
    pub spec_class: Option<String>,
    /// Damage type dealt: KINETIC, HIGH_EXPLOSIVE, ENERGY or FRAGMENTATION
    pub damage_type: Option<String>,
    /// Balance tier, roughly how advanced the weapon is
    pub tier: Option<f32>,
    /// Ordnance points to mount the weapon
    pub ordnance_points: Option<f32>,
    /// Base credit value
    pub base_value: Option<f32>,
    /// Effective range
    pub range: Option<f32>,
    /// Damage dealt per individual shot
    pub damage_per_shot: Option<f32>,
    /// Sustained damage per second
    pub damage_per_second: Option<f32>,
    /// EMP damage, disables weapons and engines
    pub emp: Option<f32>,
    /// Hit strength used against armor
    pub impact: Option<f32>,
    /// Turret turn rate in degrees per second
    pub turn_rate: Option<f32>,
    /// Ammo capacity, absent for weapons with unlimited ammo
    pub ammo: Option<f32>,
    /// Ammo regenerated per second
    pub ammo_per_sec: Option<f32>,
    /// Ammo restored per reload
    pub reload_size: Option<f32>,
    /// Flux cost per shot
    pub energy_per_shot: Option<f32>,
    /// Flux cost per second while firing
    pub energy_per_second: Option<f32>,
    /// Seconds to charge before firing
    pub chargeup: Option<f32>,
    /// Seconds to cool down after firing
    pub chargedown: Option<f32>,
    /// Number of shots in a burst
    pub burst_size: Option<f32>,
    /// Delay between shots within a burst
    pub burst_delay: Option<f32>,
    /// Projectile travel speed
    pub proj_speed: Option<f32>,
    /// Projectile lifetime in seconds
    pub flight_time: Option<f32>,
    /// Projectile hit points, if it can be shot down
    pub proj_hp: Option<f32>,
    /// Tech tier or manufacturer faction
    pub tech_manufacturer: Option<String>,
    /// Primary role label shown in the weapon tooltip
    pub primary_role: Option<String>,
    /// Behavior and UI hint flags
    pub hints: Vec<String>,
    /// Free-form classification tags
    pub tags: Vec<String>,
    /// Absolute path to the turret (or hardpoint) sprite, if any
    pub sprite: Option<PathBuf>,
}

impl Weapon {
    pub fn from_parts(row: WeaponRow, spec: WeaponSpec, core_dir: &Path) -> Option<Self> {
        if row.id.is_empty() || row.name.is_empty() {
            return None;
        }

        Some(Self {
            id: row.id,
            name: row.name,
            mount_size: spec.mount_size,
            mount_type: spec.mount_type,
            spec_class: spec.spec_class,
            damage_type: row.damage_type,
            tier: row.tier,
            ordnance_points: row.ordnance_points,
            base_value: row.base_value,
            range: row.range,
            damage_per_shot: row.damage_per_shot,
            damage_per_second: row.damage_per_second,
            emp: row.emp,
            impact: row.impact,
            turn_rate: row.turn_rate,
            ammo: row.ammo,
            ammo_per_sec: row.ammo_per_sec,
            reload_size: row.reload_size,
            energy_per_shot: row.energy_per_shot,
            energy_per_second: row.energy_per_second,
            chargeup: row.chargeup,
            chargedown: row.chargedown,
            burst_size: row.burst_size,
            burst_delay: row.burst_delay,
            proj_speed: row.proj_speed,
            flight_time: row.flight_time,
            proj_hp: row.proj_hp,
            tech_manufacturer: row.tech_manufacturer,
            primary_role: row.primary_role,
            hints: split(row.hints),
            tags: split(row.tags),
            sprite: spec.sprite.map(|rel| core_dir.join(rel)),
        })
    }
}

fn split(field: Option<String>) -> Vec<String> {
    field
        .unwrap_or_default()
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect()
}
