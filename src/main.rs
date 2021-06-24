#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}




#[get("/code")]
fn get_code() -> String {
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        "supersecret",
    );
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH).unwrap()
        .as_secs();
    //let url = totp.get_url("user@example.com", "my-org.com");
    // println!("{}", url);
    let token = totp.generate(time);
    println!("{}", token);
    return token;
}

fn main() {
    rocket::ignite().mount("/", routes![index,get_code]).launch();
}
