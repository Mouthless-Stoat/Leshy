use crate::{PlayerID, sigil};

/// Representation of the data of the card, this is the original unmodified data of the card.
/// To modify this data use a [`Card`] instead, which is simply a wrapper around this type, with
/// ability to modify the data.
#[derive(Clone)]
pub struct CardData<Sigil: Clone> {
    power: isize,
    health: isize,
    sigils: Vec<Sigil>,
}

impl<Sigil: Clone> CardData<Sigil> {
    /// The power or attack of the card.
    pub fn power(&self) -> isize {
        self.power
    }

    /// The health of the card. When a card take damage this value stay unchanged but
    /// [`Card::health_mod`] is changed instead.
    pub fn health(&self) -> isize {
        self.health
    }

    /// The sigils on this card.
    pub fn sigils_mut(&mut self) -> &mut [Sigil] {
        &mut self.sigils
    }

    /// The sigils on this card.
    pub fn sigils(&self) -> &Vec<Sigil> {
        &self.sigils
    }
}

/// Representation of a playing card, both in the hand and on the board.
#[derive(Clone)]
pub struct Card<Sigil: Clone + 'static> {
    /// Modification to the power of the card.
    pub pow_mod: isize,
    /// Modification to the health of the card.
    pub health_mod: isize,
    pub data: &'static CardData<Sigil>,
    /// The owner of this card.
    pub owner: PlayerID,
    pub sigils: Vec<Sigil>,
}
