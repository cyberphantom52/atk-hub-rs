use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    tonic_build::compile_protos("proto/dpi_led.proto")?;
    tonic_build::compile_protos("proto/mouse_info.proto")?;
    tonic_build::compile_protos("proto/performance.proto")?;
    tonic_build::compile_protos("proto/silent_height.proto")?;
    tonic_build::compile_protos("proto/profiles.proto")?;
    tonic_build::compile_protos("proto/atk-hub.proto")?;

    Ok(())
}
