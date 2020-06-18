use std::error::Error;

use shaderc::ShaderKind;

fn main() -> Result<(), Box<dyn Error>> {
    // Tell the build script to only run again if we change our source shaders
    println!("cargo:rerun-if-changed=resources/shaders/src");

    // Create destination path if necessary
    std::fs::create_dir_all("resources/shaders/compiled")?;
    for entry in std::fs::read_dir("resources/shaders/src")? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            println!("{:?}", entry);
            let in_path = entry.path();

            // Support only vertex and fragment shaders currently
            let shader_type =
                in_path
                    .extension()
                    .and_then(|ext| match ext.to_string_lossy().as_ref() {
                        "vert" => Some(ShaderKind::Vertex),
                        "frag" => Some(ShaderKind::Fragment),
                        _ => None,
                    });

            if let Some(shader_type) = shader_type {
                let source = std::fs::read_to_string(&in_path)?;
                let mut compiler = shaderc::Compiler::new().unwrap();

                let compiled = compiler.compile_into_spirv(
                    &source,
                    shader_type,
                    entry.file_name().into_string().unwrap().as_str(),
                    "main",
                    None,
                )?;

                // Determine the output path based on the input name
                let out_path = format!(
                    "resources/shaders/compiled/{}.spv",
                    in_path.file_name().unwrap().to_string_lossy()
                );

                std::fs::write(&out_path, &compiled.as_binary_u8())?;
            }
        }
    }

    Ok(())
}
