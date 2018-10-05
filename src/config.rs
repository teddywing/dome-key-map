use getopts::Options;

#[no_mangle]
#[derive(Default)]
struct Args {
    reload: bool,
    daemon: bool,
}

#[no_mangle]
#[derive(Default)]
pub struct Config {
    args: Args,
}

fn print_usage(opts: Options) {
    let brief = "Usage: dome-key [options]";
    print!("{}", opts.usage(&brief));
}

pub fn parse_args(args: &[String]) -> Config {
    let mut opts = Options::new();

    opts.optflag("d", "daemon", "run the daemon in the current shell");
    opts.optflag("r", "reload-mappings", "reload the mappings file");
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(e) => panic!(e),
    };

    let mut config = Config::default();

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
