use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use models::users::User;

mod models;
mod services;

fn main() {
    let u1 = User::new(
        1,
        String::from("tbala"),
        String::from("tbala@fandatech.net"),
        String::from("9940180875"),
        String::from("Balamurugan"),
        String::from("Thanikachalam"),
    );
    let u2 = User::default();

    u1.show();
    u2.show();
}
