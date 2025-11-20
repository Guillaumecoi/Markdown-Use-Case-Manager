//! Custom field types for methodologies.
//!
//! Custom fields extend the standard use case fields and are only available
//! in specific methodologies. They allow methodologies to capture specialized
//! information relevant to their documentation style.

/// Configuration for custom fields specific to a methodology.
///
/// Custom fields extend the standard use case fields and are only available
/// in specific methodologies. They allow methodologies to capture specialized
/// information relevant to their documentation style.
///
/// # Example
///
/// ```toml
/// [custom_fields.business_value]
/// label = "Business Value"
/// type = "string"
/// required = false
/// default = "To be determined"
/// description = "The business value this feature provides"
/// example = "Reduce support costs by 30% while improving customer satisfaction"
///
/// [custom_fields.roi_estimate]
/// label = "ROI Estimate"
/// type = "string"
/// required = false
/// description = "Estimated return on investment"
/// example = "180% ROI within 18 months, annual savings of $120,000"
/// ```
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct CustomFieldConfig {
    /// Human-readable label displayed in prompts and documentation
    /// If not provided, the field name will be used (converted to title case)
    #[serde(default)]
    pub label: Option<String>,
    /// Data type of the field: "string", "array", "number", "boolean", "text"
    #[serde(rename = "type")]
    pub field_type: String,
    /// Whether this field must be provided when creating a use case with this methodology
    #[serde(default)]
    pub required: bool,
    /// Default value if none provided (None means no default)
    #[serde(default)]
    pub default: Option<String>,
    /// Description of the field (used for verbose output and documentation)
    #[serde(default)]
    pub description: Option<String>,
    /// Example value to guide users during interactive input
    /// Displayed as placeholder or help text in CLI prompts
    #[serde(default)]
    pub example: Option<String>,
}
