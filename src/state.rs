use crate::api::{Etag, Href, Key, GameDescription, Player};

// complete record of the game that's stored on the server
#[derive(Debug)]
pub struct Game {
    pub description: GameDescription,
    pub players: Vec<Player>,
    pub turns: Vec<Turn>,
}

impl Game {
    pub fn create() -> Game {
        Game {
            description: GameDescription::default(),
            players: vec![],
            turns: vec![]
        }
    }
    pub fn join_new_player(&mut self, name: String) -> Player {
        let player = Player::new(self.players.len(), name);        
        self.players.push(player.clone());
        self.description.players = self.players.len();
        player
    }

//    fn turn(&mut self) -> Turn {}
}

// impl Default for everything

// turn state, i.e. what's sent to FE
#[derive(Debug)]
pub struct Turn {
    id: u32,
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
pub enum Rotation { R_0, R_90, R_180, R_270 }

#[derive(Debug)]
pub struct Board {
    // change this to be (Board, Change)?
    tiles: Vec<Vec<(Tile, Rotation)>>
}

#[derive(Debug)]
pub struct Tile {
    // TODO
}

#[derive(Debug)]
pub struct Card {
    // TODO
}

// Moves
