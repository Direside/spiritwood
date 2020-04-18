use crate::api::{Etag, Href, Key, GameDescription, Player, GameState, PlayerState, PlayerUpdate, Tile};
use rand::thread_rng;
use rand::Rng;
use rand::seq::SliceRandom;

// complete record of the game that's stored on the server
#[derive(Debug)]
pub struct Game {
    pub description: GameDescription,
    pub players: Vec<Player>,
    pub turns: Vec<Turn>,
    pub tileset: Vec<Tile>
}

impl Game {
    pub fn create() -> Game {
        let tileset = Tile::load_tiles();
        let mut rng = thread_rng();
        let tiles: Vec<Tile> = (1..20).map(|_| {
            tileset[rng.gen_range(1, tileset.len())].clone()
        }).collect();

        Game {
            description: GameDescription::default(),
            players: vec![],
            turns: vec![],
            tileset: tiles // TODO: tileset
        }
    }

    pub fn join_new_player(&mut self, name: String) -> Player {
        let player = Player::new(self.players.len(), name);
        self.players.push(player.clone());
        self.description.players.push(player.name.clone());
        player
    }

    pub fn start_game(&mut self) {
        self.description = GameDescription {
            state: GameState::PLAYING,
            ..self.description.clone()
        }
    }

    // TODO: move to turn
    pub fn pop_tile(&mut self) -> Option<Tile> {
        self.tileset.pop()
    }
//    fn turn(&mut self) -> Turn {}
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
pub struct Move {
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
