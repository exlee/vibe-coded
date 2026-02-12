use std::process::exit;

use vibe_coded::{clean_repo_dir, run_rules};

fn main() {
    let mut args = pico_args::Arguments::from_env();
    let Ok(url) = args.free_from_str::<String>() else {
        println!("Missing URL argument");
        exit(1)
    };
    let clean_before = args.contains(["-c", "--clean"]);
    if clean_before {
        let _ = clean_repo_dir(&url);
    };
    let _ = run_rules(&url);
}
