mod helpers;

use std::process::Command;

use crate::helpers::ShootStructureGenerator;

#[tokio::test]
async fn shoot() {
    let generator = ShootStructureGenerator::default();
    generator.generate_destination_folder().await;
    generator.generate_source_folder().await;

    let output = Command::new(env!("CARGO_BIN_EXE_snapshoot"))
        .arg("shoot")
        .arg("--source")
        .arg(&generator.source)
        .arg("--destination")
        .arg(&generator.destination)
        .output();

    generator.clean_generated_folders().await;

    match output {
        Ok(result) => {
            if !result.status.success() {
                let stderr = String::from_utf8_lossy(&result.stderr);
                println!("Command did not finish successfully: {}", stderr);
            }

            assert!(result.status.success());
        }
        Err(error) => {
            assert!(false, "Error during command execution: {}", error.to_string());
        }
    }
}
