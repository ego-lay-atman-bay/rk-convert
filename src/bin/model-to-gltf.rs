use rk_convert::glb::rk_to_gltf;
use std::{env, io, path::Path};




fn main() -> io::Result<()> {
    let args = env::args_os().collect::<Vec<_>>();
    assert!(
        args.len() == 2 || args.len() == 3,
        "usage: {} <model.rk> [anim.csv|anim.anim]",
        args[0].to_string_lossy(),
    );

    let model_path = Path::new(&args[1]);
    let anim_path = args.get(2).map(|s| Path::new(s));
    let output_path = model_path.with_extension("glb");
    
    eprintln!("load object from {}", model_path.display());
    
    rk_to_gltf(model_path, anim_path, &output_path)?;
    
    println!("Output Filename: {}", output_path.display());

    Ok(())
}
