use crate::api::{GameDescription, Player, Move, GameState, PlacedTile, Tile};
use crate::state::{Biome, Card};
use rand::thread_rng;
use rand::Rng;
use std::collections::HashMap;

use uuid::Uuid;

type Rotation = u8; // 0, 1, 2, 3
type TileID = u32;

pub struct GameConfig {
    pub tile_deck: i64
}

impl Default for GameConfig {
    fn default() -> GameConfig {
        GameConfig {
            tile_deck: 10,
        }
    }
}

#[derive(Debug, Default)]
pub struct Game {
    id: Uuid,
    etag: Uuid,
    state: GameState,
    turn: u32,
    players: Vec<Player>,
    current_player: usize, // Corresponds to index in players array.
    tile_stack: Vec<TileID>,
    tile_repo: TileRepository,
    visible_tile: Option<Tile>,
    tilemap: TileMap,
    biomes: Vec<Biome>, // TODO: useful structure for these
    discard: Vec<Card>
}

impl Game {
    pub fn create(config: &GameConfig) -> Game {
        let id = Uuid::new_v4();
        let etag = Uuid::new_v4();
        let tileset = Tile::load_tiles();
        let biomes = Biome::load_biomes();
        let mut rng = thread_rng();
        let tile_repo = TileRepository::new(&tileset);
        let tile_stack: Vec<u32> = (1..config.tile_deck).map(|_| {
            tileset[rng.gen_range(1, tileset.len())].id
        }).collect();

        Game {
            id: id,
            etag: etag,
            tile_stack: tile_stack,
            tile_repo: tile_repo,
            tilemap: TileMap::new(),
            biomes: biomes,
            ..Default::default()
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
            current_player = Some(self.current_player);
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
        self.current_player = 0;
        self.turn = 1;
    }

    pub fn pop_tile(&mut self) -> GameResult<Tile> {
        if self.visible_tile.is_some() {
            Ok(self.visible_tile.as_ref().unwrap().clone())
        } else {
            self.tile_stack.pop().and_then(|tile_id| {
                let tile = self.tile_repo.get(tile_id);
                tile.as_ref().map(|t| { self.visible_tile = Some(t.clone())});
                tile
            }).ok_or(GameplayError::ItemNotFound("No more tiles!"))
        }
    }

    pub fn get_player(&self, name: &str) -> Option<Player> {
        self.players.iter().find(|p| p.name == name).map(|p| p.clone())
    }

    pub fn get_tile(&self, x: i8, y: i8) -> Option<Tile> {
        self.tilemap.get_tile(x, y).map(|(id, _)| self.tile_repo.get(id).unwrap())
    }

    pub fn get_tiles(&self, x: i8, y: i8, radius: u8) -> GameResult<Vec<PlacedTile>> {
        if radius > 10 {
            return Err(GameplayError::IllegalMove("Cannot request more than 401 tiles (r=10)."))
        }

        let mut result = vec![];
        for i in (x - radius as i8)..(x + radius as i8) {
            for j in (y - radius as i8)..(y as i8 + radius as i8) {
                match self.tilemap.get_tile(i, j).and_then(|(tile_id, r)| self.tile_repo.get(tile_id).map(|t| (t, r))) {
                    Some((tile, rotation)) => result.push(PlacedTile {
                        x: i,
                        y: j,
                        rotation: rotation,
                        tile: tile,
                    }),
                    None => {},
                }
            }
        }
        
        Ok(result)
    }

    pub fn apply(&mut self, action: Move) -> GameResult<()> {
        // TODO: Actually execute the moves
        match action {
            Move::ReadyToStart { name: _ } =>
                GameplayError::not_implemented("Move: ReadyToStart"),
            Move::RollDice {} => GameplayError::not_implemented("Move: RollDice"),
            Move::DrawCard {} => GameplayError::not_implemented("Move: DrawCard"),
            Move::PlaceTile { x, y, tile_id, rotation } => self.set_tile(x, y, tile_id, rotation),
            Move::EndTurn {} => self.end_turn(),
        }
    }

    fn set_tile(&mut self, x: i8, y: i8, tile_id: TileID, rotation: Rotation) -> GameResult<()> {
        match self.get_tile(x, y) {
            Some(_) => Err(GameplayError::IllegalMove("A tile has already been placed here.")),
            None => {
                self.tilemap.set_tile(x, y, tile_id, rotation);
                Ok(())
            },
        }

    }

    fn end_turn(&mut self) -> GameResult<()> {
        let next_player = (self.current_player + 1) % self.players.len();
        self.current_player = next_player;
        self.turn += 1;
        Ok(())
    }
}

type GameResult<T> = ::std::result::Result<T, GameplayError>;
#[derive(Debug)]
pub enum GameplayError {
    IllegalMove(&'static str),
    OutOfTurn(&'static str),
    ItemNotFound(&'static str), // Toby: did you mean for such things to be "gameplay" errors?
    NotImplemented(&'static str)
}

impl GameplayError {
    fn not_implemented<T>(str: &'static str) -> GameResult<T> {
        Err(GameplayError::NotImplemented(str))
    }
}

#[derive(Debug, Default)]
pub struct TileMap {
    tiles: HashMap::<TilePosition, (TileID, Rotation)>,
}

impl TileMap {
    pub fn new() -> Self {
        Self {
            tiles: HashMap::<TilePosition, (TileID, Rotation)>::new(),
        }
    }

    pub fn get_tile(&self, x: i8, y: i8) -> Option<(TileID, Rotation)> {
        let pos = TilePosition { x, y };
        self.tiles.get(&pos).map(|(tile, r)| (tile.clone(), r.clone()))

    }

    pub fn set_tile(&mut self, x: i8, y: i8, tile_id: TileID, rotation: Rotation) {
        let pos = TilePosition { x, y };
        self.tiles.insert(pos, (tile_id, rotation));
    }
}

#[derive(Debug, Default, PartialEq, Eq, Hash)]
struct TilePosition {
    x: i8,
    y: i8,
}

#[derive(Debug, Default)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_description() {
        let game = Game::create(&Default::default());

        let desc = game.get_description();
        assert_eq!(desc.id, game.id);
        assert_eq!(desc.state, GameState::WAITING);
        assert_eq!(desc.players.len(), 0);
        assert_eq!(desc.current_player, None);
        assert_eq!(desc.turn, 0);
    }

    #[test]
    fn test_pop_tile() {
        let mut game = Game::create(&Default::default());

        let tile = game.pop_tile();
        assert!(tile.is_ok());
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

        tile_map.set_tile(5, 8, tileset[1].id, 0);

        assert_eq!(tile_map.get_tile(5, 8), Some((tileset[1].id, 0)));
    }

    #[test]
    fn turns() {
        let mut game = Game::create(&Default::default());
        game.join_new_player("alice".to_string());
        game.join_new_player("bob".to_string());

        assert_eq!(game.turn, 0);
        assert_eq!(game.current_player, 0);
        game.start_game();
        assert_eq!(game.turn, 1);
        assert_eq!(game.current_player, 0);
        assert!(game.end_turn().is_ok());
        assert_eq!(game.turn, 2);
        assert_eq!(game.current_player, 1);
        assert!(game.end_turn().is_ok());
        assert_eq!(game.turn, 3);
        assert_eq!(game.current_player, 0);
    }

    #[test]
    fn visible_tile() {
        let mut game = Game::create(&Default::default());
        game.start_game();
        assert_eq!(game.visible_tile, None);
        let tile = game.pop_tile().unwrap();
        assert_eq!(game.visible_tile, Some(tile.clone()));
        assert_eq!(game.pop_tile().unwrap(), tile.clone());
    }
}
