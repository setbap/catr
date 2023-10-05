mod catr;
use catr::get_args;
use catr::run;

pub fn cat() {
    if let Err(e) = get_args().and_then(run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
