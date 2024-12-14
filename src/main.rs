use clap::Parser;

use std::path::Path;
use std::fs;

use colorized::*;

use inquire::{Text, Select};

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

fn run_prompt() {
    let project_name_ans = Text::new("mod name: ")
        .with_help_message("The name of the mod you want to create")
        .prompt();

    match project_name_ans {
        Ok(project_name_ans) => colorize_print(format!("You entered: {project_name_ans} "), Colors::BrightGreenFg),
        Err(_) => colorize_println("Error: Invalid mod name!", Colors::BrightRedFg),
    }

    let mod_options = vec!["Mod", "Submod"];

    let options_ans = Select::new("mod type: ", mod_options)
        .with_help_message("Select the type of mod you want to create: ")
        .prompt();

    match options_ans {
        Ok(mod_options) => colorize_print(format!("You selected: {mod_options} "), Colors::BrightGreenFg),
        Err(_) => colorize_println("Error: Invalid mod type!", Colors::BrightRedFg),
    }
        
}

fn main() {
    run_prompt();
    let cli = Cli::parse();
    
    let project_path = Path::new(&cli.project_name);

    if project_path.file_name().unwrap().to_str().unwrap().contains(' ') {
        colorize_println("Error: Invalid folder name. Folder name cannot contain spaces.", Colors::BrightRedFg);
        return;
    }

    generate_directories(project_path);
    generate_readme(project_path, &cli.project_name);

    colorize_println(format!("Mod project {} created successfully!", cli.project_name), Colors::BrightGreenFg);
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
        fs::create_dir_all(&full_path).expect(" ❌ Failed to create directories");
        println!("✅ Created directory: {:?}", full_path);
    }
}

fn generate_readme(project_path: &Path, project_name: &str) {
    // Generates a readme file with some basic info
    let readme_path = project_path.join("README.md");
    let readme_content = format!("# {}\n\nThis is the structure of the {} mod project. \n", project_name, project_name);
    fs::write(&readme_path, readme_content).expect("❌ Failed to create readme file");
    println!("✅ Created readme file: {:?}", readme_path);
}