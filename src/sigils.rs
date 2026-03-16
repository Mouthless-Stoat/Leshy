use crate::{CardID, FightManager, PlayerID};

/// Default Inscrytion Vanilla Sigil
#[derive(Clone, Debug)]
pub enum VanillaSigil {
    BifucatedStrike,
    BoneKing,
    BombLatch,
    Amorphorus,
    RabbitHole,
    DamBuilder,
    BeesWithin,
    LooseTails,
    Sprinter,
    Waterborne,
    Guardian,
    Burrower,
}

pub trait Sigil: Sized {
    fn handle(self, fight_manager: FightManager<Self>, card: CardID, ctx: Context);
}

pub struct Context {
    pub event: SigilEvent,
    pub cause: CardID,
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
