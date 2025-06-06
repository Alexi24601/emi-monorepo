use std::fmt::Display;

use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

/// The possible structure settings
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StructureV5 {
    /// If the structure is enabled
    Enabled,

    /// The version for `structure`
    Version,

    /// The help  for `structure`
    Help,
}

impl Display for StructureV5 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StructureV5::Enabled => write!(f, "{}", Self::parameter_set_name()),
            StructureV5::Help => write!(f, "--help"),
            StructureV5::Version => write!(f, "--version"),
        }
    }
}

impl IntoDefault for StructureV5 {
    fn into_default(self) -> Self {
        match self {
            StructureV5::Enabled => StructureV5::Enabled,
            StructureV5::Help => StructureV5::Help,
            StructureV5::Version => StructureV5::Version,
        }
    }
}

impl Enablable for StructureV5 {
    fn is_enabler(&self) -> bool {
        matches!(self, StructureV5::Enabled)
    }

    fn enabler() -> Self {
        StructureV5::Enabled
    }
}

impl NamedParametersSet for StructureV5 {
    fn parameter_set_name() -> &'static str {
        "structure"
    }
}
