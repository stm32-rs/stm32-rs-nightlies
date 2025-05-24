#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    hdplcr: HDPLCR,
    hdplsr: HDPLSR,
    nexthdplcr: NEXTHDPLCR,
    _reserved3: [u8; 0x04],
    dbgcr: DBGCR,
    dbglockr: DBGLOCKR,
    _reserved5: [u8; 0x0c],
    rsscmdr: RSSCMDR,
    _reserved6: [u8; 0x68],
    epochselcr: EPOCHSELCR,
    _reserved7: [u8; 0x1c],
    seccfgr: SECCFGR,
    _reserved8: [u8; 0x3c],
    pmcr: PMCR,
    fpuimr: FPUIMR,
    mesr: MESR,
    _reserved11: [u8; 0x04],
    cccsr: CCCSR,
    ccvalr: CCVALR,
    ccswcr: CCSWCR,
    _reserved14: [u8; 0x04],
    cfgr2: CFGR2,
    _reserved15: [u8; 0x20],
    cnslckr: CNSLCKR,
    cslckr: CSLCKR,
    eccnmir: ECCNMIR,
}
impl RegisterBlock {
    ///0x10 - SBS temporal isolation control register
    #[inline(always)]
    pub const fn hdplcr(&self) -> &HDPLCR {
        &self.hdplcr
    }
    ///0x14 - SBS temporal isolation status register
    #[inline(always)]
    pub const fn hdplsr(&self) -> &HDPLSR {
        &self.hdplsr
    }
    ///0x18 - SBS next HDPL control register
    #[inline(always)]
    pub const fn nexthdplcr(&self) -> &NEXTHDPLCR {
        &self.nexthdplcr
    }
    ///0x20 - SBS debug control register
    #[inline(always)]
    pub const fn dbgcr(&self) -> &DBGCR {
        &self.dbgcr
    }
    ///0x24 - SBS debug lock register
    #[inline(always)]
    pub const fn dbglockr(&self) -> &DBGLOCKR {
        &self.dbglockr
    }
    ///0x34 - SBS RSS command register
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
    }
    ///0xa0 - SBS EPOCH selection control register
    #[inline(always)]
    pub const fn epochselcr(&self) -> &EPOCHSELCR {
        &self.epochselcr
    }
    ///0xc0 - SBS security mode configuration control register
    #[inline(always)]
    pub const fn seccfgr(&self) -> &SECCFGR {
        &self.seccfgr
    }
    ///0x100 - SBS product mode and configuration register
    #[inline(always)]
    pub const fn pmcr(&self) -> &PMCR {
        &self.pmcr
    }
    ///0x104 - SBS FPU interrupt mask register
    #[inline(always)]
    pub const fn fpuimr(&self) -> &FPUIMR {
        &self.fpuimr
    }
    ///0x108 - SBS memory erase status register
    #[inline(always)]
    pub const fn mesr(&self) -> &MESR {
        &self.mesr
    }
    ///0x110 - SBS compensation cell for I/Os control and status register
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    ///0x114 - SBS compensation cell for I/Os value register
    #[inline(always)]
    pub const fn ccvalr(&self) -> &CCVALR {
        &self.ccvalr
    }
    ///0x118 - SBS compensation cell for I/Os software code register
    #[inline(always)]
    pub const fn ccswcr(&self) -> &CCSWCR {
        &self.ccswcr
    }
    ///0x120 - SBS Class B register
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x144 - SBS CPU non-secure lock register
    #[inline(always)]
    pub const fn cnslckr(&self) -> &CNSLCKR {
        &self.cnslckr
    }
    ///0x148 - SBS CPU secure lock register
    #[inline(always)]
    pub const fn cslckr(&self) -> &CSLCKR {
        &self.cslckr
    }
    ///0x14c - SBS flift ECC NMI mask register
    #[inline(always)]
    pub const fn eccnmir(&self) -> &ECCNMIR {
        &self.eccnmir
    }
}
/**HDPLCR (rw) register accessor: SBS temporal isolation control register

You can [`read`](crate::Reg::read) this register and get [`hdplcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:HDPLCR)

For information about available fields see [`mod@hdplcr`] module*/
pub type HDPLCR = crate::Reg<hdplcr::HDPLCRrs>;
///SBS temporal isolation control register
pub mod hdplcr;
/**HDPLSR (r) register accessor: SBS temporal isolation status register

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:HDPLSR)

For information about available fields see [`mod@hdplsr`] module*/
pub type HDPLSR = crate::Reg<hdplsr::HDPLSRrs>;
///SBS temporal isolation status register
pub mod hdplsr;
/**NEXTHDPLCR (rw) register accessor: SBS next HDPL control register

You can [`read`](crate::Reg::read) this register and get [`nexthdplcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nexthdplcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:NEXTHDPLCR)

For information about available fields see [`mod@nexthdplcr`] module*/
pub type NEXTHDPLCR = crate::Reg<nexthdplcr::NEXTHDPLCRrs>;
///SBS next HDPL control register
pub mod nexthdplcr;
/**DBGCR (rw) register accessor: SBS debug control register

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:DBGCR)

For information about available fields see [`mod@dbgcr`] module*/
pub type DBGCR = crate::Reg<dbgcr::DBGCRrs>;
///SBS debug control register
pub mod dbgcr;
/**DBGLOCKR (rw) register accessor: SBS debug lock register

You can [`read`](crate::Reg::read) this register and get [`dbglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:DBGLOCKR)

For information about available fields see [`mod@dbglockr`] module*/
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKRrs>;
///SBS debug lock register
pub mod dbglockr;
/**RSSCMDR (rw) register accessor: SBS RSS command register

You can [`read`](crate::Reg::read) this register and get [`rsscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:RSSCMDR)

For information about available fields see [`mod@rsscmdr`] module*/
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
///SBS RSS command register
pub mod rsscmdr;
/**EPOCHSELCR (rw) register accessor: SBS EPOCH selection control register

You can [`read`](crate::Reg::read) this register and get [`epochselcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epochselcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:EPOCHSELCR)

For information about available fields see [`mod@epochselcr`] module*/
pub type EPOCHSELCR = crate::Reg<epochselcr::EPOCHSELCRrs>;
///SBS EPOCH selection control register
pub mod epochselcr;
/**SECCFGR (rw) register accessor: SBS security mode configuration control register

You can [`read`](crate::Reg::read) this register and get [`seccfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:SECCFGR)

For information about available fields see [`mod@seccfgr`] module*/
pub type SECCFGR = crate::Reg<seccfgr::SECCFGRrs>;
///SBS security mode configuration control register
pub mod seccfgr;
/**PMCR (rw) register accessor: SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///SBS product mode and configuration register
pub mod pmcr;
/**FPUIMR (rw) register accessor: SBS FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:FPUIMR)

For information about available fields see [`mod@fpuimr`] module*/
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
///SBS FPU interrupt mask register
pub mod fpuimr;
/**MESR (rw) register accessor: SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:MESR)

For information about available fields see [`mod@mesr`] module*/
pub type MESR = crate::Reg<mesr::MESRrs>;
///SBS memory erase status register
pub mod mesr;
/**CCCSR (rw) register accessor: SBS compensation cell for I/Os control and status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///SBS compensation cell for I/Os control and status register
pub mod cccsr;
/**CCVALR (r) register accessor: SBS compensation cell for I/Os value register

You can [`read`](crate::Reg::read) this register and get [`ccvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CCVALR)

For information about available fields see [`mod@ccvalr`] module*/
pub type CCVALR = crate::Reg<ccvalr::CCVALRrs>;
///SBS compensation cell for I/Os value register
pub mod ccvalr;
/**CCSWCR (rw) register accessor: SBS compensation cell for I/Os software code register

You can [`read`](crate::Reg::read) this register and get [`ccswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CCSWCR)

For information about available fields see [`mod@ccswcr`] module*/
pub type CCSWCR = crate::Reg<ccswcr::CCSWCRrs>;
///SBS compensation cell for I/Os software code register
pub mod ccswcr;
/**CFGR2 (rw) register accessor: SBS Class B register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///SBS Class B register
pub mod cfgr2;
/**CNSLCKR (rw) register accessor: SBS CPU non-secure lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CNSLCKR)

For information about available fields see [`mod@cnslckr`] module*/
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
///SBS CPU non-secure lock register
pub mod cnslckr;
/**CSLCKR (rw) register accessor: SBS CPU secure lock register

You can [`read`](crate::Reg::read) this register and get [`cslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:CSLCKR)

For information about available fields see [`mod@cslckr`] module*/
pub type CSLCKR = crate::Reg<cslckr::CSLCKRrs>;
///SBS CPU secure lock register
pub mod cslckr;
/**ECCNMIR (rw) register accessor: SBS flift ECC NMI mask register

You can [`read`](crate::Reg::read) this register and get [`eccnmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccnmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#SBS:ECCNMIR)

For information about available fields see [`mod@eccnmir`] module*/
pub type ECCNMIR = crate::Reg<eccnmir::ECCNMIRrs>;
///SBS flift ECC NMI mask register
pub mod eccnmir;
