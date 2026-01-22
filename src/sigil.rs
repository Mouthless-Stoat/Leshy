use crate::{Card, FightManager, PlayerID};

/// Refer to [`FightManager`] documentation to see the responsibility of this trait.
pub trait SigilHandler {
    /// The sigils type for this handler.
    type Sigil: Clone + 'static;
    /// Handle `sigil` due to `event`.
    fn handle_sigil(
        sigil: Self::Sigil,
        ctx: &Context<Self>,
        fight_manager: &mut FightManager<Self>,
    ) where
        Self: Sized;
}

/// Bundling a sigil event as well as related information.
pub struct Context<'a, Handler: SigilHandler> {
    /// The event that is being handle.
    pub event: SigilEvent,
    /// The card that causes this event to trigger.
    pub cause: Option<&'a Card<Handler::Sigil>>,
    /// The card that have this sigil, this can be the same as the cause of the event.
    pub card: &'a Card<Handler::Sigil>,
}

#[allow(clippy::enum_variant_names, missing_docs)]
pub enum SigilEvent {
    OnAttack,
    OnActivate,
    OnDeath,
    OnDraw,
    OnPlay,
    OnSacrifice,
    OnDamage,
    OnTurnEnd,
    OnPlayerStarve(PlayerID),
}
