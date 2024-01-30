use crate::traits::{Enablable, IntoDefault, NamedParametersSet};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CanopusV5 {
    Enabled,
    Version,
    Help,
}

impl ToString for CanopusV5 {
    fn to_string(&self) -> String {
        match self {
            CanopusV5::Enabled => Self::parameter_set_name().to_string(),
            CanopusV5::Help => "--help".to_string(),
            CanopusV5::Version => "--version".to_string(),
        }
    }
}

impl IntoDefault for CanopusV5 {
    fn into_default(self) -> Self {
        match self {
            CanopusV5::Enabled => CanopusV5::Enabled,
            CanopusV5::Help => CanopusV5::Help,
            CanopusV5::Version => CanopusV5::Version,
        }
    }
}

impl Enablable for CanopusV5 {
    fn is_enabler(&self) -> bool {
        match self {
            CanopusV5::Enabled => true,
            _ => false,
        }
    }

    fn enabler() -> Self {
        CanopusV5::Enabled
    }
}

impl NamedParametersSet for CanopusV5 {
    fn parameter_set_name() -> &'static str {
        "canopus"
    }
}
