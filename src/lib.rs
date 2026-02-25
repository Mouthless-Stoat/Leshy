struct FightManager<Sigil> {
    board: Board,
    cards: Vec<Card<Sigil>>,
}

struct Player {
    hand: Vec<CardID>,
}

struct Board {
    first: Vec<Slot>,
    second: Vec<Slot>,
    scale: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CardID(isize);

struct Slot {
    card_id: CardID,
}

struct Card<Sigil> {
    id: CardID,
    name: String,
    attack: f64,
    health: f64,
    sigils: Vec<Sigil>,
}

impl<Sigil> FightManager<Sigil> {
    fn get_mut_from_id(&mut self, id: CardID) -> Option<&mut Card<Sigil>> {
        self.cards.iter_mut().find(|card| card.id == id)
    }
}
