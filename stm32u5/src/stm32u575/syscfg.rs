#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    cfgr1: CFGR1,
    fpuimr: FPUIMR,
    cnslckr: CNSLCKR,
    cslockr: CSLOCKR,
    cfgr2: CFGR2,
    mesr: MESR,
    cccsr: CCCSR,
    ccvr: CCVR,
    cccr: CCCR,
    _reserved10: [u8; 0x04],
    rsscmdr: RSSCMDR,
    _reserved11: [u8; 0x40],
    ucpdr: UCPDR,
}
impl RegisterBlock {
    #[doc = "0x00 - SYSCFG secure configuration register"]
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    #[doc = "0x04 - configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    #[doc = "0x08 - FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(&self) -> &FPUIMR {
        &self.fpuimr
    }
    #[doc = "0x0c - SYSCFG CPU non-secure lock register"]
    #[inline(always)]
    pub const fn cnslckr(&self) -> &CNSLCKR {
        &self.cnslckr
    }
    #[doc = "0x10 - SYSCFG CPU secure lock register"]
    #[inline(always)]
    pub const fn cslockr(&self) -> &CSLOCKR {
        &self.cslockr
    }
    #[doc = "0x14 - configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x18 - memory erase status register"]
    #[inline(always)]
    pub const fn mesr(&self) -> &MESR {
        &self.mesr
    }
    #[doc = "0x1c - compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    #[doc = "0x20 - compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &CCVR {
        &self.ccvr
    }
    #[doc = "0x24 - compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &CCCR {
        &self.cccr
    }
    #[doc = "0x2c - RSS command register"]
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
    }
    #[doc = "0x70 - USB Type C and Power Delivery register"]
    #[inline(always)]
    pub const fn ucpdr(&self) -> &UCPDR {
        &self.ucpdr
    }
}
#[doc = "SECCFGR (rw) register accessor: SYSCFG secure configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`seccfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@seccfgr`]
module"]
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
#[doc = "SYSCFG secure configuration register"]
pub mod seccfgr;
#[doc = "CFGR1 (rw) register accessor: configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`]
module"]
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
#[doc = "configuration register 1"]
pub mod cfgr1;
#[doc = "FPUIMR (rw) register accessor: FPU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpuimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpuimr`]
module"]
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
#[doc = "FPU interrupt mask register"]
pub mod fpuimr;
#[doc = "CNSLCKR (rw) register accessor: SYSCFG CPU non-secure lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnslckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnslckr`]
module"]
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
#[doc = "SYSCFG CPU non-secure lock register"]
pub mod cnslckr;
#[doc = "CSLOCKR (rw) register accessor: SYSCFG CPU secure lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cslockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cslockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cslockr`]
module"]
pub type CSLOCKR = crate::Reg<cslockr::CSLOCKRrs>;
#[doc = "SYSCFG CPU secure lock register"]
pub mod cslockr;
#[doc = "CFGR2 (rw) register accessor: configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "configuration register 2"]
pub mod cfgr2;
#[doc = "MESR (rw) register accessor: memory erase status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mesr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mesr`]
module"]
pub type MESR = crate::Reg<mesr::MESRrs>;
#[doc = "memory erase status register"]
pub mod mesr;
#[doc = "CCCSR (rw) register accessor: compensation cell control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccsr`]
module"]
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR (r) register accessor: compensation cell value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`]
module"]
pub type CCVR = crate::Reg<ccvr::CCVRrs>;
#[doc = "compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR (rw) register accessor: compensation cell code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`]
module"]
pub type CCCR = crate::Reg<cccr::CCCRrs>;
#[doc = "compensation cell code register"]
pub mod cccr;
#[doc = "RSSCMDR (rw) register accessor: RSS command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsscmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsscmdr`]
module"]
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
#[doc = "RSS command register"]
pub mod rsscmdr;
#[doc = "UCPDR (rw) register accessor: USB Type C and Power Delivery register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucpdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ucpdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucpdr`]
module"]
pub type UCPDR = crate::Reg<ucpdr::UCPDRrs>;
#[doc = "USB Type C and Power Delivery register"]
pub mod ucpdr;
