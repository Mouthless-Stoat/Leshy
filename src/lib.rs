pub struct FightManager<Sigil> {
    pub board: Board,
    pub cards: Vec<Card<Sigil>>,
}

#[derive(Clone, Copy)]
pub enum PlayerID {
    First,
    Second,
}

pub struct Player {
    hand: Vec<CardID>,
}

pub struct Board {
    pub first: Vec<Slot>,
    pub second: Vec<Slot>,
    pub scale: f64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct CardID(isize);

pub struct Slot {
    pub card_id: CardID,
}

pub struct Card<Sigil> {
    pub id: CardID,
    pub name: String,
    pub attack: f64,
    pub health: f64,
    pub sigils: Vec<Sigil>,
}

impl<Sigil> FightManager<Sigil> {
    pub fn get_mut_from_id(&mut self, id: CardID) -> Option<&mut Card<Sigil>> {
        self.cards.iter_mut().find(|card| card.id == id)
    }

    pub fn get_from_id(&mut self, id: CardID) -> Option<&Card<Sigil>> {
        self.cards.iter().find(|card| card.id == id)
    }
}

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

pub trait Sigil {
    fn handle(self, card: CardID, ctx: Context);
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
