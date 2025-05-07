use std::path::Path;
use std::fs;

use colorized::*;

use inquire::{ Text, validator::Validation};


fn run_prompt() {

    colorize_println("SW EAW FoC Mod Generator", Colors::BrightCyanFg);
    
    // Checks and validates the mod name text input for spaces
    let name_validator = |input: &str| match input.chars().find(|c| c.is_whitespace()) {
        Some(_) => Ok(Validation::Invalid("Mod name cannot contain spaces".into(),)),
        None => Ok(Validation::Valid)
    };


    let project_name_ans = Text::new("mod name: ")
        .with_validator(name_validator)
        .with_help_message("The name of the mod you want to create")
        .prompt();

    match project_name_ans {
        Ok(project_name) => {
            let project_path = Path::new(&project_name);
            generate_project(project_path);
        },
        Err(err) => eprintln!("❌ {}", err)
    }
        
}

fn main() {
    run_prompt();
}


fn generate_project(project_path: &Path) {
    // Generates necessary mod project directories
    let directories = [
        "Data/Art",
        "Data/Art/Maps",
        "Data/Art/Models",
        "Data/Art/Textures",
        "Data/Audio",
        "Data/Xml",
        "Data/Scripts",
        "Data/Scripts/AI",
        "Data/Scripts/Evaluators",
        "Data/Scripts/FreeStore",
        "Data/Scripts/GameObject",
        "Data/Scripts/Library",
        "Data/Scripts/Miscellaneous",
        "Data/Scripts/Story",
        "Data/Text",
    ];

    for dir in &directories {
        let full_path = project_path.join(dir);
        fs::create_dir_all(&full_path).expect(" ❌ Failed to create directories");
        println!("✅ Created directory: {:?}", full_path);
    }

    // Generates a readme file with some basic info
    let readme_path = project_path.join("README.md");
    let readme_content = format!("# {}\n\nThis is the structure of the {} mod project. \n", project_path.file_name().unwrap().to_str().unwrap(), project_path.file_name().unwrap().to_str().unwrap());
    fs::write(&readme_path, readme_content).expect("❌ Failed to create readme file");
    println!("✅ Created readme file: {:?}", readme_path);
}