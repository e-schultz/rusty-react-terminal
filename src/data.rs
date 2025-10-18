use crate::models::{FieldGuideData, GlitchSanctuaryData};
use anyhow::{Context, Result};
use std::path::Path;

/// Load Field Guide data from a TOML file
pub fn load_field_guide<P: AsRef<Path>>(path: P) -> Result<FieldGuideData> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read field guide from {:?}", path))?;

    let data: FieldGuideData = toml::from_str(&content)
        .with_context(|| format!("Failed to parse field guide TOML from {:?}", path))?;

    Ok(data)
}

/// Load Glitch Sanctuary data from a TOML file
pub fn load_sanctuary<P: AsRef<Path>>(path: P) -> Result<GlitchSanctuaryData> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read sanctuary data from {:?}", path))?;

    let data: GlitchSanctuaryData = toml::from_str(&content)
        .with_context(|| format!("Failed to parse sanctuary TOML from {:?}", path))?;

    Ok(data)
}

/// Load Field Guide data from a JSON file
pub fn load_field_guide_json<P: AsRef<Path>>(path: P) -> Result<FieldGuideData> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read field guide from {:?}", path))?;

    let data: FieldGuideData = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse field guide JSON from {:?}", path))?;

    Ok(data)
}

/// Load Glitch Sanctuary data from a JSON file
pub fn load_sanctuary_json<P: AsRef<Path>>(path: P) -> Result<GlitchSanctuaryData> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read sanctuary data from {:?}", path))?;

    let data: GlitchSanctuaryData = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse sanctuary JSON from {:?}", path))?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_guide_loading() {
        // This will be tested with actual data files
        // Placeholder for now
    }

    #[test]
    fn test_sanctuary_loading() {
        // This will be tested with actual data files
        // Placeholder for now
    }
}
