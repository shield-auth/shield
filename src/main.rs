#[macro_use]
extern crate rocket;

mod organization;

use organization::org_routes;

#[get("/")]
fn index() -> &'static str {
    "Hi! It's your Shield"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/organizations", org_routes())
}

// #[shuttle_runtime::main]
// async fn rocket() -> shuttle_rocket::ShuttleRocket {
//     let rocket = rocket::build()
//         .mount("/", routes![index])
//         .mount("/organizations", org_routes());

//     Ok(rocket.into())
// }
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/organizations", org_routes())
}

// #[shuttle_runtime::main]
// async fn rocket() -> shuttle_rocket::ShuttleRocket {
//     let rocket = rocket::build()
//         .mount("/", routes![index])
//         .mount("/organizations", org_routes());

//     Ok(rocket.into())
// }
