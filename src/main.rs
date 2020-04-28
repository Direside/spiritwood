#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket::State;

use rocket::http::{Method, RawStr};
use rocket::request::FromParam;
use rocket_contrib::json::{Json, JsonValue};
use rocket_cors::{self, AllowedHeaders, AllowedOrigins, Error};
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::state::Game;
use crate::api::{Move, GameDescription, Player, PlacedTile, Tile};
use crate::fail::{FailResponse, not_found, conflict, bad_request};

mod api;
mod dice;
mod fail;
mod state;

#[derive(Serialize, Deserialize)]
struct Meta {
    name: String,
    version: String,
    commit: String
}

type Result<A> = ::std::result::Result<A, FailResponse>;

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

fn with_game<A, F: FnOnce(&mut Game) -> Result<A>> (games: State<Games>, id: UUID, action: F) -> Result<A> {
    let mut result = Err(not_found("Game not found."));
    let mut all = games.lock().unwrap();
    all.entry(id.uuid).and_modify(|g| result = action(g));
    result
}

#[put("/game/<uuid>?<player>")]
fn join_game(games: State<Games>, uuid: UUID, player: String) -> Result<Json<Player>> {
    with_game(games, uuid, |game| {
        if game.players_can_join() {
            Ok(Json(game.join_new_player(player)))
        } else {
            Err(bad_request("Game already started."))
        }
    })
}

// TODO auth
#[put("/game/<uuid>/start")]
fn start_game(games: State<Games>, uuid: UUID) -> Result<Json<GameDescription>> {
    with_game(games, uuid, |game| {
        game.start_game();
        Ok(Json(game.get_description()))
    })
}

#[get("/game/<uuid>/<name>", rank=2)]
fn get_player(games: State<Games>, uuid: UUID, name: String) -> Result<Json<Player>> {
    with_game(games, uuid, |game| {
        game.get_player(&name).map(|p| Json(p)).ok_or(not_found("Player not found."))
    })
}

#[get("/game/<uuid>/tile")]
fn get_next_tile(games: State<Games>, uuid: UUID) -> Result<Json<Tile>> {
    with_game(games, uuid, |game| {
        game.pop_tile().map(|p| Json(p)).ok_or(not_found("No more tiles!"))
    })
}

#[get("/game/<uuid>/tiles?<x>&<y>&<radius>")]
fn get_tile(games: State<Games>, uuid: UUID, x: i8, y: i8, radius: Option<u8>) -> Result<Json<Vec<PlacedTile>>> {
    with_game(games, uuid, |game| {
        let radius = match radius {
            Some(r) => r,
            None => 5,
        };
        Ok(Json(game.get_tiles(x, y, radius)))
    })
}

#[put("/game/<uuid>/placetile", data = "<placement>")]
fn place_tile(games: State<Games>, uuid: UUID, placement: Json<PlacedTile>) -> Result<Json<Tile>> {
    with_game(games, uuid, |game| {
        if game.board_space_open(placement.x, placement.y) {
            game.apply(Move::PlaceTile { x: placement.x, y: placement.y, tile: placement.tile.clone() });
            Ok(Json(placement.tile.clone()))
        } else {
            Err(conflict(&"A tile has already been placed here."))
        }
    })
}

#[put("/game/<uuid>/endturn")]
fn end_turn(games: State<Games>, uuid: UUID) -> Result<Json<GameDescription>> {
    with_game(games, uuid, |game| {
        game.apply(Move::EndTurn);
        Ok(Json(game.get_description()))
    })
}

struct UUID {
    uuid: Uuid
}

impl<'a> FromParam<'a> for UUID {
    type Error = uuid::Error;

    #[inline(always)]
    fn from_param(param: &'a RawStr) -> ::std::result::Result<UUID, Self::Error> {
        Uuid::parse_str(param).map(|u| UUID { uuid: u })
    }
}

#[catch(404)]
fn not_found_catcher() -> JsonValue {
    json!({"status": 404, "message": "Not Found."})
}


fn rocket() -> ::std::result::Result<rocket::Rocket, Error> {
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
        .register(catchers![not_found_catcher]))
}

fn main() -> ::std::result::Result<(), Error> {
    rocket()?.launch();
    Ok(())
}
