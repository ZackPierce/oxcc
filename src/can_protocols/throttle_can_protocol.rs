use board::ControlCan;
use nucleo_f767zi::can::{BaseID, CanFrame, DataFrame, ID};
use oscc_magic_byte::*;

pub const OSCC_THROTTLE_CAN_ID_INDEX: u16 = 0x90;

pub const OSCC_THROTTLE_ENABLE_CAN_ID: u16 = 0x90;

pub const OSCC_THROTTLE_DISABLE_CAN_ID: u16 = 0x91;

pub const OSCC_THROTTLE_COMMAND_CAN_ID: u16 = 0x92;

pub const OSCC_THROTTLE_REPORT_CAN_ID: u16 = 0x93;

pub const OSCC_THROTTLE_REPORT_CAN_DLC: u8 = 8;

// TODO - enum
pub const OSCC_THROTTLE_DTC_INVALID_SENSOR_VAL: u8 = 0;
pub const OSCC_THROTTLE_DTC_OPERATOR_OVERRIDE: u8 = 1;
pub const OSCC_THROTTLE_DTC_COUNT: u8 = 2;

pub struct OsccThrottleCommand {
    pub torque_request: f32,
}

impl<'a> From<&'a DataFrame> for OsccThrottleCommand {
    fn from(f: &DataFrame) -> Self {
        assert_eq!(u32::from(f.id()), OSCC_THROTTLE_COMMAND_CAN_ID as u32);
        let data = f.data();

        let raw_torque_request: u32 = data[2] as u32
            | (data[3] << 8) as u32
            | (data[4] << 16) as u32
            | (data[5] << 24) as u32;

        OsccThrottleCommand {
            torque_request: raw_torque_request as f32,
        }
    }
}

pub struct OsccThrottleReport {
    can_frame: DataFrame,
    pub enabled: bool,
    pub operator_override: bool,
    pub dtcs: u8,
}

impl OsccThrottleReport {
    pub fn new() -> Self {
        OsccThrottleReport {
            can_frame: DataFrame::new(ID::BaseID(BaseID::new(OSCC_THROTTLE_REPORT_CAN_ID))),
            enabled: false,
            operator_override: false,
            dtcs: 0,
        }
    }

    // TODO - error handling
    pub fn transmit(&mut self, can: &mut ControlCan) {
        self.update_can_frame();

        if let Err(_) = can.transmit(&self.can_frame.into()) {
            // TODO
        }
    }

    fn update_can_frame(&mut self) {
        self.can_frame
            .set_data_length(OSCC_THROTTLE_REPORT_CAN_DLC as _);

        let mut data = self.can_frame.data_as_mut();

        data[0] = OSCC_MAGIC_BYTE_0;
        data[1] = OSCC_MAGIC_BYTE_1;
        data[2] = self.enabled as _;
        data[3] = self.operator_override as _;
        data[4] = self.dtcs;
    }
}
