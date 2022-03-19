use std::path::Path;
use std::error::Error;
use std::process::Command;
use std::io::{self, Write};

pub fn update_file<T: AsRef<str>>(file_name: &Path, block: &str, labels: &[T], attribute: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let mut command = Command::new("hcl-tweak");

    command.arg("-filename").arg(file_name.as_os_str());
    command.arg("-block").arg(block);
    command.arg("-attribute").arg(attribute);
    command.arg("-value").arg(value);

    if !labels.is_empty() {
        command.arg("-labels");
        for label in labels.iter() {
            command.arg(label.as_ref());
        }
    }

    let output = command.output()?;

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    if !output.status.success() {
        return Err(format!("hcl-tweak failed to run: {}", output.status).into());
    }

    Ok(())
}
