use std::error::Error;
use std::{env, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);

    tonic_build::configure()
        .file_descriptor_set_path(out_dir.join("atk_hub.bin"))
        .compile_protos(&["proto/atk-hub.proto"], &["proto"])?;

    tonic_build::compile_protos("proto/dpi_led.proto")?;
    tonic_build::compile_protos("proto/mouse_info.proto")?;
    tonic_build::compile_protos("proto/performance.proto")?;
    tonic_build::compile_protos("proto/silent_height.proto")?;
    tonic_build::compile_protos("proto/profiles.proto")?;
    tonic_build::compile_protos("proto/atk-hub.proto")?;

    Ok(())
}
