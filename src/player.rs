use crate::{Card, Error, Result};

/// Player ID for the first and second player
#[derive(Clone, Copy, Default)]
#[allow(missing_docs)]
pub enum PlayerID {
    #[default]
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

impl<Sigil: Clone> Player<Sigil> {
    /// Add `card` into the player hand. This does not trigger or handle event, use
    /// [`FightManager::draw`] instead
    ///
    /// To draw a card from the player's deck use [`draw_deck`](Player::draw_deck)
    /// instead.
    pub fn draw(&mut self, card: Card<Sigil>) {
        self.hand.push(card);
    }

    /// Draw a card from the player deck. To add a card into the player hand use
    /// [`draw`](Player::draw) instead.
    pub fn draw_deck(&mut self) -> Result<()> {
        let card = self.decks.pop().ok_or(Error::PlayerStarve)?;
        self.draw(card);
        Ok(())
    }
}
