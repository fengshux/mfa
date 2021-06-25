#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use postgres::{Client, Error, NoTls};
use std::time::SystemTime;
use totp_rs::{Algorithm, TOTP};
use chrono::prelude::*;
use uuid::Uuid;
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/signup")]
fn signup() -> (){
    
}

fn get_database_conn() -> Result<Client, Error> {
    return Client::connect(
        "postgresql://postgres:mypass@127.0.0.1/mfa",
        NoTls,
    );
}

#[derive(Debug)]
struct User {
    id: Uuid,
    phone: String,
    token: String,
    create_time: DateTime<Local>,
    modify_time: DateTime<Local>,
}

#[post("/login")]
fn login() -> (){
    let mut conn = get_database_conn().expect("get database conn failed.");
    let phone = "15810173257".to_string();
    let row = conn.query_one("SELECT id, phone, token, create_time, modify_time FROM public.user WHERE phone = $1", &[&phone]).expect("query from database fialed");

    let user = User{
        id: row.get("id"),
        phone: row.get("phone"),
        token: row.get("token"),
        create_time:  row.get("create_time"),
        modify_time:  row.get("modify_time"),
    };
    
   println!("{:?}", user);
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
    rocket::ignite().mount("/", routes![index,get_code, login]).launch();
}
