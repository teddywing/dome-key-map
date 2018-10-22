use chrono;
use magic_crypt;
use xdg;

error_chain! {
    foreign_links {
        Xdg(xdg::BaseDirectoriesError);
        Io(::std::io::Error);
    }
}

quick_error! {
    #[derive(Debug, PartialEq)]
    pub enum DurationError {
        NegativeDuration(duration: i32) {
            description("negative duration")
            display("negative duration: '{}'", duration)
        }
    }
}

quick_error! {
    #[derive(Debug)]
    pub enum DateCryptError {
        DateParse(err: chrono::format::ParseError) {
            from()
            cause(err)
            display("unable to parse timestamp")
        }
        Decrypt(err: magic_crypt::Error) {
            from()
            display("unable to read trial key")
        }
    }
}
