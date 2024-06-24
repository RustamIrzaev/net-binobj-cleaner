# .NET Project Cleaner (net-binobj-cleaner)

## Overview

This is a command-line utility written in Rust for developers working on .NET projects. It helps to clean up unnecessary `bin` and `obj` directories recursively within a specified .NET project directory, providing detailed information before performing any deletions.

## Features

- Recursively scans for `.csproj` files within a specified directory.
- Collects and displays information about the `bin` and `obj` directories for each project, including the number of files and total disk space used.
- Lists all files to be deleted, sorted alphabetically.
- Prompts for user confirmation before deleting the bin and obj directories.
- Supports configurable maximum depth for directory scanning.

## Prerequisites

**Rust**. Ensure that Rust is installed on your system. You can install Rust from [rust-lang.org](https://rust-lang.org).

## Building the Project

Clone the repository and build the project using Cargo:

```bash
git clone https://github.com/RustamIrzaev/net-binobj-cleaner
cd net-binobj-cleaner
cargo build --release
```

## Running the Application

To run the application, specify the directory to scan:

```bash
cargo run -- -f /path/to/your/csharp/project
```

### Parameters

- `-f` or `--folder` (required): Path to the .NET project directory.
- `--max-depth` (optional): Maximum depth for directory scanning. Default is 50.

## Example output

```markdown
Project: MyProject
/bin (3 files, 17 Kb):
- MyProject.dll
- MyProject.deps.json
- MyProject.pdb
- MyProject.runtimeconfig.json
/obj (24 files, 18 Kb):
- MyProject.AssemblyInfo.cs
- MyProject.AssemblyInfoInputs.cache
- MyProject.dll
- MyProject.genruntimeconfig.cache
- MyProject.pdb

Do you want to delete these directories? (yes/no)
...
```

## Important Notes

Use with care! This utility deletes files and directories permanently. Make sure to back up your data before running the application.

## License

This project is licensed under the Apache License 2.0. See the [LICENSE](LICENSE.md) file for details.