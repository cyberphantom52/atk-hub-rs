use libatk_rs::prelude::*;

#[derive(Command, Default, Debug)]
pub struct GetBatteryStatus {
    pub level: u8,
    pub charge: u8,
    pub voltage: f32,
}

impl GetBatteryStatus {
    pub fn level(&self) -> u8 {
        self.level
    }

    pub fn charge(&self) -> u8 {
        self.charge
    }

    pub fn voltage(&self) -> f32 {
        self.voltage
    }
}

#[command_extension]
impl Command<GetBatteryStatus> {
    pub fn query() -> Command<GetBatteryStatus> {
        let mut command = Command::default();

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn config(self) -> GetBatteryStatus {
        let voltage = *self.data().get(0x2).unwrap_or(&0) as f32 / 10f32;
        GetBatteryStatus {
            level: self.data()[0x0],
            charge: self.data()[0x1],
            voltage,
        }
    }
}
