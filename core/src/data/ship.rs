use crate::parser::Hull;

#[derive(Debug)]
pub struct Ship {
    pub id: String,
    pub name: String,
    pub designation: Option<String>,
    pub tech_manufacturer: Option<String>,
    pub system_id: Option<String>,
    pub fleet_points: u32,
    pub hit_points: u32,
    pub armor_rating: u32,
    pub max_flux: u32,
    pub flux_dissipation: u32,
    pub ordnance_points: u32,
    pub fighter_bays: u32,
    pub max_speed: f32,
    pub acceleration: f32,
    pub deceleration: f32,
    pub max_turn_rate: f32,
    pub turn_acceleration: f32,
    pub mass: f32,
    pub shield_type: String,
    pub shield_arc: Option<f32>,
    pub shield_upkeep: Option<f32>,
    pub shield_efficiency: Option<f32>,
    pub phase_cost: Option<f32>,
    pub phase_upkeep: Option<f32>,
    pub min_crew: u32,
    pub max_crew: u32,
    pub cargo: u32,
    pub fuel: u32,
    pub fuel_per_ly: f32,
    pub max_burn: u32,
    pub base_value: u32,
    pub cr_percent_per_day: f32,
    pub cr_to_deploy: u32,
    pub peak_cr_sec: u32,
    pub cr_loss_per_sec: f32,
    pub supplies_per_recovery: f32,
    pub supplies_per_month: f32,
    pub hints: Vec<String>,
    pub tags: Vec<String>,
}

impl Ship {
    pub fn from_hull(hull: Hull) -> Option<Self> {
        if hull.name.is_empty() {
            return None;
        }

        Some(Self {
            id: hull.id,
            name: hull.name,
            designation: hull.designation,
            tech_manufacturer: hull.tech_manufacturer,
            system_id: hull.system_id,
            fleet_points: hull.fleet_points?,
            hit_points: hull.hit_points,
            armor_rating: hull.armor_rating,
            max_flux: hull.max_flux,
            flux_dissipation: hull.flux_dissipation,
            ordnance_points: hull.ordnance_points?,
            fighter_bays: hull.fighter_bays.unwrap_or(0),
            max_speed: hull.max_speed,
            acceleration: hull.acceleration,
            deceleration: hull.deceleration,
            max_turn_rate: hull.max_turn_rate,
            turn_acceleration: hull.turn_acceleration,
            mass: hull.mass,
            shield_type: hull.shield_type,
            shield_arc: hull.shield_arc,
            shield_upkeep: hull.shield_upkeep,
            shield_efficiency: hull.shield_efficiency,
            phase_cost: hull.phase_cost,
            phase_upkeep: hull.phase_upkeep,
            min_crew: hull.min_crew?,
            max_crew: hull.max_crew?,
            cargo: hull.cargo?,
            fuel: hull.fuel?,
            fuel_per_ly: hull.fuel_per_ly?,
            max_burn: hull.max_burn?,
            base_value: hull.base_value?,
            cr_percent_per_day: hull.cr_percent_per_day?,
            cr_to_deploy: hull.cr_to_deploy?,
            peak_cr_sec: hull.peak_cr_sec?,
            cr_loss_per_sec: hull.cr_loss_per_sec?,
            supplies_per_recovery: hull.supplies_per_recovery?,
            supplies_per_month: hull.supplies_per_month?,
            tags: split(hull.tags),
            hints: split(hull.hints),
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
