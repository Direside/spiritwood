struct GameDescription {
    id: UUID,
    href: String,
    players: u16,
    active_turn: u32,
    current_state: Etag
}

impl GameDescription {
    fn for_url(this: &self, url: String) {
        copy(href = format!("{}/{}", url, this.href))
    }
}

impl Default for GameDescription {
    fn default() -> GameDescription {
        GameDescription {
            id: UUID::new(),
            href: format!("/games/{}", id),
            players: default(),
            active_turn: default(),
            active_player: default()
        }
    }
}

// complete record of the game that's stored on the server
struct Game {
    description: GameDescription,
    turns: Vec<Turn>
}

type Etag = String

// impl Default for everything

// turn state, i.e. what's sent to FE
struct Turn {
    id: u32,
    player: Player,
    non_players: Vec<Player>
    decks: Decks,
    board: Board
}

impl Default for Turn {
    // i.e. set up the game
    fn default() -> Turn {
    }
}

struct Player {
    order: u16,
    // colour? character?
}

struct Decks {
    graveyard: Vec<Card>,
    bones: Vec<Card>
}

struct Board {
    // change this to be (Board, Change)?
    tiles: Vec<Vec<Tile>>
}

struct Tile {
    // TODO
}

struct Card {
    // TODO
}

    


