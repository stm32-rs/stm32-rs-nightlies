#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    kr: KR,
    pr: PR,
    rlr: RLR,
    sr: SR,
    winr: WINR,
    ewcr: EWCR,
}
impl RegisterBlock {
    #[doc = "0x00 - IWDG key register"]
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    #[doc = "0x04 - IWDG prescaler register"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x08 - IWDG reload register"]
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
    #[doc = "0x0c - IWDG status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x10 - IWDG window register"]
    #[inline(always)]
    pub const fn winr(&self) -> &WINR {
        &self.winr
    }
    #[doc = "0x14 - IWDG early wakeup interrupt register"]
    #[inline(always)]
    pub const fn ewcr(&self) -> &EWCR {
        &self.ewcr
    }
}
#[doc = "KR (w) register accessor: IWDG key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
pub type KR = crate::Reg<kr::KRrs>;
#[doc = "IWDG key register"]
pub mod kr;
#[doc = "PR (rw) register accessor: IWDG prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PRrs>;
#[doc = "IWDG prescaler register"]
pub mod pr;
#[doc = "RLR (rw) register accessor: IWDG reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLRrs>;
#[doc = "IWDG reload register"]
pub mod rlr;
#[doc = "SR (r) register accessor: IWDG status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "IWDG status register"]
pub mod sr;
#[doc = "WINR (rw) register accessor: IWDG window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`winr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`winr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@winr`]
module"]
pub type WINR = crate::Reg<winr::WINRrs>;
#[doc = "IWDG window register"]
pub mod winr;
#[doc = "EWCR (rw) register accessor: IWDG early wakeup interrupt register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ewcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ewcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ewcr`]
module"]
pub type EWCR = crate::Reg<ewcr::EWCRrs>;
#[doc = "IWDG early wakeup interrupt register"]
pub mod ewcr;
