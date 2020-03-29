#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket_contrib::json::{JsonValue};

mod dice;

#[derive(Serialize, Deserialize)]
struct Meta {
    name: String,
    version: String,
    commit: String
}

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("GIT_HEAD");

impl Meta {
    pub fn generate() -> MetaHolder {
        MetaHolder {
            json: json!(
                Meta {
                    name: PKG_NAME.to_string(),
                    version: PKG_VERSION.to_string(),
                    commit: GIT_COMMIT.to_string()
                }
            )
        }
    }
}


struct MetaHolder {
    json: JsonValue
}

#[get("/meta")]
fn meta(meta: State<MetaHolder>) -> JsonValue {
    json!(meta.json)
}

#[get("/roll")]
fn roll() -> JsonValue {
    json!(dice::roll(1, 6))
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": 404,
        "message": "Not Found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(Meta::generate())
        .mount("/", routes![meta, roll])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
