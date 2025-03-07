use libatk_rs::prelude::*;

#[derive(Command)]
pub struct StartPairing;

#[command_extension]
impl Command<StartPairing> {
    pub fn builder() -> CommandBuilder<StartPairing> {
        let mut command = Command::default();
        command.set_id(CommandId::SetWirelessDonglePair);
        CommandBuilder::new(command)
    }

    pub fn set_cid(&mut self, cid: u8) {
        self.set_data_byte(cid, 0x0).expect("Failed to set CID");
    }

    pub fn set_mid(&mut self, mid: u8) {
        self.set_data_byte(mid, 0x1).expect("Failed to set MID");
    }
}

#[derive(Command)]
pub struct GetPairingStatus;

#[command_extension]
impl Command<GetPairingStatus> {
    pub fn query() -> Self {
        let mut command = Command::default();
        command.set_id(CommandId::GetWirelessDonglePairResult);
        command
    }

    pub fn pair_status(&mut self) -> u8 {
        self.data()[0x0]
    }

    pub fn pair_time_left(&mut self) -> u8 {
        self.data()[0x1]
    }
}

#[derive(Command)]
pub struct ExitPairing;

#[command_extension]
impl Command<ExitPairing> {
    pub fn query() -> Self {
        let mut command = Command::default();
        command.set_id(CommandId::DongleExitPair);
        command
    }
}
