#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    hdplcr: HDPLCR,
    hdplsr: HDPLSR,
    _reserved2: [u8; 0x08],
    dbgcr: DBGCR,
    dbglockr: DBGLOCKR,
    _reserved4: [u8; 0xd8],
    pmcr: PMCR,
    fpuimr: FPUIMR,
    mesr: MESR,
    _reserved7: [u8; 0x04],
    cccsr: CCCSR,
    ccvalr: CCVALR,
    ccswcr: CCSWCR,
    _reserved10: [u8; 0x04],
    cfgr2: CFGR2,
    _reserved11: [u8; 0x20],
    cnslckr: CNSLCKR,
    _reserved12: [u8; 0x04],
    eccnmir: ECCNMIR,
}
impl RegisterBlock {
    #[doc = "0x10 - SBS temporal isolation control register"]
    #[inline(always)]
    pub const fn hdplcr(&self) -> &HDPLCR {
        &self.hdplcr
    }
    #[doc = "0x14 - SBS temporal isolation status register"]
    #[inline(always)]
    pub const fn hdplsr(&self) -> &HDPLSR {
        &self.hdplsr
    }
    #[doc = "0x20 - SBS debug control register"]
    #[inline(always)]
    pub const fn dbgcr(&self) -> &DBGCR {
        &self.dbgcr
    }
    #[doc = "0x24 - SBS debug lock register"]
    #[inline(always)]
    pub const fn dbglockr(&self) -> &DBGLOCKR {
        &self.dbglockr
    }
    #[doc = "0x100 - SBS product mode and configuration register"]
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    #[doc = "0x104 - SBS FPU interrupt mask register"]
    #[inline(always)]
    pub const fn fpuimr(&self) -> &FPUIMR {
        &self.fpuimr
    }
    #[doc = "0x108 - SBS memory erase status register"]
    #[inline(always)]
    pub const fn mesr(&self) -> &MESR {
        &self.mesr
    }
    #[doc = "0x110 - SBS compensation cell for I/Os control and status register"]
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    #[doc = "0x114 - SBS compensation cell for I/Os value register"]
    #[inline(always)]
    pub const fn ccvalr(&self) -> &CCVALR {
        &self.ccvalr
    }
    #[doc = "0x118 - SBS compensation cell for I/Os software code register"]
    #[inline(always)]
    pub const fn ccswcr(&self) -> &CCSWCR {
        &self.ccswcr
    }
    #[doc = "0x120 - SBS Class B register"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x144 - SBS CPU lock register"]
    #[inline(always)]
    pub const fn cnslckr(&self) -> &CNSLCKR {
        &self.cnslckr
    }
    #[doc = "0x14c - SBS flift ECC NMI mask register"]
    #[inline(always)]
    pub const fn eccnmir(&self) -> &ECCNMIR {
        &self.eccnmir
    }
}
#[doc = "HDPLCR (rw) register accessor: SBS temporal isolation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdplcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hdplcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdplcr`]
module"]
pub type HDPLCR = crate::Reg<hdplcr::HDPLCRrs>;
#[doc = "SBS temporal isolation control register"]
pub mod hdplcr;
#[doc = "HDPLSR (r) register accessor: SBS temporal isolation status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hdplsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hdplsr`]
module"]
pub type HDPLSR = crate::Reg<hdplsr::HDPLSRrs>;
#[doc = "SBS temporal isolation status register"]
pub mod hdplsr;
#[doc = "DBGCR (rw) register accessor: SBS debug control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbgcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbgcr`]
module"]
pub type DBGCR = crate::Reg<dbgcr::DBGCRrs>;
#[doc = "SBS debug control register"]
pub mod dbgcr;
#[doc = "DBGLOCKR (rw) register accessor: SBS debug lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbglockr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbglockr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dbglockr`]
module"]
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKRrs>;
#[doc = "SBS debug lock register"]
pub mod dbglockr;
#[doc = "PMCR (rw) register accessor: SBS product mode and configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`]
module"]
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
#[doc = "SBS product mode and configuration register"]
pub mod pmcr;
#[doc = "FPUIMR (rw) register accessor: SBS FPU interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fpuimr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fpuimr`]
module"]
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
#[doc = "SBS FPU interrupt mask register"]
pub mod fpuimr;
#[doc = "MESR (rw) register accessor: SBS memory erase status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mesr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mesr`]
module"]
pub type MESR = crate::Reg<mesr::MESRrs>;
#[doc = "SBS memory erase status register"]
pub mod mesr;
#[doc = "CCCSR (rw) register accessor: SBS compensation cell for I/Os control and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cccsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccsr`]
module"]
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
#[doc = "SBS compensation cell for I/Os control and status register"]
pub mod cccsr;
#[doc = "CCVALR (r) register accessor: SBS compensation cell for I/Os value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccvalr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvalr`]
module"]
pub type CCVALR = crate::Reg<ccvalr::CCVALRrs>;
#[doc = "SBS compensation cell for I/Os value register"]
pub mod ccvalr;
#[doc = "CCSWCR (rw) register accessor: SBS compensation cell for I/Os software code register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccswcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccswcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccswcr`]
module"]
pub type CCSWCR = crate::Reg<ccswcr::CCSWCRrs>;
#[doc = "SBS compensation cell for I/Os software code register"]
pub mod ccswcr;
#[doc = "CFGR2 (rw) register accessor: SBS Class B register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "SBS Class B register"]
pub mod cfgr2;
#[doc = "CNSLCKR (rw) register accessor: SBS CPU lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cnslckr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cnslckr`]
module"]
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
#[doc = "SBS CPU lock register"]
pub mod cnslckr;
#[doc = "ECCNMIR (rw) register accessor: SBS flift ECC NMI mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eccnmir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eccnmir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eccnmir`]
module"]
pub type ECCNMIR = crate::Reg<eccnmir::ECCNMIRrs>;
#[doc = "SBS flift ECC NMI mask register"]
pub mod eccnmir;
