use uuid::Uuid;

pub type Etag = Uuid;
pub type Key = Uuid;
pub type Href = String;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum GameState { WAITING, PLAYING, FINISHED }

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum PlayerState { WAITING, READY, ACTIVE }


// Players send these to the server, which responds with Turns
// all should have time and signature
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Move {
    ReadyToStart { name: String },
    PlaceTile {},
    DrawCard {},
    RollDice {},
}

#[derive(Clone, Debug, Serialize)]
pub struct GameDescription {
    pub id: Uuid,
    pub state: GameState,
    pub href: Href,
    pub players: Vec<String>,
    pub turn: u32,
    pub current: Etag,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    order: usize,
    key: Key
    // colour? character?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerUpdate {
    pub name: String,
    pub state: PlayerState, // TODO: Option
}

impl Player {
    pub fn new(order: usize, name: String) -> Player {
        Player {
            name: name,
            order: order,
            key: Uuid::new_v4()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Tile {
    pub id: u32,
    pub image: String,
    pub symbol: char,
}
