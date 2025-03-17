use std::collections::HashMap;

use filter::Filter;
use futures_lite::future;
use gain::Gain;
use indicator_state::IndicatorState;
use nusb::transfer::{ControlIn, ControlOut};
use nusb::{DeviceId, DeviceInfo};
use tabled::Tabled;

pub mod filter;
pub mod gain;
pub mod indicator_state;
pub mod volume_level;

pub const MOONDROP_VID: u16 = 0x2fc6;
pub const DAWN_PRO_PID: u16 = 0xf06a;

const REQUEST_INDEX: u16 = 0x09A0;
const REQUEST_VALUE: u16 = 0x0000;

const REQUEST_ID_WRITE: u8 = 0xA0;
const REQUEST_ID_READ: u8 = 0xA1;

const GET_ANY: [u8; 3] = [0xC0, 0xA5, 0xA3];
const GET_VOLUME: [u8; 3] = [0xC0, 0xA5, 0xA2];
const SET_FILTER: [u8; 3] = [0xC0, 0xA5, 0x01];
const SET_GAIN: [u8; 3] = [0xC0, 0xA5, 0x02];
const SET_VOLUME: [u8; 3] = [0xC0, 0xA5, 0x04];
const SET_INDICATOR_STATE: [u8; 3] = [0xC0, 0xA5, 0x06];

const VOLUME_IDX: usize = 4;
const FILTER_IDX: usize = 3;
const GAIN_IDX: usize = 4;
const INDICATOR_STATE_IDX: usize = 5;

pub struct Moondrop {
    devices: HashMap<DeviceId, DeviceInfo>,
}

impl Moondrop {
    pub fn new() -> Self {
        let devices = nusb::list_devices()
            .unwrap()
            .filter(|d| d.vendor_id() == MOONDROP_VID)
            .map(|d| (d.id(), d))
            .collect();
        Self { devices }
    }

    pub fn detect(&self) -> Vec<MoondropInfo> {
        self.devices
            .values()
            .map(|di| {
                let name = match di.product_string() {
                    Some(name) => name.to_string(),
                    None => match di.product_id() {
                        DAWN_PRO_PID => "MOONDROP Dawn Pro".to_string(),
                        _ => "Unknown".to_string(),
                    },
                };

                let vol = Self::read(di, &GET_VOLUME, 7)[VOLUME_IDX];
                let bus = format!("{:02}:{:02}", di.bus_number(), di.device_address());
                MoondropInfo::new(
                    name,
                    bus,
                    format!("{:02}%", volume_level::convert_volume_to_percent(vol)),
                )
            })
            .collect()
    }

    pub fn get_volume(&self) -> u8 {
        let data: &[u8] = &self
            .devices
            .iter()
            .take(1)
            .map(|(_, di)| Self::read(di, &GET_VOLUME, 7))
            .collect::<Vec<Vec<u8>>>()[0];

        data[VOLUME_IDX]
    }

    pub fn get_all(&self) -> MoondropResponse {
        let data: &[u8] = &self
            .devices
            .iter()
            .take(1)
            .map(|(_, di)| Self::read(di, &GET_ANY, 7))
            .collect::<Vec<Vec<u8>>>()[0];
        MoondropResponse::new(data)
    }

    pub fn set_gain(&self, gain: Gain) {
        let mut cmd = Vec::from(SET_GAIN);
        cmd.push(gain as u8);
        println!("Gain Command: {:?}", cmd);
        self.devices.iter().for_each(|(_, di)| {
            Self::write(di, &cmd);
        });
    }

    pub fn set_volume(&self, level: u8) {
        let value = volume_level::convert_volume_to_payload(level);
        println!("Volume Level: {level} clamped: {value}");
        let mut cmd = Vec::from(SET_VOLUME);
        // FIXME: might be incorrect
        cmd.push(value);
        println!("Volume Command: {:?}", cmd);
        self.devices.iter().for_each(|(_, di)| {
            Self::write(di, &cmd);
        });
    }

    pub fn set_filter(&self, filter: Filter) {
        let mut cmd = Vec::from(SET_FILTER);
        cmd.push(filter as u8);
        println!("Filter Command: {:?}", cmd);
        self.devices.iter().for_each(|(_, di)| {
            Self::write(di, &cmd);
        });
    }

    pub fn set_indicator_state(&self, indicator_state: IndicatorState) {
        let mut cmd = Vec::from(SET_INDICATOR_STATE);
        cmd.push(indicator_state as u8);
        println!("IndicatorState Command: {:?}", cmd);
        self.devices.iter().for_each(|(_, di)| {
            Self::write(di, &cmd);
        });
    }

    fn read(di: &DeviceInfo, cmd: &[u8], length: u16) -> Vec<u8> {
        let device = di.open().expect("device open failed");
        let _ = future::block_on(device.control_out(ControlOut {
            control_type: nusb::transfer::ControlType::Vendor,
            recipient: nusb::transfer::Recipient::Other,
            request: REQUEST_ID_WRITE,
            value: REQUEST_VALUE,
            index: REQUEST_INDEX,
            data: cmd,
        }))
        .into_result()
        .expect("write failed");
        future::block_on(device.control_in(ControlIn {
            control_type: nusb::transfer::ControlType::Vendor,
            recipient: nusb::transfer::Recipient::Other,
            request: REQUEST_ID_READ,
            value: REQUEST_VALUE,
            index: REQUEST_INDEX,
            length,
        }))
        .into_result()
        .expect("read failed")
    }

    fn write(di: &DeviceInfo, cmd: &[u8]) {
        let device = di.open().expect("device open failed");
        let _ = future::block_on(device.control_out(ControlOut {
            control_type: nusb::transfer::ControlType::Vendor,
            recipient: nusb::transfer::Recipient::Other,
            request: REQUEST_ID_WRITE,
            value: REQUEST_VALUE,
            index: REQUEST_INDEX,
            data: cmd,
        }))
        .into_result()
        .expect("write failed");
    }
}

impl Default for Moondrop {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MoondropResponse {
    pub filter: Filter,
    pub gain: Gain,
    pub state: IndicatorState,
}

impl MoondropResponse {
    pub fn new(data: &[u8]) -> Self {
        let filter = Filter::from(data[FILTER_IDX]);
        let gain = Gain::from(data[GAIN_IDX]);
        let state = IndicatorState::from(data[INDICATOR_STATE_IDX]);
        Self {
            filter,
            gain,
            state,
        }
    }
}

#[derive(Tabled)]
#[tabled(rename_all = "pascal")]
pub struct MoondropInfo {
    name: String,
    bus: String,
    volume: String,
}

impl MoondropInfo {
    pub fn new(name: String, bus: String, volume: String) -> Self {
        Self { name, bus, volume }
    }
}
