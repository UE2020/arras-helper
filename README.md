# arras-socket
Crate for interacting with arras.io

Not complete

## fasttalk

`src/fasttalk.rs` contains a Rust implementation of arras.io's fasttalk. I use an enum called `Block` to handle the dynamic nature of fasttalk. It's possible to cast a block into `f64` and `std::String`. Please note that the cast will panic if the type is incorrect, though it is possible to check a cast beforehand using `is_type`.

## captcha

`src/captcha.rs` is used to obtain spawn captchas. It is expected that you have a webdriver server running on port 4444.
