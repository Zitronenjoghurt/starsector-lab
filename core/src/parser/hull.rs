#[derive(Debug, serde::Deserialize)]
pub struct Hull {
    /// In-game display name
    pub name: String,
    /// Unique hull identifier
    pub id: String,
    /// Role label shown under the name, e.g. Interceptor or Assault
    pub designation: Option<String>,
    /// Tech tier or maker, e.g. Low Tech, Midline, High Tech
    #[serde(alias = "tech/manufacturer")]
    pub tech_manufacturer: Option<String>,
    /// Id of the ship's active system ability
    #[serde(alias = "system id")]
    pub system_id: Option<String>,
    /// Fleet point cost to field the ship
    #[serde(alias = "fleet pts")]
    pub fleet_points: Option<u32>,
    /// Hull integrity, the health under the armor
    #[serde(alias = "hitpoints")]
    pub hit_points: u32,
    /// Total armor value
    #[serde(alias = "armor rating")]
    pub armor_rating: u32,
    /// Flux capacity before the ship overloads
    #[serde(alias = "max flux")]
    pub max_flux: u32,
    /// Minimum armor per hit as a percent of total armor, by hull size
    /// (frigate 8, destroyer 6, cruiser 5, capital 4). Header hint only, always blank.
    #[serde(alias = "8/6/5/4%")]
    pub eight_six_five_four_percent: Option<u32>,
    /// Flux vented per second
    #[serde(alias = "flux dissipation")]
    pub flux_dissipation: u32,
    /// Ordnance points for fitting weapons and hull mods
    #[serde(alias = "ordnance points")]
    pub ordnance_points: Option<u32>,
    /// Number of fighter wings the ship can carry
    #[serde(alias = "fighter bays")]
    pub fighter_bays: Option<u32>,
    /// Top speed
    #[serde(alias = "max speed")]
    pub max_speed: f32,
    /// How fast the ship reaches top speed
    pub acceleration: f32,
    /// How fast the ship slows and stops
    pub deceleration: f32,
    /// Top rotation speed in degrees per second
    #[serde(alias = "max turn rate")]
    pub max_turn_rate: f32,
    /// How fast the ship reaches its max turn rate
    #[serde(alias = "turn acceleration")]
    pub turn_acceleration: f32,
    /// Physical mass, affects collisions and momentum
    pub mass: f32,
    /// Shield behavior: NONE, FRONT, OMNI or PHASE
    #[serde(alias = "shield type")]
    pub shield_type: String,
    /// Id of a special defense mounted point (unclear, rarely populated)
    #[serde(alias = "defense id")]
    pub defense_id: Option<String>,
    /// Shield coverage in degrees
    #[serde(alias = "shield arc")]
    pub shield_arc: Option<f32>,
    /// Flux per second to keep the shield raised
    #[serde(alias = "shield upkeep")]
    pub shield_upkeep: Option<f32>,
    /// Flux generated per point of damage absorbed, lower is better
    #[serde(alias = "shield efficiency")]
    pub shield_efficiency: Option<f32>,
    /// Flux per second to enter phase
    #[serde(alias = "phase cost")]
    pub phase_cost: Option<f32>,
    /// Flux per second to stay phased
    #[serde(alias = "phase upkeep")]
    pub phase_upkeep: Option<f32>,
    /// Minimum crew needed to operate the ship
    #[serde(alias = "min crew")]
    pub min_crew: Option<u32>,
    /// Maximum crew the ship holds
    #[serde(alias = "max crew")]
    pub max_crew: Option<u32>,
    /// Cargo capacity
    pub cargo: Option<u32>,
    /// Fuel capacity
    pub fuel: Option<u32>,
    /// Fuel burned per light year of travel
    #[serde(alias = "fuel/ly")]
    pub fuel_per_ly: Option<f32>,
    /// Deprecated range field, mostly unused
    pub range: Option<f32>,
    /// Max burn level, the campaign map travel speed
    #[serde(alias = "max burn")]
    pub max_burn: Option<u32>,
    /// Base credit value of the hull
    #[serde(alias = "base value")]
    pub base_value: Option<u32>,
    /// Combat readiness recovered per day
    #[serde(alias = "cr %/day")]
    pub cr_percent_per_day: Option<f32>,
    /// Combat readiness spent to deploy the ship in battle
    #[serde(alias = "CR to deploy")]
    pub cr_to_deploy: Option<u32>,
    /// Seconds at peak performance before CR starts decaying
    #[serde(alias = "peak CR sec")]
    pub peak_cr_sec: Option<u32>,
    /// Combat readiness lost per second after peak time runs out
    #[serde(alias = "CR loss/sec")]
    pub cr_loss_per_sec: Option<f32>,
    /// Supplies needed to recover CR after a battle
    #[serde(alias = "supplies/rec")]
    pub supplies_per_recovery: Option<f32>,
    /// Supplies consumed per month for maintenance
    #[serde(alias = "supplies/mo")]
    pub supplies_per_month: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "c/s")]
    pub c_s: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "c/f")]
    pub c_f: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "f/s")]
    pub f_s: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "f/f")]
    pub f_f: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "crew/s")]
    pub crew_s: Option<f32>,
    /// Salvage or recovery cost ratio (meaning unclear, rarely populated)
    #[serde(alias = "crew/f")]
    pub crew_f: Option<f32>,
    /// Behavior and UI flags, e.g. UNBOARDABLE or HIDE_IN_CODEX
    pub hints: Option<String>,
    /// Arbitrary tags used for filtering and modding
    pub tags: Option<String>,
    /// Text shown when logistics stats do not apply, e.g. for drones
    #[serde(alias = "logistics n/a reason")]
    pub logistics_na_reason: Option<String>,
    /// Variant id used to show the ship in the in-game codex
    #[serde(alias = "codex variant id")]
    pub codex_variant_id: Option<String>,
    /// Spawn rarity weight from 0 to 1
    pub rarity: Option<f32>,
    /// Chance the hull breaks into debris instead of staying recoverable
    #[serde(alias = "breakProb")]
    pub break_prob: Option<f32>,
    /// Minimum debris pieces produced when the hull breaks
    #[serde(alias = "minPieces")]
    pub min_pieces: Option<u32>,
    /// Maximum debris pieces produced when the hull breaks
    #[serde(alias = "maxPieces")]
    pub max_pieces: Option<u32>,
    /// Flag for a travel or burn drive effect (unclear, rarely populated)
    #[serde(alias = "travel drive")]
    pub travel_drive: Option<String>,
    /// Internal ordering or index value (meaning unclear)
    pub number: Option<f32>,
}
