use xdg;

error_chain! {
    foreign_links {
        Xdg(xdg::BaseDirectoriesError);
    }
}
