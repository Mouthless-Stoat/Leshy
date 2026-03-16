use crate::Sigil;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardID(isize);

/// A bundle of a raw card with its ID and wherever if the card is on the board. The FightManager
/// uses this to handle one master copy of a card that other function borrow from.
pub struct Card<S: Sigil> {
    pub id: CardID,
    pub card: CardData<S>,
    pub on_board: bool,
}

/// The raw card data that store all the information that a card might have.
pub struct CardData<S: Sigil> {
    pub name: String,
    pub attack: f64,
    pub health: f64,
    pub sigils: Vec<S>,
}
