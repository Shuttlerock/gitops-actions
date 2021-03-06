use anyhow::{Context, Result};
use clap::Parser;
use config::Pattern;
use regex::Regex;
use std::collections::HashMap;
use std::path::Path;
use uuid::Uuid;

mod cli;
mod config;
mod git;
mod glob;
mod hcl;
mod pr;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short = 'u', long)]
    git_user: String,

    #[clap(short = 'e', long)]
    git_email: String,

    #[clap(short = 'p', long)]
    git_password: Option<String>,

    #[clap(short, long)]
    config: std::path::PathBuf,

    #[clap(short = 't', long)]
    trigger: Option<String>,

    /// Key value pairs of the form 'key1=value1 key2=value2'.
    #[clap(short, long, parse(try_from_str = cli::parse_key_val), multiple_occurrences(false), value_delimiter(' '))]
    variables: Vec<(String, String)>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    let cfg = config::from_path(&args.config)?;

    let repository_str = "/tmp/repository";
    let repository_path = Path::new(&repository_str);

    for target in cfg.targets {
        // Skip disabled targets.
        if !target.enabled {
            continue
        }

        // If trigger and filter are defined, then skip this target if the trigger doesn't match the filter.
        if let (Some(filter), Some(trigger)) = (&target.filter, &args.trigger) {
            let filter = Regex::new(&filter)?;

            if !filter.is_match(&trigger) {
                continue;
            }
        }

        // Clone and checkout target repo / branch.
        let ctx = git::clone(
            &target.repository,
            &target.branch,
            repository_path,
            &args.git_user,
            &args.git_email,
            args.git_password.as_deref(),
        )?;

        // Apply all rules.
        for rule in target.rules.iter() {
            let absolute_paths = glob::files(&format!("{}/{}", repository_str, rule.file_pattern))?;

            // Strip repository path to get just relative paths in the form expected by git.
            let relative_paths = absolute_paths
                .iter()
                .map(|path| path.strip_prefix(repository_path))
                .collect::<Result<Vec<_>, _>>()?;

            // Find the first correct value for variable in the inputs.
            let value = args
                .variables
                .iter()
                .find(|(key, _)| *key == rule.variable)
                .map(|(_, value)| value)
                .with_context(|| "variable not found in inputs")?;

            match &rule.pattern {
                Pattern::Hcl {
                    block,
                    labels,
                    attributes,
                    target_attribute,
                } => {
                    for path in &absolute_paths {
                        hcl::update_file(
                            &path,
                            &block,
                            labels.as_ref().unwrap_or(&Vec::default()),
                            attributes.as_ref().unwrap_or(&HashMap::default()),
                            &target_attribute,
                            &value,
                        )?;
                    }
                }
            }

            // Add and commit updated file(s).
            ctx.add_and_commit(
                &relative_paths,
                &format!("Bump {} to {}.", rule.variable, value),
            )?;
        }

        // Generate a unique branch name.
        let origin_branch = format!("auto-promote-{}", Uuid::new_v4().to_string());

        // Push changes to new branch on origin.
        ctx.push_head(&origin_branch)?;

        // Create and merge PR.
        pr::merge_pull_request(
            &target.repository,
            &origin_branch,
            &target.branch,
            "Auto Promotion",
            "Auto Promotion",
            args.git_password.as_deref(),
        )
        .await?;
    }

    Ok(())
}
