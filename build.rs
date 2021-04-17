fn main() -> Result<(), Box<dyn std::error::Error>> {
    built::write_built_file()?;

    tonic_build::compile_protos("proto/epp.proto")?;
    Ok(())
}
