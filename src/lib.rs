pub struct FightManager<S: Sigil> {
    pub board: Board,
    pub cards: Vec<CardData<S>>,
    pub active_player: PlayerID,
    pub players: (Player<S>, Player<S>),
}

/// A bundle of a raw card with its ID and wherever if the card is on the board. The FightManager
/// uses this to handle one master copy of a card that other function borrow from.
pub struct CardData<S: Sigil> {
    id: CardID,
    card: Card<S>,
    on_board: bool,
}

#[derive(Clone, Copy)]
pub enum PlayerID {
    First,
    Second,
}

pub struct Player<S: Sigil> {
    hand: Vec<CardID>,
    deck: Vec<Card<S>>,
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

/// The raw card data that store all the information that a card might have.
pub struct Card<S: Sigil> {
    pub name: String,
    pub attack: f64,
    pub health: f64,
    pub sigils: Vec<S>,
}

impl<S: Sigil> FightManager<S> {
    pub fn get_mut_from_id(&mut self, id: CardID) -> Option<&mut Card<S>> {
        self.cards
            .iter_mut()
            .find(|card| card.id == id)
            .map(|cd| &mut cd.card)
    }

    pub fn get_from_id(&mut self, id: CardID) -> Option<&Card<S>> {
        self.cards
            .iter()
            .find(|card| card.id == id)
            .map(|cd| &cd.card)
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
                .collect(),
            PlayerID::Second => self
                .players
                .1
                .hand
                .iter()
                .cloned()
                .chain(self.players.0.hand.clone())
                .collect(),
        }
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
