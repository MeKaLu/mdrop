use std::fmt::Display;

use clap::ValueEnum;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum Filter {
    #[default]
    #[clap(alias = "froll")]
    FastRollOffLowLatency = 0,
    #[clap(alias = "fropc")]
    FastRollOffPhaseCompensated = 1,
    #[clap(alias = "sroll")]
    SlowRollOffLowLatency = 2,
    #[clap(alias = "sropc")]
    SlowRollOffPhaseCompensated = 3,
    #[clap(alias = "no")]
    NonOversampling = 4,
}

impl Filter {
    pub const ALL: [Filter; 5] = [
        Filter::FastRollOffLowLatency,
        Filter::FastRollOffPhaseCompensated,
        Filter::SlowRollOffLowLatency,
        Filter::SlowRollOffPhaseCompensated,
        Filter::NonOversampling,
    ];
}

impl From<u8> for Filter {
    fn from(value: u8) -> Self {
        match value {
            0 => Filter::FastRollOffLowLatency,
            1 => Filter::FastRollOffPhaseCompensated,
            2 => Filter::SlowRollOffLowLatency,
            3 => Filter::SlowRollOffPhaseCompensated,
            4 => Filter::NonOversampling,
            _ => Filter::FastRollOffLowLatency,
        }
    }
}

impl Display for Filter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Filter::FastRollOffLowLatency => write!(f, "Fast roll-off, low-latency"),
            Filter::FastRollOffPhaseCompensated => write!(f, "Fast roll-off, phase-compensated"),
            Filter::SlowRollOffLowLatency => write!(f, "Slow roll-off, low-latency"),
            Filter::SlowRollOffPhaseCompensated => write!(f, "Slow roll-off, phase-compensated"),
            Filter::NonOversampling => write!(f, "Non-oversampling"),
        }
    }
}
