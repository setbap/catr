mod headr;
use headr::get_args;
use headr::run;

pub fn head() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
