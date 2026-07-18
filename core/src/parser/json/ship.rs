use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ShipFile {
    /// Unique hull identifier, matches the CSV `id`
    pub hull_id: String,
    /// In-game display name
    pub hull_name: String,
    /// Hull size class, e.g. FRIGATE, DESTROYER, CRUISER, CAPITAL_SHIP, FIGHTER
    pub hull_size: String,
    /// Visual style preset, e.g. HIGH_TECH, drives engine glow and effects
    pub style: String,
    /// Sprite path including the `graphics/` prefix, e.g. `graphics/ships/foo.png`, resolved against the Starsector core or mod root
    pub sprite_name: String,
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
    /// Overlay tint color, usually left empty
    #[serde(default)]
    pub covers_color: String,
    /// Anchor point relative to the parent hull when this ship is a station module, `[x, y]`
    pub module_anchor: Option<[f32; 2]>,
    /// Collision outline as a flat list of `x, y` pairs
    #[serde(default)]
    pub bounds: Vec<f32>,
    /// Ids of hull mods built into the ship
    #[serde(default)]
    pub built_in_mods: Vec<String>,
    /// Wing ids for fighters permanently attached to the ship
    #[serde(default)]
    pub built_in_wings: Vec<String>,
    /// Weapons fixed to specific slots, keyed by weapon-slot id
    #[serde(default)]
    pub built_in_weapons: HashMap<String, String>,
    /// Engine glow / thruster placements
    #[serde(default)]
    pub engine_slots: Vec<EngineSlot>,
    /// Weapon mount placements
    #[serde(default)]
    pub weapon_slots: Vec<WeaponSlot>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EngineSlot {
    /// Direction the engine points, in degrees
    pub angle: f32,
    /// Sprite-space position of the engine, `[x, y]`
    pub location: [f32; 2],
    /// Length of the engine flame
    pub length: f32,
    /// Width of the engine flame
    pub width: f32,
    /// Size of the trailing contrail effect
    pub contrail_size: f32,
    /// Visual style preset for the engine glow, e.g. HIGH_TECH
    pub style: String,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeaponSlot {
    /// Slot identifier, unique within the ship, e.g. "WS 000"
    pub id: String,
    /// Facing of the mount relative to the hull, in degrees
    pub angle: f32,
    /// Firing arc the mount can traverse, in degrees
    pub arc: f32,
    /// Mount position(s) as a flat list of `x, y` pairs. Usually one point, but
    /// some mounts (e.g. LAUNCH_BAY) list multiple.
    pub locations: Vec<f32>,
    /// How the weapon is mounted, e.g. TURRET, HARDPOINT, HIDDEN
    pub mount: String,
    /// Mount size, e.g. SMALL, MEDIUM, LARGE
    pub size: String,
    /// Accepted weapon category, e.g. ENERGY, BALLISTIC, MISSILE, SYNERGY, LAUNCH_BAY
    #[serde(rename = "type")]
    pub weapon_type: String,
}
