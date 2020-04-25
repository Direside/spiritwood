use crate::api::{Etag, Href, Key, GameDescription, Player, Move, GameState, PlayerState, PlayerUpdate, Tile};
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

use std::collections::HashMap;

use uuid::Uuid;

// complete record of the game that's stored on the server
#[derive(Debug)]
pub struct Game {
    id: Uuid,
    etag: Uuid,
    state: GameState,
    turn: u32,
    players: Vec<Player>,
    current_player: usize, // Corresponds to index in players array.
    turns: Vec<Turn>,
    tile_stack: Vec<u32>,
    tile_repo: TileRepository,
    tilemap: TileMap,
}

impl Game {
    pub fn create() -> Game {
        let id = Uuid::new_v4();
        let etag = Uuid::new_v4();
        let tileset = Tile::load_tiles();
        let mut rng = thread_rng();
        let tile_repo = TileRepository::new(&tileset);
        let tile_stack: Vec<u32> = (1..20).map(|_| {
            tileset[rng.gen_range(1, tileset.len())].id
        }).collect();

        Game {
            id: id,
            etag: etag,
            state: GameState::WAITING,
            turn: 0,
            players: vec![],
            current_player: 0,
            turns: vec![],
            tile_stack: tile_stack,
            tile_repo: tile_repo,
            tilemap: TileMap::new(),
        }
    }

    pub fn players_can_join(&self) -> bool {
        self.state == GameState::WAITING
    }

    pub fn get_description(&self) -> GameDescription {
        let mut players = vec![];
        for p in &self.players {
            players.push(p.name.clone());
        }

        let mut current_player = None;
        if self.state == GameState::PLAYING {
            current_player = Some(String::from(&players[self.current_player]));
        }

        GameDescription {
            id: self.id,
            state: self.state,
            href: format!("/game/{}", self.id),
            players: players,
            turn: self.turn,
            current_player: current_player,
            current: self.etag, // TODO: Mutate
        }
    }

    pub fn join_new_player(&mut self, name: String) -> Player {
        let player = Player::new(self.players.len(), name);
        self.players.push(player.clone());
        player
    }

    pub fn start_game(&mut self) {
        self.state = GameState::PLAYING;
    }

    // TODO: move to turn
    pub fn pop_tile(&mut self) -> Option<Tile> {
        self.tile_stack.pop().map(|tile_id| {
            match self.tile_repo.get(tile_id) {
                Some(tile) => tile,
                None => panic!("Tile does not exist: {}", tile_id),
            }
        })
    }
//    fn turn(&mut self) -> Turn {}

    pub fn get_player(&self, name: &str) -> Option<Player> {
        self.players.iter().find(|p| p.name == name).map(|p| p.clone())
    }

    pub fn get_tile(&self, x: i8, y: i8) -> Option<Tile> {
        self.tilemap.get_tile(x, y).and_then(|tile_id| self.tile_repo.get(tile_id))
    }

    pub fn apply(&mut self, action: Move) {
        // TODO: Actually execute the moves
        match action {
            Move::ReadyToStart { name } => panic!("Move: ReadyToStart {}", name),
            Move::RollDice {} => panic!("Move: RollDice"),
            Move::DrawCard {} => panic!("Move: DrawCard"),
            Move::PlaceTile { x, y, tile } => self.set_tile(x, y, tile),
            Move::EndTurn {} => self.end_turn(),
        }
    }

    fn set_tile(&mut self, x: i8, y: i8, tile: Tile) {
        self.tilemap.set_tile(x, y, tile.id);
    }

    fn end_turn(&mut self) {
        let mut next_player = self.current_player + 1;
        if next_player == self.players.len() {
            next_player = 0;
        }
        self.current_player = next_player;
        self.turn += 1;
    }
}

// impl Default for everything

// turn state, i.e. what's sent to FE
#[derive(Debug)]
pub struct Turn {
    id: u32,
    gamestate: GameState,
    player: u16,
    decks: Decks,
    board: Board,
    moves: Vec<Move>,
    etag: Etag,
    href: Href,
}

