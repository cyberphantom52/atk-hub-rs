use libatk_rs::prelude::*;

#[derive(Command, Default, Debug)]
pub struct GetBatteryStatus {
    pub level: u8,
    pub charge: u8,
    pub voltage: f32,
}

#[command_extension]
impl Command<GetBatteryStatus> {
    pub fn query() -> Command<GetBatteryStatus> {
        let mut command = Command::default();

        command.set_id(CommandId::GetBatteryLevel);

        command
    }

    pub fn level(&self) -> u8 {
        self.data()[0x0]
    }

    pub fn charge(&self) -> u8 {
        self.data()[0x1]
    }

    pub fn voltage(&self) -> f32 {
        self.data()[0x2] as f32 / 10f32
    }

    pub fn config(self) -> GetBatteryStatus {
        GetBatteryStatus {
            level: self.level(),
            charge: self.charge(),
            voltage: self.voltage(),
        }
    }
}
