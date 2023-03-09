fn main() {
    match companion::bootstrap() {
        Ok(lock) => {
            println!("cargo:rerun-if-changed={:?}", lock);
        }
        Err(_) => println!("already launched"),
    }
}
