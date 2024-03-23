#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    kr: KR,
    pr: PR,
    rlr: RLR,
    sr: SR,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register (IWDG_KR)"]
    #[inline(always)]
    pub const fn kr(&self) -> &KR {
        &self.kr
    }
    #[doc = "0x04 - Prescaler register (IWDG_PR)"]
    #[inline(always)]
    pub const fn pr(&self) -> &PR {
        &self.pr
    }
    #[doc = "0x08 - Reload register (IWDG_RLR)"]
    #[inline(always)]
    pub const fn rlr(&self) -> &RLR {
        &self.rlr
    }
    #[doc = "0x0c - Status register (IWDG_SR)"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
}
#[doc = "KR (w) register accessor: Key register (IWDG_KR)\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@kr`]
module"]
pub type KR = crate::Reg<kr::KRrs>;
#[doc = "Key register (IWDG_KR)"]
pub mod kr;
#[doc = "PR (rw) register accessor: Prescaler register (IWDG_PR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`]
module"]
pub type PR = crate::Reg<pr::PRrs>;
#[doc = "Prescaler register (IWDG_PR)"]
pub mod pr;
#[doc = "RLR (rw) register accessor: Reload register (IWDG_RLR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLRrs>;
#[doc = "Reload register (IWDG_RLR)"]
pub mod rlr;
#[doc = "SR (r) register accessor: Status register (IWDG_SR)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "Status register (IWDG_SR)"]
pub mod sr;
