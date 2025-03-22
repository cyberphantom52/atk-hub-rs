use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("proto/dpi_led.proto")?;
    tonic_build::compile_protos("proto/atk-hub.proto")?;

    Ok(())
}
