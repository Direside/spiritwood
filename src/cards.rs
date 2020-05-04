use crate::state::Character;

#[derive(Clone)]
pub enum Effect {
    PLAYER { effect: &'static dyn FnMut(Character) },
    ITEM { name: &'static str }
}

// These may only be added to, not changed.
const CARD_HOTDOG_0001: &'static dyn FnMut(Character) = &|mut c: Character| {
    c.health += 1;
};
//

pub fn effect(name: &'static str) -> Effect {
    match name {
        "CARD_HOTDOG_0001" => Effect::PLAYER { effect: CARD_HOTDOG_0001 },
        _ => Effect::ITEM { name: name }
    }
}
