mod cli;
mod services;

use std::path::PathBuf;
use clap::Parser;
use walkdir::WalkDir;
use crate::cli::Cli;
use crate::services::delete_folder::delete_folder;
use crate::services::get_folder_info::get_folder_info;

fn main() {
    let cli = Cli::parse();
    let mut projects_info = Vec::new();
    let walker = WalkDir::new(&cli.folder)
        .max_depth(50)
        .into_iter().filter_map(|e| e.ok())
        .filter(|e| e.file_name().to_string_lossy().ends_with(".csproj"));

    for entry in walker {
        let project_path = entry.path();
        let project_name = project_path.file_stem().and_then(|s| s.to_str()).unwrap_or("Unknown");

        let mut bin_path = PathBuf::from(&entry.path());
        bin_path.pop();
        bin_path.push("bin");

        let mut obj_path = PathBuf::from(&entry.path());
        obj_path.pop();
        obj_path.push("obj");

        let bin_info = get_folder_info(&bin_path);
        let obj_info = get_folder_info(&obj_path);

        if bin_info.0 > 0 || obj_info.0 > 0 {
            projects_info.push((project_name.to_string(), bin_info, obj_info));
        }
    }

    if projects_info.is_empty() {
        println!("No bin or obj directories found.");
        return;
    }

    for (project_name, bin_info, obj_info) in &projects_info {
        println!("Project: {}", project_name);
        println!("Bin Folder:");
        println!("  Number of files: {}", bin_info.0);
        println!("  Total disk space: {:.2} KB", bin_info.1 as f64 / 1024.0);
        println!("Obj Folder:");
        println!("  Number of files: {}", obj_info.0);
        println!("  Total disk space: {:.2} KB", obj_info.1 as f64 / 1024.0);
        println!();
    }

    if !cli.enable_delete {
        println!("Deletion is disabled");
        return;
    }

    println!("Do you want to delete these directories? (yes/no)");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();

    if input == "yes" || input == "y" {
        for (_, bin_info, obj_info) in projects_info {
            delete_folder(&bin_info.2);
            delete_folder(&obj_info.2);
        }
        println!("Deleted bin and obj directories.");
    } else {
        println!("Deletion cancelled.");
    }
}