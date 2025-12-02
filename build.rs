fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!(">>> BUILD SCRIPT RAN <<<");
    tonic_prost_build::configure()
        .file_descriptor_set_path("wubrag_descriptor.bin")
        .compile_protos(&["proto/wubrag.proto"], &["proto"])?;
    Ok(())
}
