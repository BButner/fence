use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut protos_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    protos_path.pop();
    protos_path.push("protos");

    tonic_build::configure()
        .build_server(true)
        .build_client(false)
        .compile(&[protos_path.join("fence.proto").as_path()], &[protos_path])?;

    Ok(())
}
