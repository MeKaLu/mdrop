use std::fmt::Display;

use clap::ValueEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum IndicatorState {
    #[default]
    Enabled = 0,
    DisabledTemp = 1,
    Disabled = 2,
}

impl IndicatorState {
    pub const ALL: [IndicatorState; 3] = [
        IndicatorState::Enabled,
        IndicatorState::DisabledTemp,
        IndicatorState::Disabled,
    ];
}

impl From<u8> for IndicatorState {
    fn from(value: u8) -> Self {
        match value {
            0 => IndicatorState::Enabled,
            1 => IndicatorState::DisabledTemp,
            2 => IndicatorState::Disabled,
            _ => IndicatorState::Enabled,
        }
    }
}

impl Display for IndicatorState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndicatorState::Enabled => write!(f, "Enabled"),
            IndicatorState::DisabledTemp => write!(f, "Temporarily Disabled"),
            IndicatorState::Disabled => write!(f, "Disabled"),
        }
    }
}
