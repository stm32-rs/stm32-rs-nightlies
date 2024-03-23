#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    wwdg_cr: WWDG_CR,
    wwdg_cfr: WWDG_CFR,
    wwdg_sr: WWDG_SR,
    _reserved3: [u8; 0x03e4],
    wwdg_hwcfgr: WWDG_HWCFGR,
    wwdg_verr: WWDG_VERR,
    wwdg_ipidr: WWDG_IPIDR,
    wwdg_sidr: WWDG_SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn wwdg_cr(&self) -> &WWDG_CR {
        &self.wwdg_cr
    }
    #[doc = "0x04 - Configuration register"]
    #[inline(always)]
    pub const fn wwdg_cfr(&self) -> &WWDG_CFR {
        &self.wwdg_cfr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn wwdg_sr(&self) -> &WWDG_SR {
        &self.wwdg_sr
    }
    #[doc = "0x3f0 - WWDG hardware configuration register"]
    #[inline(always)]
    pub const fn wwdg_hwcfgr(&self) -> &WWDG_HWCFGR {
        &self.wwdg_hwcfgr
    }
    #[doc = "0x3f4 - WWDG version register"]
    #[inline(always)]
    pub const fn wwdg_verr(&self) -> &WWDG_VERR {
        &self.wwdg_verr
    }
    #[doc = "0x3f8 - WWDG ID register"]
    #[inline(always)]
    pub const fn wwdg_ipidr(&self) -> &WWDG_IPIDR {
        &self.wwdg_ipidr
    }
    #[doc = "0x3fc - WWDG size ID register"]
    #[inline(always)]
    pub const fn wwdg_sidr(&self) -> &WWDG_SIDR {
        &self.wwdg_sidr
    }
}
#[doc = "WWDG_CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cr`]
module"]
pub type WWDG_CR = crate::Reg<wwdg_cr::WWDG_CRrs>;
#[doc = "Control register"]
pub mod wwdg_cr;
#[doc = "WWDG_CFR (rw) register accessor: Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_cfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_cfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_cfr`]
module"]
pub type WWDG_CFR = crate::Reg<wwdg_cfr::WWDG_CFRrs>;
#[doc = "Configuration register"]
pub mod wwdg_cfr;
#[doc = "WWDG_SR (rw) register accessor: Status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_sr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wwdg_sr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_sr`]
module"]
pub type WWDG_SR = crate::Reg<wwdg_sr::WWDG_SRrs>;
#[doc = "Status register"]
pub mod wwdg_sr;
#[doc = "WWDG_HWCFGR (r) register accessor: WWDG hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_hwcfgr`]
module"]
pub type WWDG_HWCFGR = crate::Reg<wwdg_hwcfgr::WWDG_HWCFGRrs>;
#[doc = "WWDG hardware configuration register"]
pub mod wwdg_hwcfgr;
#[doc = "WWDG_VERR (r) register accessor: WWDG version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_verr`]
module"]
pub type WWDG_VERR = crate::Reg<wwdg_verr::WWDG_VERRrs>;
#[doc = "WWDG version register"]
pub mod wwdg_verr;
#[doc = "WWDG_IPIDR (r) register accessor: WWDG ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_ipidr`]
module"]
pub type WWDG_IPIDR = crate::Reg<wwdg_ipidr::WWDG_IPIDRrs>;
#[doc = "WWDG ID register"]
pub mod wwdg_ipidr;
#[doc = "WWDG_SIDR (r) register accessor: WWDG size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wwdg_sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wwdg_sidr`]
module"]
pub type WWDG_SIDR = crate::Reg<wwdg_sidr::WWDG_SIDRrs>;
#[doc = "WWDG size ID register"]
pub mod wwdg_sidr;
