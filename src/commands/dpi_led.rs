use super::{Command, CommandDescriptor, CommandId, EEPROMAddress};
use atk_command_derive::CommandDescriptor;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum RGBLightingEffects {
    Constant = 0x1,
    Breathing = 0x2,
}

impl From<u8> for RGBLightingEffects {
    fn from(value: u8) -> Self {
        match value {
            0x1 => RGBLightingEffects::Constant,
            0x2 => RGBLightingEffects::Breathing,
            _ => panic!("Invalid RGB lighting effect"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum BreathingSpeed {
    Slow = 0x1,
    Medium = 0x3,
    Fast = 0x5,
}

impl From<u8> for BreathingSpeed {
    fn from(value: u8) -> Self {
        match value {
            0x1 => BreathingSpeed::Slow,
            0x3 => BreathingSpeed::Medium,
            0x5 => BreathingSpeed::Fast,
            _ => panic!("Invalid breathing speed"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum DpiRGBEnabled {
    Disabled = 0x0,
    Enabled = 0x1,
}

impl From<u8> for DpiRGBEnabled {
    fn from(value: u8) -> Self {
        match value {
            0x0 => DpiRGBEnabled::Disabled,
            0x1 => DpiRGBEnabled::Enabled,
            _ => panic!("Invalid DPI RGB enabled"),
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum LongBrightBrightness {
    Low = 0x10,
    Medium = 0x80,
    High = 0xff,
}

impl From<u8> for LongBrightBrightness {
    fn from(value: u8) -> Self {
        match value {
            0x10 => LongBrightBrightness::Low,
            0x80 => LongBrightBrightness::Medium,
            0xff => LongBrightBrightness::High,
            _ => panic!("Invalid long bright brightness"),
        }
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct DpiLedCommand;

impl std::fmt::Display for Command<DpiLedCommand> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB Lighting Effects: {:?}\nLong Bright Brightness: {:?}\nBreathing Speed: {:?}\nDPI RGB Enabled: {:?}",
            self.rgb_lighting_effects(),
            self.long_bright_brightness(),
            self.breathing_speed(),
            self.dpi_rgb_enabled()
        )
    }
}

impl Command<DpiLedCommand> {
    pub fn new(
        enable: DpiRGBEnabled,
        effect: RGBLightingEffects,
        brightness: LongBrightBrightness,
        speed: BreathingSpeed,
    ) -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::SetEEPROM);
        command.set_eeprom_address(EEPROMAddress::DpiRgbLightingEffects);
        command.set_valid_data_len(0x8);

        command.set_dpi_rgb_enabled(enable);
        command.set_rgb_lighting_effects(effect);
        command.set_long_bright_brightness(brightness);
        command.set_breathing_speed(speed);

        command
    }

    pub fn query() -> Self {
        let mut command = Command::default();
        command.set_id(CommandId::GetEEPROM);
        command.set_eeprom_address(EEPROMAddress::DpiRgbLightingEffects);
        command.set_valid_data_len(0x8);

        command
    }

    pub fn rgb_lighting_effects(&self) -> RGBLightingEffects {
        self.as_bytes()[DpiLedCommand::base_offset()].into()
    }

    pub fn set_rgb_lighting_effects(&mut self, value: RGBLightingEffects) {
        self.set_byte_pair(value as u8, DpiLedCommand::base_offset())
            .unwrap();
    }

    pub fn long_bright_brightness(&self) -> LongBrightBrightness {
        self.as_bytes()[DpiLedCommand::base_offset() + 0x2].into()
    }

    pub fn set_long_bright_brightness(&mut self, value: LongBrightBrightness) {
        self.set_byte_pair(value as u8, DpiLedCommand::base_offset() + 0x2)
            .unwrap();
    }

    pub fn breathing_speed(&self) -> BreathingSpeed {
        self.as_bytes()[DpiLedCommand::base_offset() + 0x4].into()
    }

    pub fn set_breathing_speed(&mut self, value: BreathingSpeed) {
        self.set_byte_pair(value as u8, DpiLedCommand::base_offset() + 0x4)
            .unwrap();
    }

    pub fn dpi_rgb_enabled(&self) -> DpiRGBEnabled {
        self.as_bytes()[DpiLedCommand::base_offset() + 0x6].into()
    }

    pub fn set_dpi_rgb_enabled(&mut self, value: DpiRGBEnabled) {
        self.set_byte_pair(value as u8, DpiLedCommand::base_offset() + 0x6)
            .unwrap();
    }
}
