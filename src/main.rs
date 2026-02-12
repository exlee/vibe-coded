use vibe_coded::run_rules;

fn main() {
    let url = std::env::args().collect::<Vec<_>>()[1].clone();
    dbg!(&url);
    let _ = run_rules(&url);
}
