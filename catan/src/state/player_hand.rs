use crate::utils::Resources;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PlayerHand {
    pub resources: Resources,
    pub public_vp: u8,
    pub dvp_carp_vp: u8,
}

impl PlayerHand {
    pub(super) fn new() -> PlayerHand {
        PlayerHand {
            resources: Resources::ZERO,
            public_vp: 0,
            dvp_carp_vp: 0,
        }
    }

    pub fn total_vp(&self) -> u8 {
        return self.public_vp + self.dvp_carp_vp;
    }
}
