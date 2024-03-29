xflags::xflags! {
    src "./src/flags.rs"
    cmd app
    {
        repeated -v, --verbose
        cmd new-day {
            optional -y, --year year: String
            optional -d, --day day: String
            optional -f, --force
        }
        cmd second {
            optional -y, --year year: String
            optional -d, --day day: String
        }
    }
}
// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct App {
    pub verbose: u32,
    pub subcommand: AppCmd,
}

#[derive(Debug)]
pub enum AppCmd {
    NewDay(NewDay),
    Second(Second),
}

#[derive(Debug)]
pub struct NewDay {
    pub year: Option<String>,
    pub day: Option<String>,
    pub force: bool,
}

#[derive(Debug)]
pub struct Second {
    pub year: Option<String>,
    pub day: Option<String>,
}

impl App {
    #[allow(dead_code)]
    pub fn from_env_or_exit() -> Self {
        Self::from_env_or_exit_()
    }

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end
