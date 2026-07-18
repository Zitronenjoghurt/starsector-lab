use crate::parser::csv::hull::HullRow;
use crate::parser::json::ship::{EngineSlot, ShipFile, WeaponSlot};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Ship {
    /// Unique hull identifier
    pub id: String,
    /// In-game display name
    pub name: String,
    /// Role label shown under the name, e.g. Assault, Escort
    pub designation: Option<String>,
    /// Tech tier or manufacturer faction, e.g. Low Tech, Midline
    pub tech_manufacturer: Option<String>,
    /// Id of the built-in ship system
    pub system_id: Option<String>,
    /// Fleet points cost, drives deployment and reinforcement weighting
    pub fleet_points: u32,
    /// Base hull integrity before armor
    pub hit_points: u32,
    /// Armor value at full strength, absorbs a fraction of incoming damage
    pub armor_rating: u32,
    /// Flux capacity before the ship overloads
    pub max_flux: u32,
    /// Flux vented per second while not overloaded
    pub flux_dissipation: u32,
    /// Ordnance points available for weapons and hull mods
    pub ordnance_points: u32,
    /// Number of fighter launch bays
    pub fighter_bays: u32,
    /// Top speed in engine units per second
    pub max_speed: f32,
    /// Rate the ship gains speed
    pub acceleration: f32,
    /// Rate the ship sheds speed
    pub deceleration: f32,
    /// Peak turn rate in degrees per second
    pub max_turn_rate: f32,
    /// Rate the ship reaches its peak turn rate
    pub turn_acceleration: f32,
    /// Hull mass, affects collision impulse
    pub mass: f32,
    /// Shield behavior, e.g. FRONT, OMNI, PHASE, NONE
    pub shield_type: String,
    /// Shield coverage in degrees, absent for phase and shieldless hulls
    pub shield_arc: Option<f32>,
    /// Flux per second to keep the shield raised
    pub shield_upkeep: Option<f32>,
    /// Fraction of damage converted to flux while shielded, lower is better
    pub shield_efficiency: Option<f32>,
    /// Flux to enter phase cloak, phase hulls only
    pub phase_cost: Option<f32>,
    /// Flux per second while phase cloaked, phase hulls only
    pub phase_upkeep: Option<f32>,
    /// Crew needed to operate the ship
    pub min_crew: u32,
    /// Crew the ship can carry
    pub max_crew: u32,
    /// Cargo capacity
    pub cargo: u32,
    /// Fuel capacity
    pub fuel: u32,
    /// Fuel burned per light year of travel
    pub fuel_per_ly: f32,
    /// Campaign map burn speed
    pub max_burn: u32,
    /// Base credit value before condition and market modifiers
    pub base_value: u32,
    /// Combat readiness recovered per day
    pub cr_percent_per_day: f32,
    /// Combat readiness spent to deploy the ship
    pub cr_to_deploy: u32,
    /// Seconds of peak performance before combat readiness decays
    pub peak_cr_sec: u32,
    /// Combat readiness lost per second past peak
    pub cr_loss_per_sec: f32,
    /// Supplies to recover the ship after it is disabled
    pub supplies_per_recovery: f32,
    /// Supplies consumed per month for upkeep
    pub supplies_per_month: f32,
    /// Behavior hints read by the game, e.g. UNDER_PARETO, DO_NOT_SHOW_IN_CODEX
    pub hints: Vec<String>,
    /// Free-form classification tags
    pub tags: Vec<String>,
    /// Geometry and slot layout from the `.ship` file
    pub layout: ShipLayout,
}

/// Hull geometry and layout, sourced from the `.ship` file.
#[derive(Debug)]
pub struct ShipLayout {
    /// Hull size class, e.g. FRIGATE, DESTROYER, CRUISER, CAPITAL_SHIP, FIGHTER
    pub hull_size: String,
    /// Visual style preset, e.g. HIGH_TECH
    pub style: String,
    /// Absolute path to the hull sprite, the `graphics/...` value joined onto the Starsector core or mod root
    pub sprite: PathBuf,
    /// Sprite width in pixels
    pub width: f32,
    /// Sprite height in pixels
    pub height: f32,
    /// Sprite-space center point the ship rotates around, `[x, y]`
    pub center: [f32; 2],
    /// Radius used for coarse ship-to-ship collision
    pub collision_radius: f32,
    /// Center of the shield relative to the hull center, `[x, y]`
    pub shield_center: [f32; 2],
    /// Radius of the shield bubble
    pub shield_radius: f32,
    /// Rotation offset applied to the sprite, in degrees
    pub view_offset: f32,
    /// Collision outline as a flat list of `x, y` pairs
    pub bounds: Vec<f32>,
    /// Anchor point when this ship is a station module, `[x, y]`
    pub module_anchor: Option<[f32; 2]>,
    /// Ids of hull mods built into the ship
    pub built_in_mods: Vec<String>,
    /// Wing ids for fighters permanently attached to the ship
    pub built_in_wings: Vec<String>,
    /// Weapons fixed to specific slots, keyed by weapon-slot id
    pub built_in_weapons: HashMap<String, String>,
    /// Engine glow / thruster placements
    pub engine_slots: Vec<EngineSlot>,
    /// Weapon mount placements
    pub weapon_slots: Vec<WeaponSlot>,
}

impl ShipLayout {
    fn from_file(file: ShipFile, core_dir: &Path) -> Self {
        Self {
            hull_size: file.hull_size,
            style: file.style,
            sprite: core_dir.join(file.sprite_name),
            width: file.width,
            height: file.height,
            center: file.center,
            collision_radius: file.collision_radius,
            shield_center: file.shield_center,
            shield_radius: file.shield_radius,
            view_offset: file.view_offset,
            bounds: file.bounds,
            module_anchor: file.module_anchor,
            built_in_mods: file.built_in_mods,
            built_in_wings: file.built_in_wings,
            built_in_weapons: file.built_in_weapons,
            engine_slots: file.engine_slots,
            weapon_slots: file.weapon_slots,
        }
    }
}

impl Ship {
    pub fn from_parts(hull: HullRow, layout: ShipFile, core_dir: &Path) -> Option<Self> {
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
            layout: ShipLayout::from_file(layout, core_dir),
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