#[derive(Debug)]
pub struct Decks {
    graveyard: Vec<Card>,
    bones: Vec<Card>
}

#[derive(Debug)]
pub enum Rotation { NONE, RIGHT, HALF, LEFT }

#[derive(Debug)]
pub struct Board {
    // change this to be (Board, Change)?
    tiles: Vec<Vec<(Tile, Rotation)>>
}


impl Tile {
    fn new(id: u32, symbol: char, image: &'static str) -> Tile {
        Tile {id: id, symbol: symbol, image: image.to_string()}
    }

    // TODO DB
    pub fn load_tiles() -> Vec<Tile> {
        vec![
            Self::new(1, '+', "images/tiles/cross.png"),
            Self::new(2, '-', "images/tiles/straight.png"),
            Self::new(3, 'T', "images/tiles/tee.png"),
        ]
     }
}

#[derive(Debug)]
struct TileRepository {
    tile_index: HashMap<u32, Tile>,
}

impl TileRepository {
    fn new(tileset: &Vec<Tile>) -> Self {
        let mut index = HashMap::<u32, Tile>::new();
        for tile in tileset {
            index.insert(tile.id, tile.clone());
        }
        Self {
            tile_index: index,
        }
    }

    fn get(&self, tile_id: u32) -> Option<Tile> {
        self.tile_index.get(&tile_id).map(|t| t.clone())
    }
}

#[derive(Debug)]
pub struct Character {
    id: u32,
    name: String,
    description: String,
    skill: String,
    equipment: u16,
    health: u16,
    speed: u16,
    attack: u16,
}

impl Character {
    fn new(id: u32, name: &'static str, description: &'static str, skill: &'static str,
           equipment: u16, health: u16, speed: u16, attack: u16) -> Character {
        Character {
            id: id,
            name: name.to_string(),
            description: description.to_string(),
            skill: skill.to_string(),
            equipment: equipment,
            health: health,
            speed: speed,
            attack: attack
        }
    }

    // TODO: DB
    pub fn load_characters() -> Vec<Character> {
        vec![
            Self::new(1, "Akeel", "Oldest in a family of seven",
                      "Cartographer: Draw and place one extra tile",
                      1, 3, 2, 3),
        ]
    }
}

#[derive(Debug)]
pub struct Card {
    // TODO
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct TilePosition {
    x: i8,
    y: i8,
}

#[derive(Debug)]
pub struct TileMap {
    tiles: HashMap::<TilePosition, u32>,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::<TilePosition, u32>::new(),
        }
    }

    pub fn get_tile(&self, x: i8, y: i8) -> Option<u32> {
        let pos = TilePosition { x, y };
        self.tiles.get(&pos).map(|tile| tile.clone())

    }

    pub fn set_tile(&mut self, x: i8, y: i8, tile_id: u32) {
        let pos = TilePosition { x, y };
        self.tiles.insert(pos, tile_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_description() {
        let mut game = Game::create();

        let desc = game.get_description();
        assert_eq!(desc.id, game.id);
        assert_eq!(desc.state, GameState::WAITING);
        assert_eq!(desc.players.len(), 0);
        assert_eq!(desc.current_player, None);
        assert_eq!(desc.turn, 0);
    }

    #[test]
    fn test_pop_tile() {
        let mut game = Game::create();

        let tile = game.pop_tile();
        if tile == None {
            panic!("No tile was returned.");
        }
    }

    #[test]
    fn test_tile_repo() {
        let tileset = Tile::load_tiles();
        let test_tile = tileset[1].clone();

        let repo = TileRepository::new(&tileset);

        assert_eq!(repo.get(test_tile.id), Some(test_tile));
        assert_eq!(repo.get(42), None);
    }

    #[test]
    fn test_tile_map() {
        let tileset = Tile::load_tiles();

        let mut tile_map = TileMap::new();

        assert_eq!(tile_map.get_tile(5, 8), None);

        tile_map.set_tile(5, 8, tileset[1].id);

        assert_eq!(tile_map.get_tile(5, 8), Some(tileset[1].id));
    }
}
