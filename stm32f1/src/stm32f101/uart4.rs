#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sr: SR,
    dr: DR,
    brr: BRR,
    cr1: CR1,
    cr2: CR2,
    cr3: CR3,
}
impl RegisterBlock {
    #[doc = "0x00 - UART4_SR"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x04 - UART4_DR"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - UART4_BRR"]
    #[inline(always)]
    pub const fn brr(&self) -> &BRR {
        &self.brr
    }
    #[doc = "0x0c - UART4_CR1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &CR1 {
        &self.cr1
    }
    #[doc = "0x10 - UART4_CR2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &CR2 {
        &self.cr2
    }
    #[doc = "0x14 - UART4_CR3"]
    #[inline(always)]
    pub const fn cr3(&self) -> &CR3 {
        &self.cr3
    }
}
#[doc = "SR (rw) register accessor: UART4_SR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "UART4_SR"]
pub mod sr;
#[doc = "DR (rw) register accessor: UART4_DR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "UART4_DR"]
pub mod dr;
#[doc = "BRR (rw) register accessor: UART4_BRR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`brr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@brr`]
module"]
pub type BRR = crate::Reg<brr::BRRrs>;
#[doc = "UART4_BRR"]
pub mod brr;
#[doc = "CR1 (rw) register accessor: UART4_CR1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`]
module"]
pub type CR1 = crate::Reg<cr1::CR1rs>;
#[doc = "UART4_CR1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: UART4_CR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`]
module"]
pub type CR2 = crate::Reg<cr2::CR2rs>;
#[doc = "UART4_CR2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: UART4_CR3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`]
module"]
pub type CR3 = crate::Reg<cr3::CR3rs>;
#[doc = "UART4_CR3"]
pub mod cr3;
