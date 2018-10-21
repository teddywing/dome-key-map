use xdg;

error_chain! {
    foreign_links {
        Xdg(xdg::BaseDirectoriesError);
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
