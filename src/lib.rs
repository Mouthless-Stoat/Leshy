//! # Leshy
//!
//! A framework for creating rule engine for Inscryption, it handle everything from combat to
//! sigils handling as well as win condition and card interaction.

mod board;
use board::*;

mod card;
use card::*;

mod sigil;
use sigil::*;

mod player;
use player::*;

mod inscryption;

pub enum Error {
    PlayerStarve,
}

pub type Result<T> = std::result::Result<T, Error>;

/// The manager for a fight, this is the main entry point for most situation to simulate a game.
///
/// The generic type `Handler` must implement the `SigilHandler` type. This `Handler`
/// is responsible to handling how sigil resolve. The handler is called whenever a [`SigilEvent`]
/// happens, it is then given the sigil current being process, the event that trigger this handle
/// and the state of the fight with the ability mutate it. Sigils are activated in order starting
/// with the first columns from the first player perspective and in the order they are in on the
/// card. All sigils are process during every [`SigilEvent`].
///
/// Each [`SigilHandler`] handle a specific custom `Sigil` type, usually this is an enum refer to
/// the default [`inscryption`] implement for examples.
#[derive(Clone)]
pub struct FightManager<Handler: SigilHandler> {
    /// The board use for this fight. This will be mutated if an action affect the board state.
    pub board: Board<Handler::Sigil>,
    /// The current state of the scale, positive value indicate the scale is in favor of the first
    /// plyaer and negative mean in favor of the second player.
    pub scale: isize,
    player: (Player<Handler::Sigil>, Player<Handler::Sigil>),
}

impl<Sigil: Clone> Player<Sigil> {
    pub fn draw(&mut self, card: Card<Sigil>) {
        self.hand.push(card);
    }

    pub fn draw_deck(&mut self) -> Result<()> {
        let card = self.decks.pop().ok_or(Error::PlayerStarve)?;
        self.draw(card);
        Ok(())
    }
}

impl<Handler: SigilHandler> FightManager<Handler> {
    /// Draw a card from `player` deck.
    pub fn draw(&mut self, player: PlayerID) -> Result<()> {
        for card in self.board.p1.clone() {
            match card {
                Slot::Card(mut card) => {
                    for sigil in card.data().sigils() {
                        Handler::handle_sigil(sigil.clone(), SigilEvent::OnDraw, &mut card, self);
                    }
                    let player = match player {
                        PlayerID::First => &mut self.player.0,
                        PlayerID::Second => &mut self.player.1,
                    };
                    let card = player.decks.pop().ok_or(Error::PlayerStarve)?;
                    player.draw(card);
                }
                Slot::Blank => (),
            }
        }
        Ok(())
    }
}
