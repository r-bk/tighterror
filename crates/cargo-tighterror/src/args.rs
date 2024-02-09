use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum CargoCli {
    Tighterror(Args),
}

#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// The spec file path [default=tighterror.yaml]
    #[arg(short, long, value_name = "FILE")]
    pub spec: Option<String>,

    /// The destination file path
    #[arg(short, long, value_name = "DEST")]
    pub dst: Option<String>,

    /// Include a unit-test in the generated code
    #[arg(short, long)]
    pub test: bool,
}

impl Args {
    pub fn parse_args() -> Self {
        let CargoCli::Tighterror(args) = CargoCli::parse();
        args
    }

    #[inline]
    fn bool_to_opt(v: bool) -> Option<bool> {
        if v {
            Some(v)
        } else {
            None
        }
    }

    pub fn test(&self) -> Option<bool> {
        Self::bool_to_opt(self.test)
    }
}
