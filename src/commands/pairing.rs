use super::{Command, CommandDescriptor, CommandId};
use atk_command_derive::CommandDescriptor;

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct Pair;

impl Command<Pair> {
    pub fn pair(cid: u8, mid: u8) -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::SetWirelessDonglePair);
        command.set_valid_data_len(0x2);
        command.set_cid(cid);
        command.set_mid(mid);

        command
    }

    fn set_cid(&mut self, cid: u8) {
        self.raw_mut()[Pair::base_offset()] = cid;
    }

    fn set_mid(&mut self, mid: u8) {
        self.raw_mut()[Pair::base_offset() + 0x1] = mid;
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct PairingStatus;

impl std::fmt::Display for Command<PairingStatus> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Pairing Status: {} | Time Left: {}",
            self.pair_status(),
            self.pair_time_left()
        )
    }
}

impl Command<PairingStatus> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::GetWirelessDonglePairResult);

        command
    }

    pub fn pair_status(&self) -> u8 {
        self.as_bytes()[PairingStatus::base_offset()]
    }

    pub fn pair_time_left(&self) -> u8 {
        self.as_bytes()[PairingStatus::base_offset() + 0x1]
    }
}

#[derive(CommandDescriptor)]
#[command_descriptor(base_offset = 0x5, report_id = 0x8, cmd_len = 0x10)]
pub struct ExitPairing;

impl Command<ExitPairing> {
    pub fn query() -> Self {
        let mut command = Command::default();

        command.set_id(CommandId::DongleExitPair);

        command
    }
}
