#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    dr: DR,
    _reserved3: [u8; 0x04],
    htcr: HTCR,
}
impl RegisterBlock {
    #[doc = "0x00 - RNG control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - RNG status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x10 - health test control register"]
    #[inline(always)]
    pub const fn htcr(&self) -> &HTCR {
        &self.htcr
    }
}
#[doc = "CR (rw) register accessor: RNG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "RNG control register"]
pub mod cr;
#[doc = "SR (rw) register accessor: RNG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "RNG status register"]
pub mod sr;
#[doc = "DR (r) register accessor: The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "The RNG_DR register is a read-only register that delivers a 32-bit random value when read. The content of this register is valid when DRDY= 1, even if RNGEN=0."]
pub mod dr;
#[doc = "HTCR (rw) register accessor: health test control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htcr`]
module"]
pub type HTCR = crate::Reg<htcr::HTCRrs>;
#[doc = "health test control register"]
pub mod htcr;
