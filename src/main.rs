use std::env;
use toml_edit::DocumentMut;

#[derive(Debug, thiserror::Error)]
enum ProgError {
    #[error("missing argument: please provide a TOML file path")]
    MissingArg,
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Toml(#[from] toml_edit::TomlError),
}

fn run() -> Result<(), ProgError> {
    let mut args = env::args();
    args.next();

    let toml_file = args.next().ok_or(ProgError::MissingArg)?;
    let toml_content = std::fs::read_to_string(&toml_file)?;

    let mut doc: DocumentMut = toml_content.parse()?;

    for key in ["dependencies", "dev-dependencies", "build-dependencies"] {
        if let Some(table) = doc.get_mut(key).and_then(|item| item.as_table_mut()) {
            table.sort_values();
        }
    }

    if let Some(ws) = doc
        .get_mut("workspace")
        .and_then(|item| item.as_table_mut())
    {
        if let Some(deps) = ws
            .get_mut("dependencies")
            .and_then(|item| item.as_table_mut())
        {
            deps.sort_values();
        }
    }

    std::fs::write(&toml_file, doc.to_string())?;

    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
