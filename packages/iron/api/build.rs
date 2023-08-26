fn main() -> anyhow::Result<()> {
    tonic_build::compile_protos("proto/minecraft_service.proto")?;

    Ok(())
}
