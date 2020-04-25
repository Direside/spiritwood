#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;
use rocket::http::Method;
use rocket::http::RawStr;
use rocket::request::FromParam;
use rocket::response::status::Conflict;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::catch_all_options_routes;
use rocket_cors::{AllowedHeaders, AllowedOrigins, Error};
use rocket_cors;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::state::Game;
use crate::api::{Move, GameDescription, Player, PlacedTile, Tile};

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
    let description = game.get_description();
    games.lock().unwrap().insert(description.id, game);
    Json(description)
}

#[get("/game/<uuid>")]
fn get_game(games: State<Games>, uuid: UUID) -> Option<Json<GameDescription>> {
    let all = games.lock().unwrap();
    all.get(&uuid.uuid).map(|game| Json(game.get_description()))
}

fn with_game<A, F: FnOnce(&mut Game) -> A>(games: State<Games>, id: UUID, default: A, action: F) -> A {
    let mut result: A = default;
    let mut all = games.lock().unwrap();
    all.entry(id.uuid).and_modify(|g| result = action(g));
    result
}

#[put("/game/<uuid>?<player>")]
fn join_game(games: State<Games>, uuid: UUID, player: String) -> Option<Json<Player>> {
    let new_player = with_game(games, uuid, None, |game| {
        if game.players_can_join() {
            Some(Json(game.join_new_player(player)))
        } else {
            None
        }
    });
    new_player
}

// TODO auth
#[put("/game/<uuid>/start")]
fn start_game(games: State<Games>, uuid: UUID) -> Option<Json<GameDescription>> {
    let mut all = games.lock().unwrap();
    all.entry(uuid.uuid).and_modify(|game| {game.start_game()});
    all.get(&uuid.uuid).map(|game| Json(game.get_description()))
}

#[get("/game/<uuid>/<name>", rank=2)]
fn get_player(games: State<Games>, uuid: UUID, name: String) -> Option<Json<Player>> {
    games.lock().unwrap().get(&uuid.uuid).map(|game| {
        game.get_player(&name).map(|p| Json(p))
    }).flatten()
}

#[get("/game/<uuid>/tile")]
fn get_next_tile(games: State<Games>, uuid: UUID) -> Option<Json<Tile>> {
    let tile = with_game(games, uuid, None, |game| {
        game.pop_tile()
    });
    tile.map(|t| Json(t))
}

#[get("/game/<uuid>/tiles?<x>&<y>&<radius>")]
fn get_tile(games: State<Games>, uuid: UUID, x: i8, y: i8, radius: Option<u8>) -> Option<Json<Vec<PlacedTile>>> {
    let tile = with_game(games, uuid, None, |game| {
        let radius = match radius {
            Some(r) => r,
            None => 5,
        };
        Some(game.get_tiles(x, y, radius))
    });
    tile.map(|t| Json(t))
}

#[put("/game/<uuid>/tiles/<x>/<y>", data = "<tile>")]
fn place_tile(games: State<Games>, uuid: UUID, x: i8, y: i8, tile: Json<Tile>) -> Result<Json<Tile>, Conflict<JsonValue>> {
    let existed = with_game(games, uuid, false, |game| {
        if game.board_space_open(x, y) {
            game.apply(Move::PlaceTile { x, y, tile: tile.clone() });
            false
        } else {
            true
        }
    });
    if existed {
        Err(conflict(&"A tile has already been placed here.".to_string()))
    } else {
        Ok(tile)
    }
}

#[put("/game/<uuid>/endturn")]
fn end_turn(games: State<Games>, uuid: UUID) -> Option<Json<GameDescription>> {
    let desc = with_game(games, uuid, None, |game| {
        game.apply(Move::EndTurn);
        Some(game.get_description())
    });
    desc.map(|d| Json(d))
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

fn conflict(message: &String) -> Conflict<JsonValue> {
    Conflict(Some(json!({"status": 409, "message": *message})))
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
                            get_player, start_game, get_next_tile, get_tile, place_tile, end_turn])
        .attach(cors)
        .register(catchers![not_found]))
}

fn main() -> Result<(), Error> {
    rocket()?.launch();
    Ok(())
}
