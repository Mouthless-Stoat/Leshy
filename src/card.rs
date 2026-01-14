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
    pub fn sigils(&self) -> &Vec<Sigil> {
        &self.sigils
    }
}

/// Representation of a playing card, both in the hand and
#[derive(Clone)]
pub struct Card<Sigil: Clone + 'static> {
    /// Modification to the power of the card.
    pub pow_mod: isize,
    /// Modification to the health of the card.
    pub health_mod: isize,
    data: &'static CardData<Sigil>,
}

impl<Sigil: Clone> Card<Sigil> {
    /// The base original data used for the card.
    pub fn data(&self) -> &'static CardData<Sigil> {
        self.data
    }
}
