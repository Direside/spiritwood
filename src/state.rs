use crate::api::Tile;

// impl Default for everything

// turn state, i.e. what's sent to FE
#[derive(Clone, Debug)]
pub struct Card {
    id: u32,
    name: String,
    effect: &'static str
}

#[derive(Clone, Debug, Default)]
pub struct Biome {
    id: u32,
    name: String,
    deck: Vec<Card>
}

#[derive(Clone, Debug)]
pub enum Rotation { NONE, RIGHT, HALF, LEFT }

#[derive(Clone, Debug, Default)]
pub struct Board {
    // change this to be (Board, Change)?
    tiles: Vec<Vec<(Tile, Rotation)>>
}

impl Tile {
    fn new(id: u32, symbol: char, image: &'static str) -> Tile {
        Self {id: id, symbol: symbol, image: image.to_string()}
    }

    // TODO DB
    pub fn load_tiles() -> Vec<Tile> {
        vec![
            Self::new(1, '+', "images/tiles/cross.png"),
            Self::new(2, '-', "images/tiles/straight.png"),
            Self::new(3, '-', "images/tiles/straight-river.png"),
            Self::new(4, 'T', "images/tiles/branch.png"),
            Self::new(5, 'L', "images/tiles/corner.png"),
            Self::new(6, 'L', "images/tiles/corner-mine.png"),
            Self::new(7, '.', "images/tiles/branch.png"),

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
    pub health: u16,
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

impl Card {
    pub fn load_cards() -> Vec<Card> {
        vec![
            Card {
                id: 0,
                name: "Hot Dog".to_string(),
                effect: "CARD_HOTDOG_0001"
            }
        ]
    }
}

impl Biome {
    fn new(id: u32, name: &'static str) -> Biome {
        Self {
            id: id,
            name: name.to_string(),
            deck: vec![]
        }
    }

    pub fn load_biomes() -> Vec<Biome> {
        let biomes = vec![
            Self::new(0, "Nature"),
            Self::new(1, "Town"),
            Self::new(2, "Industrial"),
            Self::new(3, "Mystical"),
            Self::new(4, "None")
        ];

        biomes
    }
}
