use clap::Parser;

use std::path::Path;
use std::fs;

/// CLI tool for scaffolding an empty Star Wars: Empire at War: Forces of Corruption mod project
#[derive(Parser)]
#[clap(name = "SW EAW FoC Mod Generator")]
#[clap(version = "1.0")]
#[clap(about = "CLI tool for scaffolding an empty Star Wars: Empire at War: Forces of Corruption mod project")]
struct Cli {
    /// The name of the mod
    #[clap(value_parser)]
    project_name: String,
}

fn main() {
    let cli = Cli::parse();
    
    let project_path = Path::new(&cli.project_name);

    if project_path.file_name().unwrap().to_str().unwrap().contains(' ') {
        eprintln!("Error: Invalid folder name. Folder name cannot contain spaces.");
        return;
    }

    generate_directories(project_path);
    generate_readme(project_path, &cli.project_name);

    println!("Mod project {} created successfully!", cli.project_name);
}

fn generate_directories(project_path: &Path) {
    // Generates necessary mod project directories
    let directories = [
        "Data/Art",
        "Data/Audio",
        "Data/Xml",
        "Data/Scripts",
        "Data/Text",
    ];

    for dir in &directories {
        let full_path = project_path.join(dir);
        fs::create_dir_all(&full_path).expect("Failed to create directories");
        println!("Created directory: {:?}", full_path);
    }
}

fn generate_readme(project_path: &Path, project_name: &str) {
    // Generates a readme file with some basic info
    let readme_path = project_path.join("README.md");
    let readme_content = format!("# {}\n\nThis is the structure of the {} mod project. \n", project_name, project_name);
    fs::write(&readme_path, readme_content).expect("Failed to create readme file");
    println!("Created readme file: {:?}", readme_path);
}