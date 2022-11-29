use chrono::Utc;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> String {
    let dt = Utc::now();
    let timestamp = dt.timestamp().to_string();
    timestamp
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
