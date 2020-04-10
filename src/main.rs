#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use uuid::Uuid;
use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

use rocket::request::FromParam;
use rocket::http::RawStr;

use std::sync::Mutex;
use std::collections::HashMap;

use crate::state::Game;
use crate::api::{GameDescription, GameState, Player};

mod api;
mod dice;
mod state;

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

type Games = Mutex<HashMap<Uuid, Game>>;

#[get("/meta")]
fn meta(meta: State<MetaHolder>) -> JsonValue {
    json!(meta.json)
}

#[get("/roll")]
fn roll() -> JsonValue {
    json!(dice::roll(1, 6))
}

#[post("/game")]
fn new_game(games: State<Games>) -> Json<GameDescription> {
    let game = Game::create();
    let description = game.description.clone();
    games.lock().unwrap().insert(description.id, game);
    Json(description)
}

#[get("/game/<uuid>")]
fn get_game(games: State<Games>, uuid: UUID) -> Option<Json<GameDescription>> {
    let all = games.lock().unwrap();
    println!("{:?}", all);
    println!("{}", uuid.uuid.to_urn());

    all.get(&uuid.uuid).map(|game| Json(game.description.clone()))
}

fn with_game<F: FnOnce(&mut Game) -> ()>(games: State<Games>, id: UUID, action: F)
{
    let mut all = games.lock().unwrap();
    all.entry(id.uuid).and_modify(action);
}

#[put("/game/<uuid>/<name>")]
fn join_game(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    let mut player: Option<Json<Player>> = None;
    with_game(games, uuid, |game| {
        if game.description.state == GameState::WAITING {
            player = Some(Json(game.join_new_player(name)));
        }
        // TODO: else error
    });
    player
}

// TODO make this less dumb, PUT with body to player resource
#[put("/game/<uuid>/<name>/ready")]
fn ready_player(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    let mut player: Option<Player> = None;
    with_game(games, uuid, |game| {
        if game.description.state == GameState::WAITING {
            player = game.player_ready(name)
        }
    });
    player.map(|p| Json(p))
}

#[get("/game/<uuid>/<name>")]
fn get_player(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    games.lock().unwrap().get(&uuid.uuid).map(|game| {
        game.players.iter().find(|p| p.name == name).map(|p| Json(p.clone()))
    }).flatten()
}

struct UUID {
    uuid: Uuid
}

impl<'a> FromParam<'a> for UUID {
    type Error = uuid::Error;

    #[inline(always)]
    fn from_param(param: &'a RawStr) -> Result<UUID, Self::Error> {
        Uuid::parse_str(param).map(|u| UUID { uuid: u })
    }
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
        .manage(Mutex::new(HashMap::<Uuid, Game>::new()))
        .mount("/", routes![meta, roll, new_game, get_game, join_game, get_player, ready_player])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
