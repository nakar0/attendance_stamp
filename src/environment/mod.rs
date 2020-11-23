use std::env;
use std::process;

const CORPORATE_ID_KEY: &str = "CORPORATE_ID";
const EMAIL_KEY: &str = "EMAIL";
const PASSWORD_KEY: &str = "PASSWORD";

#[derive(Debug)]
pub struct Env {
    corporate_id: String,
    email: String,
    password: String,
}

trait Animal {
    fn new() -> Self;
    fn corporate_id(&self) -> &str;
    fn email(&self) -> &str;
    fn password(&self) -> &str;
}

impl Env {
    pub fn new() -> Env {
        Env {
            corporate_id: get_env(CORPORATE_ID_KEY),
            email: get_env(EMAIL_KEY),
            password: get_env(PASSWORD_KEY),
        }
    }

    pub fn corporate_id(&self) -> &str {
        &self.corporate_id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(err) => {
            println!("{}: {}", err, key);
            process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;

    static CORPORATE_ID: &str = "c";
    static EMAIL: &str = "e";
    static PASSWORD: &str = "p";

    fn setup() {
        env::set_var(CORPORATE_ID_KEY, CORPORATE_ID);
        env::set_var(EMAIL_KEY, EMAIL);
        env::set_var(PASSWORD_KEY, PASSWORD);
    }

    #[test]
    fn construct_env() {
        setup();
        let e = Env::new();
        assert_eq!(e.corporate_id(), CORPORATE_ID);
        assert_eq!(e.email(), EMAIL);
        assert_eq!(e.password(), PASSWORD);
    }
}
