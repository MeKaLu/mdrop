use std::fmt::Display;

const VOLUME_MAX: u8 = 0x00;
const VOLUME_MIN: u8 = 0x70;

/// Moondrop Device Volume in Percent (Not Payload)
#[derive(Clone, Copy, Debug, Default)]
pub struct Volume(u32);

impl Volume {
    pub fn new(level: u32) -> Self {
        Self(level)
    }
    pub fn to_payload(&self) -> u8 {
        let v = (VOLUME_MIN as u32 - self.0 * VOLUME_MIN as u32 / 100) as u8 - 1;
        v.clamp(VOLUME_MAX, VOLUME_MIN)
    }

    pub fn from_payload(value: u8) -> Self {
        let val = value.clamp(VOLUME_MAX, VOLUME_MIN) as u32;
        Self((VOLUME_MIN as u32 - val) * 100 / VOLUME_MIN as u32)
    }

    pub fn inner(&self) -> u32 {
        self.0
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}%", self.0)
    }
}
