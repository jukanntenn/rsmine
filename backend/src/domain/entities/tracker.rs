use serde::{Deserialize, Serialize};

/// Tracker domain entity (pure Rust struct, independent of any ORM)
/// Represents issue tracker types like Bug, Feature, Support, etc.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tracker {
    pub id: i32,
    pub name: String,
    pub position: Option<i32>,
    pub is_in_roadmap: bool,
    pub fields_bits: Option<i32>,
    pub default_status_id: i32,
}

impl Tracker {
    /// Get the list of enabled standard fields based on fields_bits
    /// This maps the bits to field names
    pub fn enabled_standard_fields(&self) -> Vec<String> {
        // Standard fields that can be enabled/disabled per tracker
        // These are the common fields from Redmine
        let all_fields = vec![
            "assigned_to_id",
            "category_id",
            "fixed_version_id",
            "parent_issue_id",
            "start_date",
            "due_date",
            "estimated_hours",
            "done_ratio",
            "description",
            "priority_id",
        ];

        // If no fields_bits is set, return all fields (default behavior)
        let bits = match self.fields_bits {
            Some(b) => b,
            None => return all_fields.iter().map(|s| s.to_string()).collect(),
        };

        // Each bit represents whether a field is enabled
        // If all bits are 0 or unset, return all fields
        if bits == 0 {
            return all_fields.iter().map(|s| s.to_string()).collect();
        }

        // Map bits to fields
        all_fields
            .iter()
            .enumerate()
            .filter(|(i, _)| (bits & (1 << i)) != 0 || bits == 0)
            .map(|(_, field)| field.to_string())
            .collect()
    }
}

/// Default status info for API response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultStatus {
    pub id: i32,
    pub name: String,
}
