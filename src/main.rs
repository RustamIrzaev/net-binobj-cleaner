mod cli;
mod services;

use std::path::PathBuf;
use clap::Parser;
use colored::Colorize;
use walkdir::WalkDir;
use crate::cli::Cli;
use crate::services::delete_folder::delete_folder;
use crate::services::get_folder_info::get_folder_info;
use crate::services::list_files::list_files;

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
            let mut bin_files = list_files(&bin_path);
            let mut obj_files = list_files(&obj_path);

            bin_files.sort();
            obj_files.sort();

            projects_info.push((project_name.to_string(), bin_info, obj_info, bin_files, obj_files));
        }
    }

    if projects_info.is_empty() {
        println!("{}", "No bin or obj directories found.".bright_red());
        return;
    }

    for (project_name, bin_info, obj_info, bin_files, obj_files) in &projects_info {
        println!("Project: {}", project_name.bright_blue().bold());
        println!(" {} ({} files, {:.2} Kb):", "/bin".bold().yellow(), bin_info.0.to_string().bright_green(),
                 (bin_info.1 as f64 / 1024.0).to_string().bright_green());

        if !bin_files.is_empty() {
            for file in bin_files {
                println!("    - {}", file.to_string().white());
            }
        }

        println!(" {} ({} files, {:.2} Kb):", "/obj".bold().yellow(), obj_info.0.to_string().bright_green(),
                 (obj_info.1 as f64 / 1024.0).to_string().bright_green());

        if !obj_files.is_empty() {
            for file in obj_files {
                println!("    - {}", file.to_string().white());
            }
        }

        println!();
    }

    println!("Do you want to delete these directories? ({}/no)", "yes".bright_red());
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let input = input.trim().to_lowercase();

    if input == "yes" || input == "y" {
        for (_, bin_info, obj_info, _, _) in projects_info {
            delete_folder(&bin_info.2);
            delete_folder(&obj_info.2);
        }
        println!("Deleted bin and obj directories.");
    } else {
        println!("Deletion cancelled.");
    }
}