fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/student.proto")?;
    tonic_build::compile_protos("proto/employee.proto")?;
    tonic_build::compile_protos("proto/executive.proto")?;
    Ok(())
}