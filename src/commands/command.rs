use super::{CommandId, EEPROMAddress};

pub trait CommandDescriptor {
    fn base_offset() -> usize;

    fn report_id() -> u8;

    fn cmd_len() -> usize;
}

/*
| Command ID | Command Status | EEPROM Address | Data Valid Length |     Data     | Checksum |
|------------|----------------|----------------|-------------------|--------------|----------|
|   1 Byte   |     1 Byte     |     2 Bytes    |       1 Byte      |   10 Bytes   |  1 Byte  |
|------------|----------------|----------------|-------------------|--------------|----------|
base_offset: The offset of the first byte of the data field
*/
pub struct Command<T: CommandDescriptor> {
    raw: Vec<u8>,
    _cmd: std::marker::PhantomData<T>,
}

impl<T: CommandDescriptor> Default for Command<T> {
    fn default() -> Self {
        Self {
            raw: vec![0u8; T::cmd_len()],
            _cmd: std::marker::PhantomData,
        }
    }
}

impl<T: CommandDescriptor> TryFrom<&[u8]> for Command<T> {
    type Error = String;

    fn try_from(raw: &[u8]) -> Result<Self, Self::Error> {
        if raw.len() != T::cmd_len() {
            return Err(format!(
                "Invalid buffer length: expected {}, got {}",
                T::cmd_len(),
                raw.len()
            ));
        }

        Ok(Self {
            raw: raw.to_vec(),
            _cmd: std::marker::PhantomData,
        })
    }
}

impl<T: CommandDescriptor> Command<T> {
    pub fn raw_mut(&mut self) -> &mut [u8] {
        self.raw.as_mut_slice()
    }

    pub fn set_byte_pair(&mut self, value: u8, offset: usize) -> Result<(), &'static str> {
        if offset < T::base_offset() {
            return Err("Provided offset is less than the base offset");
        }

        if offset >= self.raw.len() - 2 {
            return Err("Provided offset is greater than the length of the data payload");
        }

        if (offset - T::base_offset()) % 2 != 0 {
            return Err("Provided offset is not aligned to a byte pair boundary");
        }

        self.raw[offset] = value;
        self.raw[offset + 0x1] = 0x55u8.wrapping_sub(value);

        self.set_checksum();
        Ok(())
    }

    pub fn id(&self) -> CommandId {
        self.raw[0x0].into()
    }

    pub fn set_id(&mut self, id: CommandId) {
        self.raw[0x0] = id as u8;
        self.set_checksum();
    }

    pub fn status(&self) -> u8 {
        self.raw[0x1]
    }

    pub fn set_status(&mut self, status: u8) {
        self.raw[0x1] = status;
        self.set_checksum();
    }

    pub fn eeprom_address(&self) -> EEPROMAddress {
        let addr = u16::from_be_bytes([self.raw[0x2], self.raw[0x3]]);
        addr.into()
    }

    pub fn set_eeprom_address(&mut self, address: EEPROMAddress) {
        self.raw[0x2..0x4].copy_from_slice(&(address as u16).to_be_bytes());
        self.set_checksum();
    }

    pub fn valid_data_len(&self) -> u8 {
        self.raw[0x4]
    }

    pub fn set_valid_data_len(&mut self, len: u8) {
        self.raw[0x4] = len;
        self.set_checksum();
    }

    pub fn checksum(&self) -> u8 {
        self.raw[0xf]
    }

    pub fn set_checksum(&mut self) {
        let sum: u8 = {
            let sum_bytes: u16 = self.raw[0..0xf]
                .iter()
                .fold(0, |acc, &byte| acc + byte as u16);
            ((T::report_id() as u16 + sum_bytes) & 0xff) as u8
        };
        let checksum = 0x55u8.wrapping_sub(sum);
        self.raw[0xf] = checksum;
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_slice()
    }
}
