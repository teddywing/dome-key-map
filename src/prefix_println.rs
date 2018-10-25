#[macro_export]
macro_rules! dkeprintln {
    () => (eprint!("\n"));
    ($($arg:tt)*) => {
        eprintln!(
            "dome-key: error: {}",
            format!($($arg)*),
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn dkprintln_macro() {
        dkeprintln!();
        dkeprintln!("test");
        dkeprintln!("multiple arguments {}, {}", 5, 50 / 2);
    }
}
