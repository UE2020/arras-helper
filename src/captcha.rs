use serde_json::Value;
use thirtyfour_sync::prelude::*;

pub fn make_key() -> String {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().expect("Failed to set headless");
    let driver =
        WebDriver::new("http://localhost:4444", &caps).expect("Failed to create webdriver");

    // Navigate to https://arras.io/.
    driver
        .get("https://arras.io/")
        .expect("Failed to navigate to arras");

    let ret = driver.execute_script(r#"
       async function mkkey() {
           return new Promise((resolve, reject) => {
               grecaptcha.execute("6LdwidEZAAAAAKGQ9ngDYVnClNn_aTAJcvg6cZUc", { action: "spawn" }).then(c => resolve(c));
           });
       }
       let ret = await mkkey();
       return ret;
       "#
   ).expect("Failed to execute script");

    let ret = ret.value();

    let ret = match ret {
        Value::String(s) => s.to_string(),
        _ => panic!("Expected captcha key as string"),
    };

    ret
}

pub fn make_keys() -> Vec<String> {
    let mut caps = DesiredCapabilities::chrome();
    caps.set_headless().expect("Failed to set headless");
    let driver =
        WebDriver::new("http://localhost:4444", &caps).expect("Failed to create webdriver");

    // Navigate to https://moomoo.io.
    driver
        .get("https://moomoo.io")
        .expect("Failed to navigate to moomoo");

    let ret = driver.execute_script(r#"
       async function mkkey() {
           return new Promise((resolve, reject) => {
               grecaptcha.execute("6LdwidEZAAAAAKGQ9ngDYVnClNn_aTAJcvg6cZUc", { action: "spawn" }).then(c => resolve(c));
           });
       }
       let keys = [];
       for (let i = 0; i < 100; i++) {
           keys.push(await mkkey());
       }
       return keys;
       "#
   ).expect("Failed to execute script");

    let ret = ret.value().as_array().unwrap();

    let mut rret = Vec::new();
    for value in ret.iter() {
        rret.push(value.clone().as_str().unwrap().to_owned());
    }

    rret
}