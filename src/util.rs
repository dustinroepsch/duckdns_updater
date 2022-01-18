use anyhow::Error;

pub fn warn_error(err: &Error) {
    println!("Warning!! Error encountered: {}", err);
}
