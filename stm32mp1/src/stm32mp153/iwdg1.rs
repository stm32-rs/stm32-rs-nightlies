#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    iwdg_kr: IWDG_KR,
    iwdg_pr: IWDG_PR,
    iwdg_rlr: IWDG_RLR,
    iwdg_sr: IWDG_SR,
    iwdg_winr: IWDG_WINR,
    _reserved5: [u8; 0x03dc],
    iwdg_hwcfgr: IWDG_HWCFGR,
    iwdg_verr: IWDG_VERR,
    iwdg_idr: IWDG_IDR,
    iwdg_sidr: IWDG_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Key register"]
    #[inline(always)]
    pub const fn iwdg_kr(&self) -> &IWDG_KR {
        &self.iwdg_kr
    }
    #[doc = "0x04 - Prescaler register"]
    #[inline(always)]
    pub const fn iwdg_pr(&self) -> &IWDG_PR {
        &self.iwdg_pr
    }
    #[doc = "0x08 - Reload register"]
    #[inline(always)]
    pub const fn iwdg_rlr(&self) -> &IWDG_RLR {
        &self.iwdg_rlr
    }
    #[doc = "0x0c - Status register"]
    #[inline(always)]
    pub const fn iwdg_sr(&self) -> &IWDG_SR {
        &self.iwdg_sr
    }
    #[doc = "0x10 - Window register"]
    #[inline(always)]
    pub const fn iwdg_winr(&self) -> &IWDG_WINR {
        &self.iwdg_winr
    }
    #[doc = "0x3f0 - IWDG hardware configuration register"]
    #[inline(always)]
    pub const fn iwdg_hwcfgr(&self) -> &IWDG_HWCFGR {
        &self.iwdg_hwcfgr
    }
    #[doc = "0x3f4 - IWDG version register"]
    #[inline(always)]
    pub const fn iwdg_verr(&self) -> &IWDG_VERR {
        &self.iwdg_verr
    }
    #[doc = "0x3f8 - IWDG identification register"]
    #[inline(always)]
    pub const fn iwdg_idr(&self) -> &IWDG_IDR {
        &self.iwdg_idr
    }
    #[doc = "0x3fc - IWDG size identification register"]
    #[inline(always)]
    pub const fn iwdg_sidr(&self) -> &IWDG_SIDR {
        &self.iwdg_sidr
    }
}
#[doc = "IWDG_KR (w) register accessor: Key register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_kr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_kr`]
module"]
pub type IWDG_KR = crate::Reg<iwdg_kr::IWDG_KRrs>;
#[doc = "Key register"]
pub mod iwdg_kr;
#[doc = "IWDG_PR (rw) register accessor: Prescaler register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_pr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_pr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_pr`]
module"]
pub type IWDG_PR = crate::Reg<iwdg_pr::IWDG_PRrs>;
#[doc = "Prescaler register"]
pub mod iwdg_pr;
#[doc = "IWDG_RLR (rw) register accessor: Reload register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_rlr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_rlr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_rlr`]
module"]
pub type IWDG_RLR = crate::Reg<iwdg_rlr::IWDG_RLRrs>;
#[doc = "Reload register"]
pub mod iwdg_rlr;
#[doc = "IWDG_SR (r) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sr`]
module"]
pub type IWDG_SR = crate::Reg<iwdg_sr::IWDG_SRrs>;
#[doc = "Status register"]
pub mod iwdg_sr;
#[doc = "IWDG_WINR (rw) register accessor: Window register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_winr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iwdg_winr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_winr`]
module"]
pub type IWDG_WINR = crate::Reg<iwdg_winr::IWDG_WINRrs>;
#[doc = "Window register"]
pub mod iwdg_winr;
#[doc = "IWDG_HWCFGR (r) register accessor: IWDG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_hwcfgr`]
module"]
pub type IWDG_HWCFGR = crate::Reg<iwdg_hwcfgr::IWDG_HWCFGRrs>;
#[doc = "IWDG hardware configuration register"]
pub mod iwdg_hwcfgr;
#[doc = "IWDG_VERR (r) register accessor: IWDG version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_verr`]
module"]
pub type IWDG_VERR = crate::Reg<iwdg_verr::IWDG_VERRrs>;
#[doc = "IWDG version register"]
pub mod iwdg_verr;
#[doc = "IWDG_IDR (r) register accessor: IWDG identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_idr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_idr`]
module"]
pub type IWDG_IDR = crate::Reg<iwdg_idr::IWDG_IDRrs>;
#[doc = "IWDG identification register"]
pub mod iwdg_idr;
#[doc = "IWDG_SIDR (r) register accessor: IWDG size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iwdg_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iwdg_sidr`]
module"]
pub type IWDG_SIDR = crate::Reg<iwdg_sidr::IWDG_SIDRrs>;
#[doc = "IWDG size identification register"]
pub mod iwdg_sidr;
