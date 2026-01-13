mod inscryption;

#[derive(Clone)]
pub struct CardData<Sigil: Clone> {
    power: isize,
    health: isize,
    sigils: Vec<Sigil>,
}

#[derive(Clone)]
pub struct Card<Sigil: Clone> {
    pow_mod: isize,
    health_mod: isize,
    data: CardData<Sigil>,
}

#[derive(Clone)]
pub enum Slot<Sigil: Clone> {
    Blank,
    Card(Card<Sigil>),
}

#[derive(Clone)]
pub struct Board<Sigil: Clone> {
    pub p1: Vec<Slot<Sigil>>,
    pub p2: Vec<Slot<Sigil>>,
}

impl<Sigil: Clone> Board<Sigil> {
    pub fn new() -> Self {
        Board {
            p1: vec![Slot::Blank; 4],
            p2: vec![Slot::Blank; 4],
        }
    }
}

impl<Sigil: Clone> Default for Board<Sigil> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct FightManager<Handler: SigilHandler> {
    pub board: Board<Handler::Sigil>,
    pub scale: isize,
    handler: Handler,
}

pub enum PlayerID {
    First,
    Second,
}

pub struct Player<Sigil: Clone> {
    pub hand: Vec<CardData<Sigil>>,
    pub deck: Vec<CardData<Sigil>>,
}

impl<Handler: SigilHandler> FightManager<Handler> {
    pub fn draw(player: PlayerID) {}
}

pub trait SigilHandler {
    type Sigil: Clone;
    fn handle_sigil(sigil: Self::Sigil, fight_manager: &FightManager<Self>)
    where
        Self: Sized;
}

#[allow(clippy::enum_variant_names)]
pub enum SigilEvent {
    OnPlay,
    OnAttack,
    OnDamage,
}
