fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=proto/");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .build_transport(true)
        .compile(
            &["proto/relay/v1/client.proto", "proto/relay/v1/daemon.proto"],
            &["proto/"],
        )?;

    Ok(())
}
