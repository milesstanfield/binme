use std::env;

pub fn current_dir() -> String {
    match env::current_dir() {
        Ok(val) => val.into_os_string().into_string().expect("not a string"),
        Err(e) => panic!("no current_dir i guess {}", e),
    }
}
