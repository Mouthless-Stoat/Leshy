mod sigils;
pub use sigils::*; // Re-export all of sigils stuff

mod cards;
pub use cards::*;

pub struct FightManager<S: Sigil> {
    pub board: Board,
    pub cards: Vec<Card<S>>,
    pub active_player: PlayerID,
    pub players: (Player<S>, Player<S>),
}

#[derive(Clone, Copy)]
pub enum PlayerID {
    First,
    Second,
}

pub struct Player<S: Sigil> {
    hand: Vec<CardID>,
    deck: Vec<CardData<S>>,
}

pub struct Board {
    pub first: Vec<Slot>,
    pub second: Vec<Slot>,
    pub scale: f64,
}

pub struct Slot {
    pub card_id: CardID,
}

impl<S: Sigil> FightManager<S> {
    /// Get the card object from a given ID, this will be mutable.
    pub fn get_mut_from_id(&mut self, id: CardID) -> Option<&mut Card<S>> {
        self.cards.iter_mut().find(|card| card.id == id)
    }

    /// Get the card object from a given ID
    pub fn get_from_id(&mut self, id: CardID) -> Option<&Card<S>> {
        self.cards.iter().find(|card| card.id == id)
    }

    pub fn draw_card(player: PlayerID) {}

    pub fn activation_order(self) -> Vec<CardID> {
        match self.active_player {
            PlayerID::First => self
                .players
                .0
                .hand
                .iter()
                .cloned()
                .chain(self.players.1.hand.clone())
                .chain(self.board.first.iter().map(|s| s.card_id).clone())
                .collect(),
            PlayerID::Second => self
                .players
                .1
                .hand
                .iter()
                .cloned()
                .chain(self.players.0.hand.clone())
                .chain(self.board.second.iter().map(|s| s.card_id).clone())
                .collect(),
        }
    }
}
