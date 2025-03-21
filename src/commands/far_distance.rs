use libatk_rs::prelude::*;

#[derive(Command, Default, Debug)]
pub struct FarDistanceMode(bool);

impl std::fmt::Display for FarDistanceMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ultra Long Range Mode: {}", self.far_distance_mode())
    }
}

#[allow(dead_code)]
impl FarDistanceMode {
    pub fn far_distance_mode(&self) -> bool {
        self.0
    }

    pub fn builder(&self) -> CommandBuilder<FarDistanceMode> {
        Command::builder().far_distance_mode(self.far_distance_mode())
    }
}

#[command_extension]
impl Command<FarDistanceMode> {
    pub fn builder() -> CommandBuilder<FarDistanceMode> {
        let mut command = Command::default();

        command.set_id(CommandId::SetFarDistanceMode);

        CommandBuilder::new(command)
    }

    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetFarDistanceMode);

        command
    }

    pub fn config(self) -> FarDistanceMode {
        FarDistanceMode(self.data()[0x0] == 0x01)
    }

    pub fn set_far_distance_mode(&mut self, mode: bool) {
        self.set_data_byte(mode as u8, 0x0)
            .expect("Failed to set far distance mode");
    }
}
