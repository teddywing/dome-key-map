use std::ffi::CString;
use std::fs;
use std::ptr;

use libc::c_char;
use getopts::Options;
use toml;
use xdg;

use errors::*;
use prefix_println;

type Milliseconds = u16;

#[repr(C)]
pub struct Args {
    pub reload: bool,
    pub daemon: bool,
    pub version: bool,
    pub license: *mut c_char,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            reload: false,
            daemon: false,
            version: false,
            license: ptr::null_mut(),
        }
    }
}

#[repr(C)]
#[derive(Deserialize)]
#[serde(default)]
pub struct Config {
    #[serde(skip)]
    pub args: Args,
    pub timeout: Milliseconds,
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
    opts.optopt(
        "",
        "license",
        "register the software using a license plist file",
        "FILE"
    );
    opts.optflag("v", "version", "print the program version");
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
    } else if let Some(license_path) = matches.opt_str("license") {
        match CString::new(license_path) {
            Ok(str) => config.args.license = str.into_raw(),
            Err(e) => dkeprintln!("{}", e),
        }
    } else if matches.opt_present("v") {
        config.args.version = true;
    } else {
        print_usage(opts);
    }

    config
}

pub fn get_config() -> Result<Config> {
    let config = match xdg::BaseDirectories::with_prefix("dome-key") {
        Ok(xdg_dirs) => {
            match xdg_dirs.find_config_file("config.toml") {
                Some(config_file) => {
                    let config_str = fs::read_to_string(config_file)
                        .chain_err(|| "failed to read config file")?;

                    toml::from_str(&config_str)
                        .chain_err(|| "failed to parse config file")?
                },
                None => Config::default(),
            }
        },
        Err(_) => Config::default(),
    };

    Ok(config)
}
