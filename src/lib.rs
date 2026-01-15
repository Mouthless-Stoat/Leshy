//! # Leshy
//!
//! A framework for creating rule engine for Inscryption, it handle everything from combat to
//! sigils handling as well as win condition and card interaction.
//!
//! The main type is the [`FightManager`], this handle everything to do with a fight, from board to
//! player and sigil event. The method on the manager is the only function that handle sigil events
//! automatically, if one were to call another method like [`Player::draw`] this would not
//! automatically handle the [`SigilEvent::OnDraw`] event.

mod board;
use board::*;

mod card;
pub use card::*;

mod sigil;
pub use sigil::*;

mod player;
pub use player::*;

mod inscryption;

/// Common result type return between library function and method.
///
/// Return of function are usually just [`Result<T>`] instead of the usual `Result<T, Error>`
/// because their error type are all [`Error`] to make the syntax simpler.
pub type Result<T> = std::result::Result<T, Error>;

/// Common error type wrapped in the custom [`Result`] type
pub enum Error {
    /// Indicate a draw attempt from an empty deck by a player.
    PlayerStarve,
}

/// The manager for a fight, this is the main entry point for most situation to simulate a game.
/// Method on the manager will automatically handle sigil event as well as any propagation of these
/// event.
///
/// The generic type `Handler` must implement the [`SigilHandler`] type. This `Handler`
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
    /// Add `card` into the player hand. To draw a card from the player's deck use
    /// [`draw_deck`](Player::draw_deck) instead.
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
    /// Draw a card from `player` deck. This also trigger the [`SigilEvent::OnDraw`] events after
    /// the card is drawn but before being added to the hand.
    ///
    /// Will return [`Error::PlayerStarve`] if stravation occur and handle
    /// [`SigilEvent::OnPlayerStarve`] as well
    pub fn draw(&mut self, player: PlayerID) -> Result<()> {
        for card in self.board.p1.clone() {
            match card {
                Slot::Card(card) => {
                    let Some(drawn_card) = (match player {
                        PlayerID::First => self.player.0.decks.pop(),
                        PlayerID::Second => self.player.1.decks.pop(),
                    }) else {
                        return Err(Error::PlayerStarve);
                    };

                    let ctx = Context {
                        event: SigilEvent::OnDraw,
                        cause: &drawn_card,
                        card: &card,
                        fight_manager: self,
                    };
                    for sigil in card.data().sigils() {
                        Handler::handle_sigil(sigil.clone(), &ctx);
                    }
                    match player {
                        PlayerID::First => self.player.0.draw(drawn_card),
                        PlayerID::Second => self.player.1.draw(drawn_card),
                    }
                }
                Slot::Blank => (),
            }
        }
        Ok(())
    }
}
