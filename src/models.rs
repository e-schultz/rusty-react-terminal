use serde::{Deserialize, Serialize};

/// A single pattern entry with description, signals, and protocol
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Entry {
    pub pattern: String,
    pub description: String,
    pub signals: Vec<String>,
    pub protocol: String,
}

/// A section containing multiple entries with styling metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Section {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub color: String,
    pub entries: Vec<Entry>,
}

/// Metadata for the entire guide
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Meta {
    pub title: String,
    pub subtitle: String,
    pub version: String,
    pub context: String,
    pub date: String,
}

/// Complete Field Guide data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldGuideData {
    pub meta: Meta,
    pub sections: Vec<Section>,
}

/// A record within a sanctuary program
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Record {
    pub id: String,
    pub name: String,
    pub description: String,
    pub steps: Vec<String>,
    pub status: RecordStatus,
}

/// Status of a record
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecordStatus {
    #[serde(rename = "OPERATIONAL")]
    Operational,
    #[serde(rename = "CAUTION")]
    Caution,
    #[serde(rename = "DEGRADED")]
    Degraded,
    #[serde(rename = "CRITICAL")]
    Critical,
}

impl std::fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RecordStatus::Operational => write!(f, "OPERATIONAL"),
            RecordStatus::Caution => write!(f, "CAUTION"),
            RecordStatus::Degraded => write!(f, "DEGRADED"),
            RecordStatus::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// A program containing multiple records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub id: String,
    pub title: String,
    pub icon: String,
    pub color: String,
    pub records: Vec<Record>,
}

/// System metrics for the sanctuary
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metrics {
    pub focus: u8,
    pub energy: u8,
    pub stability: u8,
    pub creativity: u8,
    pub last_reset: String,
}

/// Sanctuary metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanctuaryMeta {
    pub title: String,
    pub subtitle: String,
    pub version: String,
    pub status: String,
}

/// Complete Glitch Sanctuary data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlitchSanctuaryData {
    pub meta: SanctuaryMeta,
    pub metrics: Metrics,
    pub programs: Vec<Program>,
}

/// Available views in the application
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum View {
    FieldGuide,
    GlitchSanctuary,
}

impl std::fmt::Display for View {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            View::FieldGuide => write!(f, "field-guide"),
            View::GlitchSanctuary => write!(f, "glitch-sanctuary"),
        }
    }
}

impl View {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "field-guide" | "guide" => Some(View::FieldGuide),
            "glitch-sanctuary" | "sanctuary" => Some(View::GlitchSanctuary),
            _ => None,
        }
    }
}
