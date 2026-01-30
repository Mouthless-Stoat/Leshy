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

pub mod inscryption;

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
/// The generic type `Handler` must implement the [`SigilHandler`] type. This `Handler` is
/// responsible to handling how sigil resolve. The handler is called whenever a [`SigilEvent`]
/// happens, it is then given the sigil current being process, the event that trigger this handle
/// and the state of the fight with the ability mutate it. Sigils are activated in order starting
/// with the first column from the active player perspective and in the order they are in on the
/// card then move to the first column from the non-active player perspective. All sigils are
/// process during every [`SigilEvent`].
///
/// Each [`SigilHandler`] handle a specific custom `Sigil` type, usually this is an enum refer to
/// the default [`inscryption`] implement for examples.
#[derive(Clone)]
pub struct FightManager<Sigil: SigilTrait> {
    /// The board use for this fight. This will be mutated if an action affect the board state.
    pub board: Board<Sigil>,
    /// The current state of the scale, positive value indicate the scale is in favor of the first
    /// plyaer and negative mean in favor of the second player.
    pub scale: isize,
    player: (Player<Sigil>, Player<Sigil>),
    active_player: PlayerID,
}

impl<Sigil: SigilTrait> FightManager<Sigil> {
    /// Draw a card from `player` deck. This also trigger the [`SigilEvent::OnDraw`] events for
    /// the drawn card first then the rest of the board after. These all triggers after the
    /// card is drawn (i.e removed from the deck) but before it is added to the hand.
    ///
    /// When the event trigger on the card being drawn, a copy of the card is made instead due to
    /// some rust limitation and then the mutated copy is put back into the hand.
    ///
    /// Will return [`Error::PlayerStarve`] if starvation occur and handle
    /// [`SigilEvent::OnPlayerStarve`] as well.
    pub fn draw(&mut self, player: PlayerID) -> Result<()> {
        let draw_temp = match player {
            PlayerID::First => self.player.0.decks.pop(),
            PlayerID::Second => self.player.1.decks.pop(),
        };
        let Some(drawn_card) = draw_temp else {
            self.handle_sigils(SigilEvent::OnPlayerStarve(player), None);
            return Err(Error::PlayerStarve);
        };
        let mut drawn_card = match player {
            PlayerID::First => {
                self.player.0.draw(drawn_card);
                self.player
                    .0
                    .hand
                    .last_mut()
                    .expect("Oh on this shouldn't happen.\nUnable to obtain drawn card from hand.")
                    .clone()
            }
            PlayerID::Second => {
                self.player.1.draw(drawn_card);
                self.player
                    .1
                    .hand
                    .last_mut()
                    .expect("Oh on this shouldn't happen.\nUnable to obtain drawn card from hand.")
                    .clone()
            }
        };
        self.handle_sigils(SigilEvent::OnDraw, Some(&mut drawn_card));
        std::mem::swap(
            self.player
                .0
                .hand
                .last_mut()
                .expect("Oh no this shouldn't happen.\nUnable to obtain drawn card from hand."),
            &mut drawn_card,
        );
        Ok(())
    }

    /// Handle all the sigils on a `card` with a given `event` and `cause`.
    pub fn handle_sigils(&mut self, event: SigilEvent, cause: Option<&mut Card<Sigil>>) {
        let ctx = Context { event, cause };
        for card in self.activation_order() {
            match card {
                Slot::Card(mut card) => {
                    let mut sigils = std::mem::take(&mut card.sigils);
                    for s in &mut sigils {
                        s.handle_sigil(&mut card, &ctx, self);
                    }
                    card.sigils = sigils
                }
                Slot::Blank => (),
            }
        }
    }

    /// Return the correct sigil activation order.
    ///
    /// Start with the left most card from the current player then the other player left most from
    /// that player perspective.
    pub fn activation_order(&self) -> Vec<Slot<Sigil>> {
        match self.active_player {
            PlayerID::First => self
                .board
                .first
                .iter()
                .chain(self.board.second.iter())
                .cloned()
                .collect(),
            PlayerID::Second => self
                .board
                .second
                .iter()
                .chain(self.board.first.iter())
                .cloned()
                .collect(),
        }
    }
}
