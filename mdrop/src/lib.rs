use std::collections::HashMap;

use futures_lite::future;
use nusb::transfer::{ControlIn, ControlOut};
use nusb::{DeviceId, DeviceInfo};
use tabled::Tabled;

use crate::filter::Filter;
use crate::gain::Gain;
use crate::indicator_state::IndicatorState;
use crate::volume::Volume;

pub mod filter;
pub mod gain;
pub mod indicator_state;
pub mod volume;

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
    single: Option<DeviceId>,
}

impl Moondrop {
    pub fn new() -> Self {
        let devices: HashMap<DeviceId, DeviceInfo> = nusb::list_devices()
            .unwrap()
            .filter(|d| d.vendor_id() == MOONDROP_VID)
            .map(|d| (d.id(), d))
            .collect();
        let single = if devices.len() == 1 {
            devices.keys().next().cloned()
        } else {
            None
        };
        Self { devices, single }
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

                let vol_data = Self::read(di, &GET_VOLUME, 7)[VOLUME_IDX];
                let vol = Volume::from_payload(vol_data);
                let bus = format!("{:02}:{:02}", di.bus_number(), di.device_address());
                let data = Self::read(di, &GET_ANY, 7);
                MoondropInfo::new(name, bus, vol, &data)
            })
            .collect()
    }

    pub fn get_volume(&self) -> Volume {
        if let Some(id) = self.single {
            let di = self.devices.get(&id).expect("Hashmap should be populated");
            let data = Self::read(di, &GET_VOLUME, 7);
            return Volume::from_payload(data[VOLUME_IDX]);
        }

        // TODO: fix
        Volume::from_payload(0)
        // let data: &[u8] = &self
        //     .devices
        //     .iter()
        //     .take(1)
        //     .map(|(_, di)| Self::read(di, &GET_VOLUME, 7))
        //     .collect::<Vec<Vec<u8>>>()[0];
        //
        // data[VOLUME_IDX]
    }

    pub fn get_all(&self) -> Option<MoondropInfo> {
        if let Some(id) = self.single {
            let di = self.devices.get(&id).expect("Hashmap should be populated");
            let name = match di.product_string() {
                Some(name) => name.to_string(),
                None => match di.product_id() {
                    DAWN_PRO_PID => "MOONDROP Dawn Pro".to_string(),
                    _ => "Unknown".to_string(),
                },
            };

            let vol_data = Self::read(di, &GET_VOLUME, 7)[VOLUME_IDX];
            let vol = Volume::from_payload(vol_data);
            let bus = format!("{:02}:{:02}", di.bus_number(), di.device_address());
            let data = Self::read(di, &GET_ANY, 7);
            return Some(MoondropInfo::new(name, bus, vol, &data));
        }

        None

    }

    pub fn set_gain(&self, gain: Gain) {
        let mut cmd = Vec::from(SET_GAIN);
        cmd.push(gain as u8);
        println!("Gain Command: {:?}", cmd);
        self.devices.iter().for_each(|(_, di)| {
            Self::write(di, &cmd);
        });
    }

    pub fn set_volume(&self, level: Volume) {
        let value = level.to_payload();
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

#[derive(Tabled)]
#[tabled(rename_all = "snake")]
pub struct MoondropInfo {
    pub name: String,
    pub bus: String,
    pub volume: Volume,
    pub filter: Filter,
    pub gain: Gain,
    pub indicator_state: IndicatorState,
}

impl MoondropInfo {
    pub fn new(name: String, bus: String, volume: Volume, data: &[u8]) -> Self {
        let filter = Filter::from(data[FILTER_IDX]);
        let gain = Gain::from(data[GAIN_IDX]);
        let state = IndicatorState::from(data[INDICATOR_STATE_IDX]);
        Self {
            name,
            bus,
            volume,
            filter,
            gain,
            indicator_state: state,
        }
    }
}
