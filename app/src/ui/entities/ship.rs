use crate::ui::widgets::table::{CellValue, Column, TableEntity};
use starsector_lab::data::ship::Ship;
use std::path::Path;

fn opt_text(value: Option<&str>) -> CellValue {
    value.map_or(CellValue::Empty, |s| CellValue::Text(s.to_owned()))
}

fn opt_f32(value: Option<f32>) -> CellValue {
    value.map_or(CellValue::Empty, |v| CellValue::Float(v as f64))
}

macro_rules! col {
    ($id:literal, $label:literal, $num:expr, $vis:expr, $hb:expr, $w:expr, $tip:literal, $val:expr $(,)?) => {
        Column {
            id: $id,
            label: $label,
            numeric: $num,
            default_visible: $vis,
            higher_better: $hb,
            width: $w,
            tooltip: $tip,
            value: $val,
        }
    };
}

const COLUMNS: &[Column<Ship>] = &[
    // Identity
    col!(
        "name",
        "Name",
        false,
        true,
        None,
        150.0,
        "In-game display name of the hull.",
        |s| CellValue::Text(s.name.clone())
    ),
    col!(
        "designation",
        "Role",
        false,
        true,
        None,
        90.0,
        "Role label shown under the name, e.g. Assault or Escort.",
        |s| opt_text(s.designation.as_deref())
    ),
    col!(
        "hull_size",
        "Size",
        false,
        true,
        None,
        90.0,
        "Hull size class: FRIGATE, DESTROYER, CRUISER, CAPITAL_SHIP or FIGHTER.",
        |s| CellValue::Text(s.layout.hull_size.clone())
    ),
    col!(
        "tech",
        "Tech",
        false,
        false,
        None,
        90.0,
        "Tech tier or manufacturer faction, e.g. Low Tech, Midline, High Tech.",
        |s| opt_text(s.tech_manufacturer.as_deref())
    ),
    col!(
        "system",
        "System",
        false,
        false,
        None,
        120.0,
        "Id of the built-in ship system.",
        |s| opt_text(s.system_id.as_deref())
    ),
    // Cost / meta
    col!(
        "fleet_points",
        "FP",
        true,
        true,
        Some(false),
        44.0,
        "Fleet points. Deployment cost and reinforcement weighting. Lower is cheaper.",
        |s| CellValue::Int(s.fleet_points as i64)
    ),
    col!(
        "ordnance_points",
        "OP",
        true,
        true,
        Some(true),
        44.0,
        "Ordnance points available for weapons and hull mods.",
        |s| CellValue::Int(s.ordnance_points as i64)
    ),
    col!(
        "base_value",
        "Value",
        true,
        true,
        Some(true),
        70.0,
        "Base credit value before condition and market modifiers.",
        |s| CellValue::Int(s.base_value as i64)
    ),
    col!(
        "mounts",
        "Mounts",
        true,
        false,
        None,
        55.0,
        "Number of weapon mounts on the hull.",
        |s| CellValue::Int(s.layout.weapon_slots.len() as i64)
    ),
    // Defense
    col!(
        "hit_points",
        "Hull",
        true,
        true,
        Some(true),
        60.0,
        "Base hull integrity, before armor.",
        |s| CellValue::Int(s.hit_points as i64)
    ),
    col!(
        "armor_rating",
        "Armor",
        true,
        true,
        Some(true),
        60.0,
        "Armor value at full strength. Absorbs a fraction of incoming damage.",
        |s| CellValue::Int(s.armor_rating as i64)
    ),
    col!(
        "max_flux",
        "Flux",
        true,
        true,
        Some(true),
        60.0,
        "Flux capacity before the ship overloads.",
        |s| CellValue::Int(s.max_flux as i64)
    ),
    col!(
        "flux_dissipation",
        "Diss",
        true,
        true,
        Some(true),
        55.0,
        "Flux vented per second while not overloaded.",
        |s| CellValue::Int(s.flux_dissipation as i64)
    ),
    // Shields / phase
    col!(
        "shield_type",
        "Shield",
        false,
        false,
        None,
        70.0,
        "Shield behavior: FRONT, OMNI, PHASE or NONE.",
        |s| CellValue::Text(s.shield_type.clone())
    ),
    col!(
        "shield_arc",
        "Sh.Arc",
        true,
        false,
        Some(true),
        55.0,
        "Shield coverage in degrees. Absent for phase and shieldless hulls.",
        |s| opt_f32(s.shield_arc)
    ),
    col!(
        "shield_upkeep",
        "Sh.Up",
        true,
        false,
        Some(false),
        55.0,
        "Flux per second to keep the shield raised. Lower is cheaper.",
        |s| opt_f32(s.shield_upkeep)
    ),
    col!(
        "shield_efficiency",
        "Sh.Eff",
        true,
        false,
        Some(false),
        55.0,
        "Fraction of damage converted to flux while shielded. Lower is better.",
        |s| opt_f32(s.shield_efficiency)
    ),
    col!(
        "phase_cost",
        "Ph.Cost",
        true,
        false,
        Some(false),
        60.0,
        "Flux to enter phase cloak. Phase hulls only.",
        |s| opt_f32(s.phase_cost)
    ),
    col!(
        "phase_upkeep",
        "Ph.Up",
        true,
        false,
        Some(false),
        55.0,
        "Flux per second while phase cloaked. Phase hulls only.",
        |s| opt_f32(s.phase_upkeep)
    ),
    // Mobility
    col!(
        "max_speed",
        "Speed",
        true,
        true,
        Some(true),
        55.0,
        "Top speed in engine units per second.",
        |s| CellValue::Float(s.max_speed as f64)
    ),
    col!(
        "acceleration",
        "Accel",
        true,
        false,
        Some(true),
        55.0,
        "Rate the ship gains speed.",
        |s| CellValue::Float(s.acceleration as f64)
    ),
    col!(
        "deceleration",
        "Decel",
        true,
        false,
        Some(true),
        55.0,
        "Rate the ship sheds speed.",
        |s| CellValue::Float(s.deceleration as f64)
    ),
    col!(
        "max_turn_rate",
        "Turn",
        true,
        false,
        Some(true),
        55.0,
        "Peak turn rate in degrees per second.",
        |s| CellValue::Float(s.max_turn_rate as f64)
    ),
    col!(
        "turn_acceleration",
        "TurnAcc",
        true,
        false,
        Some(true),
        65.0,
        "Rate the ship reaches its peak turn rate.",
        |s| CellValue::Float(s.turn_acceleration as f64)
    ),
    col!(
        "mass",
        "Mass",
        true,
        false,
        None,
        55.0,
        "Hull mass. Affects collision impulse.",
        |s| CellValue::Float(s.mass as f64)
    ),
    // Crew / logistics
    col!(
        "fighter_bays",
        "Bays",
        true,
        false,
        Some(true),
        45.0,
        "Number of fighter launch bays.",
        |s| CellValue::Int(s.fighter_bays as i64)
    ),
    col!(
        "min_crew",
        "MinCrew",
        true,
        false,
        Some(false),
        65.0,
        "Crew needed to operate the ship. Lower is cheaper.",
        |s| CellValue::Int(s.min_crew as i64)
    ),
    col!(
        "max_crew",
        "MaxCrew",
        true,
        false,
        Some(true),
        65.0,
        "Crew the ship can carry.",
        |s| CellValue::Int(s.max_crew as i64)
    ),
    col!(
        "cargo",
        "Cargo",
        true,
        false,
        Some(true),
        55.0,
        "Cargo capacity.",
        |s| CellValue::Int(s.cargo as i64)
    ),
    col!(
        "fuel",
        "Fuel",
        true,
        false,
        Some(true),
        50.0,
        "Fuel capacity.",
        |s| CellValue::Int(s.fuel as i64)
    ),
    col!(
        "fuel_per_ly",
        "Fuel/ly",
        true,
        false,
        Some(false),
        60.0,
        "Fuel burned per light year of travel. Lower is cheaper.",
        |s| CellValue::Float(s.fuel_per_ly as f64)
    ),
    col!(
        "max_burn",
        "Burn",
        true,
        false,
        Some(true),
        45.0,
        "Campaign map burn (travel) speed.",
        |s| CellValue::Int(s.max_burn as i64)
    ),
    col!(
        "supplies_per_month",
        "Sup/mo",
        true,
        false,
        Some(false),
        60.0,
        "Supplies consumed per month for upkeep. Lower is cheaper.",
        |s| CellValue::Float(s.supplies_per_month as f64)
    ),
    col!(
        "supplies_per_recovery",
        "Sup/rec",
        true,
        false,
        Some(false),
        65.0,
        "Supplies to recover the ship after it is disabled. Lower is cheaper.",
        |s| CellValue::Float(s.supplies_per_recovery as f64)
    ),
    // Combat readiness
    col!(
        "cr_percent_per_day",
        "CR/day",
        true,
        false,
        Some(true),
        55.0,
        "Combat readiness recovered per day.",
        |s| CellValue::Float(s.cr_percent_per_day as f64)
    ),
    col!(
        "cr_to_deploy",
        "CRdep",
        true,
        false,
        Some(false),
        55.0,
        "Combat readiness spent to deploy the ship. Lower is cheaper.",
        |s| CellValue::Int(s.cr_to_deploy as i64)
    ),
    col!(
        "peak_cr_sec",
        "PeakCR",
        true,
        false,
        Some(true),
        60.0,
        "Seconds of peak performance before combat readiness decays.",
        |s| CellValue::Int(s.peak_cr_sec as i64)
    ),
    col!(
        "cr_loss_per_sec",
        "CRloss",
        true,
        false,
        Some(false),
        60.0,
        "Combat readiness lost per second past peak. Lower is better.",
        |s| CellValue::Float(s.cr_loss_per_sec as f64)
    ),
];

impl TableEntity for Ship {
    fn row_id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn sprite_path(&self) -> Option<&Path> {
        Some(&self.layout.sprite)
    }

    fn columns() -> &'static [Column<Self>] {
        COLUMNS
    }
}
