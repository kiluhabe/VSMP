#[allow(dead_code)]
pub enum Command {
    PanelSetting,
    PowerSetting,
    PowerOff,
    PowerOffSequenceSetting,
    PowerOn,
    PowerOnMeasure,
    BoosterSoftStart,
    DeepSleep,
    DataStartTransmission1,
    DataStop,
    DisplayRefresh,
    ImageProcess,
    LUTForVCOM,
    LUTBlue,
    LUTWhite,
    LUTGray1,
    LUTGray2,
    LUTRed0,
    LUTRed1,
    LUTRed2,
    LUTRed3,
    LUTXon,
    PLLControl,
    TemperatureSensor,
    TemperatureCalibration,
    TemperatureSensorWrite,
    TemperatureSensorRead,
    VCOMAndDataIntervalSetting,
    LowPowerDetection,
    TCONSetting,
    TCONResolution,
    SPIFlashControl,
    Revision,
    GetStatus,
    AutoMeasurementVCOM,
    ReadVCOMValue,
    VCMDCSetting,
    FlashMode,
}

impl Command {
    pub fn value(&self) -> u8 {
        match *self {
            Command::PanelSetting => 0x00,
            Command::PowerSetting => 0x01,
            Command::PowerOff => 0x02,
            Command::PowerOffSequenceSetting => 0x03,
            Command::PowerOn => 0x04,
            Command::PowerOnMeasure => 0x05,
            Command::BoosterSoftStart => 0x06,
            Command::DeepSleep => 0x07,
            Command::DataStartTransmission1 => 0x10,
            Command::DataStop => 0x11,
            Command::DisplayRefresh => 0x12,
            Command::ImageProcess => 0x13,
            Command::LUTForVCOM => 0x20,
            Command::LUTBlue => 0x21,
            Command::LUTWhite => 0x22,
            Command::LUTGray1 => 0x23,
            Command::LUTGray2 => 0x24,
            Command::LUTRed0 => 0x25,
            Command::LUTRed1 => 0x26,
            Command::LUTRed2 => 0x27,
            Command::LUTRed3 => 0x28,
            Command::LUTXon => 0x29,
            Command::PLLControl => 0x30,
            Command::TemperatureSensor => 0x40,
            Command::TemperatureCalibration => 0x41,
            Command::TemperatureSensorWrite => 0x42,
            Command::TemperatureSensorRead => 0x43,
            Command::VCOMAndDataIntervalSetting => 0x50,
            Command::LowPowerDetection => 0x51,
            Command::TCONSetting => 0x60,
            Command::TCONResolution => 0x61,
            Command::SPIFlashControl => 0x65,
            Command::Revision => 0x70,
            Command::GetStatus => 0x71,
            Command::AutoMeasurementVCOM => 0x80,
            Command::ReadVCOMValue => 0x81,
            Command::VCMDCSetting => 0x82,
            Command::FlashMode => 0xe5,
        }
    }
}
