use crate::Card;

/// Representation of a slot used on the [`Board`].
#[derive(Clone)]
pub enum Slot<Sigil: Clone + 'static> {
    /// A blank slot with no card
    Blank,
    /// A slot with a card in it
    Card(Card<Sigil>),
}

/// Representation of a collection of [`Slot`] for the board.
#[derive(Clone)]
#[allow(missing_docs)]
pub struct Board<Sigil: Clone + 'static> {
    pub p1: Vec<Slot<Sigil>>,
    pub p2: Vec<Slot<Sigil>>,
}

impl<Sigil: Clone> Board<Sigil> {
    /// Create a new, empty [`Board`]. All slot are filled with blank
    pub fn new() -> Self {
        Board {
            p1: vec![Slot::Blank; 4],
            p2: vec![Slot::Blank; 4],
        }
    }
}

impl<Sigil: Clone> Default for Board<Sigil> {
    fn default() -> Self {
        Self::new()
    }
}
