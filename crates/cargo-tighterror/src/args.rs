use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Args {
    /// The spec file path [default=tighterror.yaml]
    #[arg(short, long, value_name = "FILE")]
    pub spec: Option<String>,

    /// The destination file path
    #[arg(short, long, value_name = "OUT")]
    pub dst: Option<String>,

    /// Include a unit-test in the generated code
    #[arg(short, long)]
    pub test: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn test(&self) -> Option<bool> {
        if self.test {
            Some(true)
        } else {
            None
        }
    }
}
