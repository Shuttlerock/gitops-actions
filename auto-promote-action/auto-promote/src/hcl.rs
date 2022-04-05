use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

pub fn update_file<T: AsRef<str>>(
    file_name: &Path,
    block: &str,
    labels: &[T],
    attributes: &HashMap<String, String>,
    target_attribute: &str,
    target_value: &str,
) -> Result<()> {
    let mut command = Command::new("hcl-tweak");

    command.arg("-filename").arg(file_name.as_os_str());
    command.arg("-block").arg(block);
    command.arg("-target-attribute").arg(target_attribute);
    command.arg("-target-value").arg(target_value);

    for label in labels {
        command.arg("-label");
        command.arg(label.as_ref());
    }

    for (name, value) in attributes {
        command.arg("-attribute");
        command.arg(format!("{}={}", name, value));
    }

    println!("Updating HCL file: {}", file_name.display());

    let output = command
        .output()
        .with_context(|| "hcl-tweak failed to run")?;

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if !output.status.success() {
        return Err(anyhow!("hcl-tweak failed to run: {}", output.status));
    }

    Ok(())
}
