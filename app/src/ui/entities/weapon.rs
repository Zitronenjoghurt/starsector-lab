use crate::ui::widgets::table::{CellValue, Column, TableEntity};
use starsector_lab::data::weapon::Weapon;
use std::path::Path;

fn opt_text(value: Option<&str>) -> CellValue {
    value.map_or(CellValue::Empty, |s| CellValue::Text(s.to_owned()))
}

fn f(value: Option<f32>) -> CellValue {
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

const COLUMNS: &[Column<Weapon>] = &[
    // Identity
    col!(
        "name",
        "Name",
        false,
        true,
        None,
        160.0,
        "In-game display name of the weapon.",
        |w| CellValue::Text(w.name.clone())
    ),
    col!(
        "role",
        "Role",
        false,
        true,
        None,
        110.0,
        "Primary role label shown in the weapon tooltip.",
        |w| opt_text(w.primary_role.as_deref())
    ),
    col!(
        "mount_size",
        "Size",
        false,
        true,
        None,
        70.0,
        "Mount size the weapon fits: SMALL, MEDIUM or LARGE.",
        |w| CellValue::Text(w.mount_size.clone())
    ),
    col!(
        "mount_type",
        "Mount",
        false,
        true,
        None,
        80.0,
        "Mount type the weapon fits: BALLISTIC, ENERGY, MISSILE, etc.",
        |w| CellValue::Text(w.mount_type.clone())
    ),
    col!(
        "damage_type",
        "Dmg type",
        false,
        true,
        None,
        90.0,
        "Damage type: KINETIC, HIGH_EXPLOSIVE, ENERGY or FRAGMENTATION.",
        |w| opt_text(w.damage_type.as_deref())
    ),
    col!(
        "class",
        "Class",
        false,
        false,
        None,
        80.0,
        "Projectile behavior class, e.g. projectile, beam, missile.",
        |w| opt_text(w.spec_class.as_deref())
    ),
    col!(
        "tech",
        "Tech",
        false,
        false,
        None,
        90.0,
        "Tech tier or manufacturer faction.",
        |w| opt_text(w.tech_manufacturer.as_deref())
    ),
    col!(
        "tier",
        "Tier",
        true,
        false,
        None,
        44.0,
        "Balance tier, roughly how advanced the weapon is.",
        |w| f(w.tier)
    ),
    // Cost
    col!(
        "op",
        "OP",
        true,
        true,
        Some(false),
        44.0,
        "Ordnance points to mount the weapon. Lower is cheaper.",
        |w| f(w.ordnance_points)
    ),
    col!(
        "value",
        "Value",
        true,
        false,
        Some(true),
        70.0,
        "Base credit value.",
        |w| f(w.base_value)
    ),
    // Offense
    col!(
        "range",
        "Range",
        true,
        true,
        Some(true),
        60.0,
        "Effective range.",
        |w| f(w.range)
    ),
    col!(
        "dps",
        "DPS",
        true,
        true,
        Some(true),
        60.0,
        "Sustained damage per second.",
        |w| f(w.damage_per_second)
    ),
    col!(
        "dmg_shot",
        "Dmg/shot",
        true,
        true,
        Some(true),
        70.0,
        "Damage dealt per individual shot.",
        |w| f(w.damage_per_shot)
    ),
    col!(
        "emp",
        "EMP",
        true,
        false,
        Some(true),
        55.0,
        "EMP damage, disables weapons and engines.",
        |w| f(w.emp)
    ),
    col!(
        "impact",
        "Impact",
        true,
        false,
        Some(true),
        60.0,
        "Hit strength used against armor.",
        |w| f(w.impact)
    ),
    col!(
        "turn_rate",
        "Turn",
        true,
        false,
        Some(true),
        55.0,
        "Turret turn rate in degrees per second.",
        |w| f(w.turn_rate)
    ),
    // Ammo / firing
    col!(
        "ammo",
        "Ammo",
        true,
        false,
        Some(true),
        55.0,
        "Ammo capacity, absent for unlimited-ammo weapons.",
        |w| f(w.ammo)
    ),
    col!(
        "ammo_sec",
        "Ammo/s",
        true,
        false,
        Some(true),
        60.0,
        "Ammo regenerated per second.",
        |w| f(w.ammo_per_sec)
    ),
    col!(
        "reload",
        "Reload",
        true,
        false,
        Some(true),
        60.0,
        "Ammo restored per reload.",
        |w| f(w.reload_size)
    ),
    col!(
        "flux_shot",
        "Flux/shot",
        true,
        false,
        Some(false),
        70.0,
        "Flux cost per shot. Lower is cheaper.",
        |w| f(w.energy_per_shot)
    ),
    col!(
        "flux_sec",
        "Flux/s",
        true,
        false,
        Some(false),
        60.0,
        "Flux cost per second while firing. Lower is cheaper.",
        |w| f(w.energy_per_second)
    ),
    col!(
        "chargeup",
        "Chargeup",
        true,
        false,
        Some(false),
        70.0,
        "Seconds to charge before firing. Lower is faster.",
        |w| f(w.chargeup)
    ),
    col!(
        "chargedown",
        "Chargedn",
        true,
        false,
        Some(false),
        70.0,
        "Seconds to cool down after firing. Lower is faster.",
        |w| f(w.chargedown)
    ),
    col!(
        "burst_size",
        "Burst",
        true,
        false,
        None,
        55.0,
        "Number of shots in a burst.",
        |w| f(w.burst_size)
    ),
    col!(
        "burst_delay",
        "BurstDly",
        true,
        false,
        Some(false),
        70.0,
        "Delay between shots within a burst. Lower is faster.",
        |w| f(w.burst_delay)
    ),
    // Projectile
    col!(
        "proj_speed",
        "ProjSpd",
        true,
        false,
        Some(true),
        65.0,
        "Projectile travel speed.",
        |w| f(w.proj_speed)
    ),
    col!(
        "flight",
        "Flight",
        true,
        false,
        Some(true),
        60.0,
        "Projectile lifetime in seconds.",
        |w| f(w.flight_time)
    ),
    col!(
        "proj_hp",
        "ProjHP",
        true,
        false,
        Some(true),
        60.0,
        "Projectile hit points, if it can be shot down.",
        |w| f(w.proj_hp)
    ),
];

impl TableEntity for Weapon {
    fn row_id(&self) -> &str {
        &self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn sprite_path(&self) -> Option<&Path> {
        self.sprite.as_deref()
    }

    fn columns() -> &'static [Column<Self>] {
        COLUMNS
    }
}
