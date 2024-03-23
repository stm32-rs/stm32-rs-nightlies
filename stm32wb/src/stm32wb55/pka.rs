#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: CR,
    sr: SR,
    clrfr: CLRFR,
    _reserved3: [u8; 0x1fe8],
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x04 - PKA status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x08 - PKA clear flag register"]
    #[inline(always)]
    pub const fn clrfr(&self) -> &CLRFR {
        &self.clrfr
    }
    #[doc = "0x1ff4 - PKA version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x1ff8 - PKA identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x1ffc - PKA size ID register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Control register"]
pub mod cr;
#[doc = "SR (r) register accessor: PKA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "PKA status register"]
pub mod sr;
#[doc = "CLRFR (rw) register accessor: PKA clear flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clrfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clrfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrfr`]
module"]
pub type CLRFR = crate::Reg<clrfr::CLRFRrs>;
#[doc = "PKA clear flag register"]
pub mod clrfr;
#[doc = "VERR (r) register accessor: PKA version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "PKA version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: PKA identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "PKA identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: PKA size ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "PKA size ID register"]
pub mod sidr;
