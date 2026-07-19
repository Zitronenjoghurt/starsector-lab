use crate::error::LabResult;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct WeaponSpec {
    pub id: String,
    /// Mount type: BALLISTIC, ENERGY, MISSILE, etc.
    pub mount_type: String,
    /// Mount size: SMALL, MEDIUM or LARGE.
    pub mount_size: String,
    /// Projectile behavior class, e.g. projectile, beam, missile.
    pub spec_class: Option<String>,
    /// Sprite path relative to the core root (turret preferred, else hardpoint).
    pub sprite: Option<String>,
}

pub fn read_weapon_specs(dir: &Path) -> LabResult<HashMap<String, WeaponSpec>> {
    let mut specs = HashMap::new();
    for entry in std::fs::read_dir(dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("wpn") {
            continue;
        }

        let raw = std::fs::read_to_string(&path)?;
        let text = strip_comment_lines(&raw);

        let id = field(&text, "id").unwrap_or_else(|| {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
        });
        let sprite = field(&text, "turretSprite")
            .filter(|s| !s.is_empty())
            .or_else(|| field(&text, "hardpointSprite").filter(|s| !s.is_empty()));

        specs.insert(
            id.clone(),
            WeaponSpec {
                id,
                mount_type: field(&text, "type").unwrap_or_default(),
                mount_size: field(&text, "size").unwrap_or_default(),
                spec_class: field(&text, "specClass"),
                sprite,
            },
        );
    }
    Ok(specs)
}

fn strip_comment_lines(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim_start().starts_with('#'))
        .collect::<Vec<_>>()
        .join("\n")
}

fn field(text: &str, key: &str) -> Option<String> {
    let needle = format!("\"{key}\"");
    let after_key = &text[text.find(&needle)? + needle.len()..];
    let after_colon = &after_key[after_key.find(':')? + 1..];
    let after_open = &after_colon[after_colon.find('"')? + 1..];
    let end = after_open.find('"')?;
    Some(after_open[..end].to_owned())
}
