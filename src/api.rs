use uuid::Uuid;

pub type Etag = Uuid;
pub type Key = Uuid;
pub type Href = String;

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub enum GameState { WAITING, PLAYING, FINISHED }

#[derive(Clone, Debug, Serialize)]
pub struct GameDescription {
    pub id: Uuid,
    pub state: GameState,
    href: String,
    pub players: usize,
    turn: u32,
    current: Etag,
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
    order: usize,
    key: Key
    // colour? character?
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
