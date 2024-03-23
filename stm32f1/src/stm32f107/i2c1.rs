#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: CR1,
    cr2: CR2,
    oar1: OAR1,
    oar2: OAR2,
    dr: DR,
    sr1: SR1,
    sr2: SR2,
    ccr: CCR,
    trise: TRISE,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x04 - Control register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x08 - Own address register 1"]
    #[inline(always)]
    pub const fn oar1(&self) -> &OAR1 {
        &self.oar1
    }
    #[doc = "0x0c - Own address register 2"]
    #[inline(always)]
    pub const fn oar2(&self) -> &OAR2 {
        &self.oar2
    }
    #[doc = "0x10 - Data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x14 - Status register 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &SR1 {
        &self.sr1
    }
    #[doc = "0x18 - Status register 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &SR2 {
        &self.sr2
    }
    #[doc = "0x1c - Clock control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &CCR {
        &self.ccr
    }
    #[doc = "0x20 - TRISE register"]
    #[inline(always)]
    pub const fn trise(&self) -> &TRISE {
        &self.trise
    }
}
#[doc = "CR1 (rw) register accessor: Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "Control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "Control register 2"]
pub mod cr2;
#[doc = "OAR1 (rw) register accessor: Own address register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar1`]
module"]
pub type OAR1 = crate::Reg<oar1::OAR1rs>;
#[doc = "Own address register 1"]
pub mod oar1;
#[doc = "OAR2 (rw) register accessor: Own address register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oar2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oar2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oar2`]
module"]
pub type OAR2 = crate::Reg<oar2::OAR2rs>;
#[doc = "Own address register 2"]
pub mod oar2;
#[doc = "DR (rw) register accessor: Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "Data register"]
pub mod dr;
#[doc = "SR1 (rw) register accessor: Status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`]
module"]
pub type SR1 = crate::Reg<sr1::SR1rs>;
#[doc = "Status register 1"]
pub mod sr1;
#[doc = "SR2 (r) register accessor: Status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`]
module"]
pub type SR2 = crate::Reg<sr2::SR2rs>;
#[doc = "Status register 2"]
pub mod sr2;
#[doc = "CCR (rw) register accessor: Clock control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`]
module"]
pub type CCR = crate::Reg<ccr::CCRrs>;
#[doc = "Clock control register"]
pub mod ccr;
#[doc = "TRISE (rw) register accessor: TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trise::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trise::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trise`]
module"]
pub type TRISE = crate::Reg<trise::TRISErs>;
#[doc = "TRISE register"]
pub mod trise;
