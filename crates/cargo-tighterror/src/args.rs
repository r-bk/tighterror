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
    /// The specification file path
    #[arg(short, long, value_name = "PATH")]
    pub spec: Option<String>,

    /// The output path
    #[arg(short, long, value_name = "PATH")]
    pub output: Option<String>,

    /// Include a unit-test in the generated code
    #[arg(short, long)]
    pub test: bool,

    /// Do not overwrite the output file if data is unchanged
    #[arg(short, long)]
    pub update: bool,

    /// Write modules in separate files
    #[arg(short = 'S', long)]
    pub separate_files: bool,
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

    pub fn update(&self) -> Option<bool> {
        Self::bool_to_opt(self.update)
    }

    pub fn separate_files(&self) -> Option<bool> {
        Self::bool_to_opt(self.separate_files)
    }
}
