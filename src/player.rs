/// Player ID for the first and second player
#[allow(missing_docs)]
pub enum PlayerID {
    First,
    Second,
}

type Deck<Sigil> = Vec<Card<Sigil>>;

/// Representation of a player.
#[derive(Clone)]
pub struct Player<Sigil: Clone + 'static> {
    /// The card in the player's hand.
    pub hand: Vec<Card<Sigil>>,
    /// The deck that the player have.
    pub decks: Deck<Sigil>,
}
