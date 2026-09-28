fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::configure().compile_protos(
        &[
            "proto/auth/v1/auth.proto",
            "proto/auth/v1/session.proto",
            "proto/events/auth/v1/auth_events.proto",
            "proto/common/v1/common.proto",
        ],
        &["proto"],
    )?;

    println!("cargo:rerun-if-changed=proto");

    Ok(())
}
