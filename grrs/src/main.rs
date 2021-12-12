use structopt::StructOpt;
use anyhow::{Context, Result};

#[derive(StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::from_args();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout());
    Ok(())
}


fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write + std::fmt::Debug) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(&mut writer, "{}", line)
                .with_context(|| format!("could not read file"));
        }
    }
    println!("{:?}", writer);
}


