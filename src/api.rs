use uuid::Uuid;

pub type Etag = Uuid;
pub type Key = Uuid;
pub type Href = String;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum GameState { WAITING, PLAYING, FINISHED }

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum PlayerState { WAITING, READY, ACTIVE }

// Players send these to the server, which responds with Turns
// all should have time and signature
trait Move {}

struct ReadyToStart{ name: String }
struct PlaceTile{}
struct DrawCard{}
struct RollDice{}

impl Move for ReadyToStart {}
impl Move for PlaceTile {}
impl Move for DrawCard {}
impl Move for RollDice {}

#[derive(Clone, Debug, Serialize)]
pub struct GameDescription {
    pub id: Uuid,
    pub state: GameState,
    pub href: Href,
    pub players: usize,
    pub turn: u32,
    pub current: Etag,
}

impl Default for GameDescription {
    fn default() -> GameDescription {
        let id = Uuid::new_v4();
        let etag = Uuid::new_v4();
        GameDescription {
            id: id,
            state: GameState::WAITING,
            href: format!("/game/{}", id),
            players: 0,
            turn: 0,
            current: etag
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct Player {
    pub name: String,
    pub state: PlayerState,
    order: usize,
    key: Key
    // colour? character?
}

impl Player {
    pub fn new(order: usize, name: String) -> Player {
        Player {
            name: name,
            state: PlayerState::WAITING,
            order: order,
            key: Uuid::new_v4()
        }
    }
}
