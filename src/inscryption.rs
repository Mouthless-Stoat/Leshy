use crate::{FightManager, SigilHandler};

#[derive(Clone, Debug)]
enum Sigil {
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

struct Handler;
impl SigilHandler for Handler {
    type Sigil = Sigil;

    fn handle_sigil(sigil: Self::Sigil, fight_manager: &FightManager<Self>) {
        match sigil {
            Sigil::BifucatedStrike => todo!(),
            _ => todo!("This sigil ({sigil:?}) is not implemented yet"),
        }
    }
}
