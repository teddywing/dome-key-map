// Copyright (c) 2018 Teddy Wing
//
// This file is part of DomeKey.
//
// *Purchasing policy notice:* All users of the software are expected to
// purchase a license from Teddy Wing unless they have a good reason not to
// pay. Users who can't purchase a license may apply to receive one for free
// at inquiry@domekey.teddywing.com. Users are free to:
//
// * download, build, and modify the app;
// * share the modified source code;
// * share the purchased or custom-built binaries (with unmodified license
//   and contact info), provided that the purchasing policy is explained to
//   all potential users.
//
// This software is available under a modified version of the Open Community
// Indie Software License:
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose is hereby granted, subject to the following conditions:
//
// * all copies retain the above copyright notice, the above purchasing
//   policy notice and this permission notice unmodified;
//
// * all copies retain the name of the software (DomeKey), the name of the
//   author (Teddy Wing), and contact information (including, but not limited
//   to, inquiry@domekey.teddywing.com, and domekey.teddywing.com URLs)
//   unmodified;
//
// * no fee is charged for distribution of the software;
//
// * the best effort is made to explain the purchasing policy to all users of
//   the software.
//
// THE SOFTWARE IS PROVIDED "AS IS", AND THE AUTHOR AND COPYRIGHT HOLDERS
// DISCLAIM ALL WARRANTIES, EXPRESS OR IMPLIED, WITH REGARD TO THIS SOFTWARE,
// INCLUDING BUT NOT LIMITED TO WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE. IN NO EVENT SHALL THE AUTHOR OR COPYRIGHT HOLDERS BE
// LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY
// DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA, OR PROFITS, WHETHER
// IN AN ACTION OF CONTRACT, NEGLIGENCE, OR OTHER TORTIOUS ACTION, ARISING
// OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.

use std::ffi::CString;
use std::fs;
use std::ptr;

use exitcode;
use libc::c_char;
use getopts::Options;
use toml;
use xdg;

use errors::*;

type Milliseconds = u16;

#[repr(C)]
pub struct Args {
    pub reload: bool,
    pub daemon: bool,
    pub audio: bool,
    pub version: bool,
    pub license: *mut c_char,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            reload: false,
            daemon: false,
            audio: false,
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

    ::std::process::exit(exitcode::OK);
}

pub fn parse_args<'a>(args: &[String], config: &'a mut Config) -> &'a mut Config {
    let mut opts = Options::new();

    opts.optflag("d", "daemon", "run the daemon in the current shell");
    opts.optflag("r", "reload-mappings", "reload the mappings file");
    opts.optflag("", "audio", "play interface audio");
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

    if matches.opt_present("audio") {
        config.args.audio = true;
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
