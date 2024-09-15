use git2::{Repository, Status};

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let (repo_path, output_path) = parse_args(&args)?;

    let repo = Repository::open(&repo_path)?;
    let mut output: Box<dyn Write> = if let Some(path) = output_path {
        Box::new(File::create(path)?)
    } else {
        Box::new(io::stdout())
    };

    traverse_repo(
        &repo,
        repo.workdir().ok_or("Invalid repository")?,
        &mut output,
    )?;
    Ok(())
}

fn parse_args(args: &[String]) -> Result<(String, Option<String>), Box<dyn Error>> {
    match args.len() {
        1 => Ok((".".to_string(), None)),
        2 => Ok((args[1].clone(), None)),
        3 => Ok((args[1].clone(), Some(args[2].clone()))),
        _ => Err("Usage: git-file-concatenator [repo_path] [output_file]".into()),
    }
}

fn traverse_repo(
    repo: &Repository,
    dir: &Path,
    output: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if !file_name_str.starts_with('.') {
                        traverse_repo(repo, &path, output)?;
                    }
                }
            }
        } else {
            process_file(repo, &path, output)?;
        }
    }
    Ok(())
}

fn process_file(
    repo: &Repository,
    path: &Path,
    output: &mut dyn Write,
) -> Result<(), Box<dyn Error>> {
    let repo_workdir = repo.workdir().ok_or("Invalid repository")?;
    let repo_path = path
        .strip_prefix(repo_workdir)
        .map_err(|e| format!("Failed to strip prefix: {}", e))?;

    let status = repo
        .status_file(repo_path)
        .map_err(|e| format!("Failed to get file status: {}", e))?;

    if status.is_empty() || status.contains(Status::WT_MODIFIED) || status.contains(Status::WT_NEW)
    {
        writeln!(output, "// Processing File {:?}", repo_path)?;
        let content = std::fs::read_to_string(path)?;
        writeln!(output, "// File: {:?}", repo_path)?;
        writeln!(output, "{}\n", content)?;
    }
    Ok(())
}
