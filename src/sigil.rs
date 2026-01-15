use crate::{Card, FightManager};

/// Refer to [`FightManager`] documentation to see the responsibility of this trait.
pub trait SigilHandler {
    /// The sigils type for this handler.
    type Sigil: Clone + 'static;
    /// Handle `sigil` due to `event`.
    fn handle_sigil(sigil: Self::Sigil, ctx: &Context<Self>)
    where
        Self: Sized;
}

/// Bundling a sigil event as well as related information.
pub struct Context<'a, Handler: SigilHandler> {
    pub event: SigilEvent,
    pub cause: &'a Card<Handler::Sigil>,
    pub card: &'a Card<Handler::Sigil>,
    pub fight_manager: &'a mut FightManager<Handler>,
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
    OnPlayerStarve,
}
