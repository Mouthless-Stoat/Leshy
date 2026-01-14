/// Refer to [`Board`] documentation to see the responsibility of this trait.
pub trait SigilHandler {
    /// The sigils type for this handler.
    type Sigil: Clone + 'static;
    /// Handle `sigil` due to `event`.
    fn handle_sigil(
        sigil: Self::Sigil,
        event: SigilEvent,
        card: &mut Card<Self::Sigil>,
        fight_manager: &mut FightManager<Self>,
    ) where
        Self: Sized;
}

#[allow(clippy::enum_variant_names, missing_docs)]
pub enum SigilEvent {
    OnAttack,
    OnActivate,
    OnDeath,
    OnDraw,
    OnSelfDraw,
    OnPlay,
    OnPlayerCardDeath,
    OnSacrifice,
    OnDamage,
    OnTurnEnd,
    OnOpponentAttack,
    OnOpponentPlay,
}
