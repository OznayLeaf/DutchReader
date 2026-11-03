//! Desktop preview entry point. Android uses `android_main` in the library.

fn main() {
    if let Err(error) = dutchreader::app::run(None) {
        eprintln!("DutchReader failed to start: {error}");
        std::process::exit(1);
    }
}
