#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    seccfgr: SECCFGR,
    cfgr1: CFGR1,
    fpuimr: FPUIMR,
    cnslckr: CNSLCKR,
    cslockr: CSLOCKR,
    cfgr2: CFGR2,
    scsr: SCSR,
    skr: SKR,
    swpr: SWPR,
    swpr2: SWPR2,
    _reserved10: [u8; 0x04],
    rsscmdr: RSSCMDR,
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
    #[doc = "0x14 - CFGR2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x18 - SCSR"]
    #[inline(always)]
    pub const fn scsr(&self) -> &SCSR {
        &self.scsr
    }
    #[doc = "0x1c - SKR"]
    #[inline(always)]
    pub const fn skr(&self) -> &SKR {
        &self.skr
    }
    #[doc = "0x20 - SWPR"]
    #[inline(always)]
    pub const fn swpr(&self) -> &SWPR {
        &self.swpr
    }
    #[doc = "0x24 - SWPR2"]
    #[inline(always)]
    pub const fn swpr2(&self) -> &SWPR2 {
        &self.swpr2
    }
    #[doc = "0x2c - RSSCMDR"]
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
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
#[doc = "SCSR (rw) register accessor: SCSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scsr`]
module"]
pub type SCSR = crate::Reg<scsr::SCSRrs>;
#[doc = "SCSR"]
pub mod scsr;
#[doc = "CFGR2 (rw) register accessor: CFGR2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "CFGR2"]
pub mod cfgr2;
#[doc = "SWPR (w) register accessor: SWPR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr`]
module"]
pub type SWPR = crate::Reg<swpr::SWPRrs>;
#[doc = "SWPR"]
pub mod swpr;
#[doc = "SKR (w) register accessor: SKR\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`skr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@skr`]
module"]
pub type SKR = crate::Reg<skr::SKRrs>;
#[doc = "SKR"]
pub mod skr;
#[doc = "SWPR2 (w) register accessor: SWPR2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swpr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swpr2`]
module"]
pub type SWPR2 = crate::Reg<swpr2::SWPR2rs>;
#[doc = "SWPR2"]
pub mod swpr2;
#[doc = "RSSCMDR (rw) register accessor: RSSCMDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsscmdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsscmdr`]
module"]
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
#[doc = "RSSCMDR"]
pub mod rsscmdr;
