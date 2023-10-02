#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Jai Shree Ram!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index]);

    Ok(rocket.into())
}
