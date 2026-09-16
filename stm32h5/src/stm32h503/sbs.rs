#[repr(C)]
#[derive(Debug)]
///Register block
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
    ///0x144 - SBS CPU lock register
    #[inline(always)]
    pub const fn cnslckr(&self) -> &CNSLCKR {
        &self.cnslckr
    }
    ///0x14c - SBS flift ECC NMI mask register
    #[inline(always)]
    pub const fn eccnmir(&self) -> &ECCNMIR {
        &self.eccnmir
    }
}
/**HDPLCR (rw) register accessor: SBS temporal isolation control register

You can [`read`](crate::Reg::read) this register and get [`hdplcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:HDPLCR)

For information about available fields see [`mod@hdplcr`] module*/
pub type HDPLCR = crate::Reg<hdplcr::HDPLCRrs>;
///SBS temporal isolation control register
pub mod hdplcr;
/**HDPLSR (r) register accessor: SBS temporal isolation status register

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:HDPLSR)

For information about available fields see [`mod@hdplsr`] module*/
pub type HDPLSR = crate::Reg<hdplsr::HDPLSRrs>;
///SBS temporal isolation status register
pub mod hdplsr;
/**DBGCR (rw) register accessor: SBS debug control register

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:DBGCR)

For information about available fields see [`mod@dbgcr`] module*/
pub type DBGCR = crate::Reg<dbgcr::DBGCRrs>;
///SBS debug control register
pub mod dbgcr;
/**DBGLOCKR (rw) register accessor: SBS debug lock register

You can [`read`](crate::Reg::read) this register and get [`dbglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:DBGLOCKR)

For information about available fields see [`mod@dbglockr`] module*/
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKRrs>;
///SBS debug lock register
pub mod dbglockr;
/**PMCR (rw) register accessor: SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///SBS product mode and configuration register
pub mod pmcr;
/**FPUIMR (rw) register accessor: SBS FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:FPUIMR)

For information about available fields see [`mod@fpuimr`] module*/
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
///SBS FPU interrupt mask register
pub mod fpuimr;
/**MESR (rw) register accessor: SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:MESR)

For information about available fields see [`mod@mesr`] module*/
pub type MESR = crate::Reg<mesr::MESRrs>;
///SBS memory erase status register
pub mod mesr;
/**CCCSR (rw) register accessor: SBS compensation cell for I/Os control and status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///SBS compensation cell for I/Os control and status register
pub mod cccsr;
/**CCVALR (r) register accessor: SBS compensation cell for I/Os value register

You can [`read`](crate::Reg::read) this register and get [`ccvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CCVALR)

For information about available fields see [`mod@ccvalr`] module*/
pub type CCVALR = crate::Reg<ccvalr::CCVALRrs>;
///SBS compensation cell for I/Os value register
pub mod ccvalr;
/**CCSWCR (rw) register accessor: SBS compensation cell for I/Os software code register

You can [`read`](crate::Reg::read) this register and get [`ccswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CCSWCR)

For information about available fields see [`mod@ccswcr`] module*/
pub type CCSWCR = crate::Reg<ccswcr::CCSWCRrs>;
///SBS compensation cell for I/Os software code register
pub mod ccswcr;
/**CFGR2 (rw) register accessor: SBS Class B register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///SBS Class B register
pub mod cfgr2;
/**CNSLCKR (rw) register accessor: SBS CPU lock register

You can [`read`](crate::Reg::read) this register and get [`cnslckr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnslckr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:CNSLCKR)

For information about available fields see [`mod@cnslckr`] module*/
pub type CNSLCKR = crate::Reg<cnslckr::CNSLCKRrs>;
///SBS CPU lock register
pub mod cnslckr;
/**ECCNMIR (rw) register accessor: SBS flift ECC NMI mask register

You can [`read`](crate::Reg::read) this register and get [`eccnmir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccnmir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#SBS:ECCNMIR)

For information about available fields see [`mod@eccnmir`] module*/
pub type ECCNMIR = crate::Reg<eccnmir::ECCNMIRrs>;
///SBS flift ECC NMI mask register
pub mod eccnmir;
