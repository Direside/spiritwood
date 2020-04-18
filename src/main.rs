#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use uuid::Uuid;
use rocket::State;
use rocket::http::Method;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors;
use rocket_cors::catch_all_options_routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};

use rocket::request::FromParam;
use rocket::http::RawStr;

use std::sync::Mutex;
use std::collections::HashMap;

use crate::state::Game;
use crate::api::{GameDescription, GameState, Player, PlayerUpdate, Tile};

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

#[put("/game/<uuid>/<name>", rank=3)]
fn join_game(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    let mut player: Option<Json<Player>> = None;
    with_game(games, uuid, |game| {
        player = if game.description.state == GameState::WAITING {
            Some(Json(game.join_new_player(name)))
        } else {
            None
        }
    });
    player
}

// TODO auth
#[put("/game/<uuid>/<name>", format = "json", data = "<update>", rank=2)]
fn ready_player(games: State<Games>, uuid: UUID, name: String, update: Json<PlayerUpdate>) -> Option<Json<Player>> {
    let mut player: Option<Player> = None;
    with_game(games, uuid, |game| {
        if game.description.state == GameState::WAITING {
            player = game.update_player(&name, &update)
        }
    });
    player.map(|p| Json(p))
}

#[get("/game/<uuid>/<name>", rank=2)]
fn get_player(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    games.lock().unwrap().get(&uuid.uuid).map(|game| {
        game.players.iter().find(|p| p.name == name).map(|p| Json(p.clone()))
    }).flatten()
}

#[get("/game/<uuid>/tile", rank=1)]
fn get_tile(games: State<Games>, uuid: UUID) -> Option<Json<Tile>> {
    let mut tile: Option<Tile> = None;
    with_game(games, uuid, |game| {
        tile = game.pop_tile()
    });
    tile.map(|t| Json(t))
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

fn rocket() -> Result<rocket::Rocket, Error> {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Options].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()?;

    Ok(rocket::ignite()
        .manage(Meta::generate())
        .manage(Mutex::new(HashMap::<Uuid, Game>::new()))
        .mount("/", routes![meta, roll, new_game, get_game, join_game,
                            get_player, ready_player, get_tile])
//        .mount("/", catch_all_options_routes())
        .attach(cors)
        .register(catchers![not_found]))
}

fn main() -> Result<(), Error> {
    rocket()?.launch();
    Ok(())
}
