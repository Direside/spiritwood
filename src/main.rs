#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::State;

use rocket::http::{Method, RawStr};
use rocket::request::FromParam;
use rocket_contrib::json::Json;
use rocket_cors::{self, AllowedHeaders, AllowedOrigins, Error};
use rocket::fairing::AdHoc;
use std::collections::HashMap;
use std::sync::Mutex;
use uuid::Uuid;

use crate::game::{Game, GameConfig, GameplayError};
use crate::api::{Move, GameDescription, Player, PlacedTile, Tile};
use crate::fail::{FailResponse, not_found, conflict, bad_request, server_error, unprocessable};

mod api;
mod cards;
mod dice;
mod fail;
mod game;
mod state;

#[derive(Clone, Serialize, Deserialize)]
struct Meta {
    name: String,
    version: String,
    commit: String
}

type ServerResult<A> = ::std::result::Result<A, FailResponse>;

const PKG_NAME: &str = env!("CARGO_PKG_NAME");
const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const GIT_COMMIT: &str = env!("GIT_HEAD");

impl Meta {
    pub fn generate() -> MetaHolder {
        MetaHolder {
            meta: Meta {
                    name: PKG_NAME.to_string(),
                    version: PKG_VERSION.to_string(),
                    commit: GIT_COMMIT.to_string()
                }
        }
    }
}

struct MetaHolder {
    meta: Meta
}

type Games = Mutex<HashMap<Uuid, Game>>;

#[get("/meta")]
fn meta(state: State<MetaHolder>) -> Json<Meta> {
    Json(state.meta.clone())
}

#[get("/roll")]
fn roll() -> Json<Vec<u16>> {
    Json(dice::roll(1, 6))
}

#[options("/game")]
fn new_game_preflight() -> Json<u8> {
    Json(0)
}

#[post("/game")]
fn new_game(games: State<Games>, config: State<GameConfig>) -> Json<GameDescription> {
    let game = Game::create(&config);
    let description = game.get_description();
    games.lock().unwrap().insert(description.id, game);
    Json(description)
}

#[get("/game/<uuid>")]
fn get_game(games: State<Games>, uuid: UUID) -> ServerResult<Json<GameDescription>> {
    let all = games.lock().unwrap();
    let game = all.get(&uuid.uuid).ok_or(not_found("Game not found!"))?;
    Ok(Json(game.get_description()))
}

fn with_game<A, F: FnOnce(&mut Game) -> ServerResult<A>> (games: State<Games>, id: UUID, action: F) -> ServerResult<A> {
    let mut result = Err(not_found("Game not found."));
    let mut all = games.lock().unwrap();
    all.entry(id.uuid).and_modify(|g| result = action(g));
    result
}

#[put("/game/<uuid>?<player>")]
fn join_game(games: State<Games>, uuid: UUID, player: String) -> ServerResult<Json<Player>> {
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
fn start_game(games: State<Games>, uuid: UUID) -> ServerResult<Json<GameDescription>> {
    with_game(games, uuid, |game| {
        game.start_game();
        Ok(Json(game.get_description()))
    })
}

#[get("/game/<uuid>/<name>", rank=2)]
fn get_player(games: State<Games>, uuid: UUID, name: String) -> ServerResult<Json<Player>> {
    with_game(games, uuid, |game| {
        let player = game.get_player(&name).ok_or(not_found("Player not found."))?;
        Ok(Json(player))
    })
}

#[get("/game/<uuid>/tile")]
fn get_next_tile(games: State<Games>, uuid: UUID) -> ServerResult<Json<Tile>> {
    with_game(games, uuid, |game| {
        let tile = game.pop_tile()?;
        Ok(Json(tile))
    })
}

#[get("/game/<uuid>/tiles?<x>&<y>&<radius>")]
fn get_tile(games: State<Games>, uuid: UUID, x: i8, y: i8, radius: Option<u8>) -> ServerResult<Json<Vec<PlacedTile>>> {
    with_game(games, uuid, |game| {
        let radius = match radius {
            Some(r) => r,
            None => 5,
        };
        let tiles = game.get_tiles(x, y, radius)?;
        Ok(Json(tiles))
    })
}

#[derive(Deserialize, Serialize)]
struct PlaceTileRequestBody {
    x: i8,
    y: i8,
    tile: u32, // Tile ID
    rotation: u8 // rotation
}

#[put("/game/<uuid>/placetile", data = "<body>")]
fn place_tile(games: State<Games>, uuid: UUID, body: Json<PlaceTileRequestBody>) -> ServerResult<Json<Tile>> {
    with_game(games, uuid, |game| {
        game.apply(Move::PlaceTile { x: body.x, y: body.y, tile_id: body.tile, rotation: body.rotation })?;
        let tile = game.get_tile(body.x, body.y).ok_or(server_error("tile placement failed"))?;
        Ok(Json(tile))
    })
}

#[put("/game/<uuid>/endturn")]
fn end_turn(games: State<Games>, uuid: UUID) -> ServerResult<Json<GameDescription>> {
    with_game(games, uuid, |game| {
        game.apply(Move::EndTurn)?;
        Ok(Json(game.get_description()))
    })
}

impl From<GameplayError> for FailResponse {
    fn from(err: GameplayError) -> FailResponse {
        match err {
            GameplayError::IllegalMove(msg) => conflict(msg),
            GameplayError::OutOfTurn(msg) => bad_request(msg),
            GameplayError::ItemNotFound(msg) => not_found(msg),
            GameplayError::NotImplemented(msg) => server_error(msg)
        }
    }
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
       .register(catchers![catch_not_found, catch_unprocessable])
       .attach(cors)
       .attach(AdHoc::on_attach("Game Config", |r| {
           let game_config = GameConfig {
               tile_deck: r.config().get_int("tile_deck").unwrap()
           };
           Ok(r.manage(game_config))
       })))
}

#[catch(404)]
fn catch_not_found() -> FailResponse {
    not_found("Endpoint not found!")
}

#[catch(422)]
fn catch_unprocessable() -> FailResponse {
    // TODO somehow get in earlier and get serde details
    unprocessable("Couldn't parse request!")
}

fn main() -> ::std::result::Result<(), Error> {
    rocket()?.launch();
    Ok(())
}
