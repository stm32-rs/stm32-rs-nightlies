#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c1cr: C1CR,
    c1mr: C1MR,
    c1scr: C1SCR,
    c1toc2sr: C1TOC2SR,
    c2cr: C2CR,
    c2mr: C2MR,
    c2scr: C2SCR,
    c2toc1sr: C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register CPU1"]
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    #[doc = "0x04 - Mask register CPU1"]
    #[inline(always)]
    pub const fn c1mr(&self) -> &C1MR {
        &self.c1mr
    }
    #[doc = "0x08 - Status Set or Clear register CPU1"]
    #[inline(always)]
    pub const fn c1scr(&self) -> &C1SCR {
        &self.c1scr
    }
    #[doc = "0x0c - CPU1 to CPU2 status register"]
    #[inline(always)]
    pub const fn c1toc2sr(&self) -> &C1TOC2SR {
        &self.c1toc2sr
    }
    #[doc = "0x10 - Control register CPU2"]
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    #[doc = "0x14 - Mask register CPU2"]
    #[inline(always)]
    pub const fn c2mr(&self) -> &C2MR {
        &self.c2mr
    }
    #[doc = "0x18 - Status Set or Clear register CPU2"]
    #[inline(always)]
    pub const fn c2scr(&self) -> &C2SCR {
        &self.c2scr
    }
    #[doc = "0x1c - CPU2 to CPU1 status register"]
    #[inline(always)]
    pub const fn c2toc1sr(&self) -> &C2TOC1SR {
        &self.c2toc1sr
    }
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    #[doc = "0x3f4 - IPCC version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - IPCC indentification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - IPCC size indentification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "C1CR (rw) register accessor: Control register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1cr`]
module"]
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
#[doc = "Control register CPU1"]
pub mod c1cr;
#[doc = "C1MR (rw) register accessor: Mask register CPU1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1mr`]
module"]
pub type C1MR = crate::Reg<c1mr::C1MRrs>;
#[doc = "Mask register CPU1"]
pub mod c1mr;
#[doc = "C1SCR (w) register accessor: Status Set or Clear register CPU1\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1scr`]
module"]
pub type C1SCR = crate::Reg<c1scr::C1SCRrs>;
#[doc = "Status Set or Clear register CPU1"]
pub mod c1scr;
#[doc = "C1TOC2SR (r) register accessor: CPU1 to CPU2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1toc2sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1toc2sr`]
module"]
pub type C1TOC2SR = crate::Reg<c1toc2sr::C1TOC2SRrs>;
#[doc = "CPU1 to CPU2 status register"]
pub mod c1toc2sr;
#[doc = "C2CR (rw) register accessor: Control register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2cr`]
module"]
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
#[doc = "Control register CPU2"]
pub mod c2cr;
#[doc = "C2MR (rw) register accessor: Mask register CPU2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2mr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2mr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2mr`]
module"]
pub type C2MR = crate::Reg<c2mr::C2MRrs>;
#[doc = "Mask register CPU2"]
pub mod c2mr;
#[doc = "C2SCR (w) register accessor: Status Set or Clear register CPU2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2scr`]
module"]
pub type C2SCR = crate::Reg<c2scr::C2SCRrs>;
#[doc = "Status Set or Clear register CPU2"]
pub mod c2scr;
#[doc = "C2TOC1SR (r) register accessor: CPU2 to CPU1 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2toc1sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2toc1sr`]
module"]
pub type C2TOC1SR = crate::Reg<c2toc1sr::C2TOC1SRrs>;
#[doc = "CPU2 to CPU1 status register"]
pub mod c2toc1sr;
#[doc = "HWCFGR (r) register accessor: IPCC Hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr`]
module"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
#[doc = "IPCC Hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: IPCC version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "IPCC version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: IPCC indentification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "IPCC indentification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: IPCC size indentification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "IPCC size indentification register"]
pub mod sidr;
