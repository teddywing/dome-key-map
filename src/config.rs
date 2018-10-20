use std::fs;

use getopts::Options;
use toml;
use xdg;

use errors::*;

type Milliseconds = u16;

#[repr(C)]
#[derive(Default)]
struct Args {
    reload: bool,
    daemon: bool,
}

#[repr(C)]
#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip)]
    args: Args,
    timeout: Milliseconds,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            args: Args::default(),
            timeout: 500,
        }
    }
}

fn print_usage(opts: Options) {
    let brief = "Usage: dome-key [options]";
    print!("{}", opts.usage(&brief));
}

pub fn parse_args<'a>(args: &[String], config: &'a mut Config) -> &'a mut Config {
    let mut opts = Options::new();

    opts.optflag("d", "daemon", "run the daemon in the current shell");
    opts.optflag("r", "reload-mappings", "reload the mappings file");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => panic!(e),
    };

    if matches.opt_present("h") {
        print_usage(opts);
        return config;
    }

    if matches.opt_present("r") {
        config.args.reload = true;
    } else if matches.opt_present("d") {
        config.args.daemon = true;
    } else {
        print_usage(opts);
    }

    config
}

pub fn get_config() -> Result<Config> {
    let xdg_dirs = xdg::BaseDirectories::with_prefix("dome-key")?;
    let config = match xdg_dirs.find_config_file("config.toml") {
        Some(config_file) => {
            let config_str = fs::read_to_string(config_file)
                .chain_err(|| "failed to read config file")?;

            toml::from_str(&config_str)
                .chain_err(|| "failed to parse config file")?
        },
        None => Config::default(),
    };

    Ok(config)
}
