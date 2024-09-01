use std::path::PathBuf;
use std::{fs, thread};
use std::process::Command;

//probably will never be updated
fn main() {
    let project_path: &str = "C:/.Projects/java/Azaria-Dev";
    let mod_name: &str = "AzariaDesktop.zip";
    let destination_folder: PathBuf = dirs::data_dir().unwrap().join("Mindustry").join("mods");
    let mindusry_path = r#"C:/Program Files/Mindustry/Mindustry.exe"#;

    let build_thread = thread::spawn(move || {
        match Command::new("cmd")
            .args(&["/C", "gradlew", "jar"])
            .current_dir(project_path)
            .status()
        {
            Ok(_) => (),
            Err(err) => {
                eprintln!("{err}");
                return;
            }
        }
    });

    let old_version_path = destination_folder.join(mod_name);
    if old_version_path.exists() {
        fs::remove_file(old_version_path).expect("File not found");
        println!("Old version deleted successfully");
    }

    build_thread.join().unwrap();
    let jar_path = PathBuf::from("build/libs/AzariaDesktop.jar");
    fs::copy(
        PathBuf::from(project_path).join(jar_path),
        destination_folder.join("AzariaDesktop.jar"),
    )
    .expect("Cannot copy");

    let _ = Command::new(mindusry_path).status().expect("Cannot run mindustry");
}
