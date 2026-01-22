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
///
/// [`first`](Board::first) and [`second`](Board::second) store the slot from their respective
/// player perspective. That mean when seeing which card is opposing each other the slot should be
/// in verse for the opposing player: first player left most card (0) opposed second player right
/// most card (3).
#[derive(Clone)]
#[allow(missing_docs)]
pub struct Board<Sigil: Clone + 'static> {
    pub first: Vec<Slot<Sigil>>,
    pub second: Vec<Slot<Sigil>>,
}

impl<Sigil: Clone> Board<Sigil> {
    /// Create a new, empty [`Board`]. All slot are filled with [`Slot::Blank`]
    pub fn new() -> Self {
        Board {
            first: vec![Slot::Blank; 4],
            second: vec![Slot::Blank; 4],
        }
    }
}

impl<Sigil: Clone> Default for Board<Sigil> {
    fn default() -> Self {
        Self::new()
    }
}
