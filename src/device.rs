use atk_command::Command;
use hidapi::HidDevice;
static MAX_REPORT_LENGTH: usize = 64;

pub struct Device(HidDevice);

impl std::fmt::Display for Device {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = self.0.get_device_info().unwrap();
        let product_string = info.product_string().unwrap();
        let manufacturer_string = info.manufacturer_string().unwrap();
        let serial_number_string = info.serial_number().unwrap();
        let path = info.path().to_str().unwrap();
        write!(
            f,
            "Device: {}\nManufacturer: {}\nSerial Number: {}\nPath: {}",
            product_string, manufacturer_string, serial_number_string, path
        )
    }
}

impl Device {
    pub fn new(
        vendor_id: u16,
        product_id: u16,
        usage_page: u16,
        usage: u16,
    ) -> Result<Self, hidapi::HidError> {
        let context = hidapi::HidApi::new().unwrap();

        let device = context
            .device_list()
            .filter(|&d| {
                d.product_id() == product_id
                    && d.vendor_id() == vendor_id
                    && d.usage_page() == usage_page
                    && d.usage() == usage
            })
            .next()
            .ok_or(hidapi::HidError::HidApiErrorEmpty)?;

        Ok(Device(device.open_device(&context)?))
    }

    pub fn send<C: Into<Box<dyn Command>>>(
        &self,
        report_id: u8,
        command: C,
    ) -> Result<usize, hidapi::HidError> {
        // Prepend Report ID to the command
        let data = [&[report_id], command.into().as_bytes()].concat();
        self.0.write(&data)
    }

    pub fn read(&self) -> Result<Vec<u8>, hidapi::HidError> {
        let mut buf = [0u8; MAX_REPORT_LENGTH];
        let bytes_read = self.0.read(&mut buf)?;

        // Remove Report ID from the response
        Ok(buf[1..bytes_read].to_vec())
    }
}
