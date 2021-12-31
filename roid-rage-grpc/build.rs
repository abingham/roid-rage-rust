fn main() -> Result<(), Box<dyn std::error::Error>> {
	tonic_build::compile_protos("proto/roid-rage/roid-rage.proto")?;
	Ok(())
    }
    
    