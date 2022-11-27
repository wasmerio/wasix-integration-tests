
fn main() {
    use std::path::PathBuf;

    let root = PathBuf::from("/tmp/wasix-tests");
    std::fs::remove_dir_all(&root).ok();

    std::fs::create_dir(&root).expect("colud not create wasix-tests dir");
    std::fs::metadata(&root).expect("could not get metadata");
}
