use crate::{Card, FightManager, PlayerID};

/// Refer to [`FightManager`] documentation to see the responsibility of this trait.
pub trait SigilTrait: Clone + 'static {
    /// Handle `sigil` due to `event`.
    fn handle_sigil(
        &mut self,
        card: &mut Card<Self>,
        ctx: &Context<Self>,
        fight_manager: &mut FightManager<Self>,
    ) where
        Self: Sized;
}

/// Bundling a sigil event as well as related information.
pub struct Context<'a, Sigil: SigilTrait> {
    /// The event that is being handle.
    pub event: SigilEvent,
    /// The card that causes this event to trigger.
    pub cause: Option<&'a mut Card<Sigil>>,
}

#[derive(Clone)]
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
