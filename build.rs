fn main() -> Result<(), Box<dyn std::error::Error>> {
    built::write_built_file()?;

    tonic_build::configure()
        .build_client(false)
        .build_server(true)
        .file_descriptor_set_path(
            std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap())
                .join("file_descriptor_set.bin")
        )
        .compile(&["proto/epp.proto"], &["proto/", "~/.local/include"])?;
    Ok(())
}
