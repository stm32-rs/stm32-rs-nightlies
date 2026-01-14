#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    bootsr: BOOTSR,
    _reserved1: [u8; 0x0c],
    hdplcr: HDPLCR,
    hdplsr: HDPLSR,
    _reserved3: [u8; 0x08],
    dbgcr: DBGCR,
    dbglockr: DBGLOCKR,
    _reserved5: [u8; 0x0c],
    rsscmdr: RSSCMDR,
    _reserved6: [u8; 0xc8],
    pmcr: PMCR,
    fpuimr: FPUIMR,
    mesr: MESR,
    _reserved9: [u8; 0x04],
    cccsr: CCCSR,
    ccvalr: CCVALR,
    ccswvalr: CCSWVALR,
    _reserved12: [u8; 0x04],
    bklockr: BKLOCKR,
    _reserved13: [u8; 0x0c],
    exticr1: EXTICR1,
    exticr2: EXTICR2,
    exticr3: EXTICR3,
    exticr4: EXTICR4,
}
impl RegisterBlock {
    ///0x00 - SBS boot status register
    #[inline(always)]
    pub const fn bootsr(&self) -> &BOOTSR {
        &self.bootsr
    }
    ///0x10 - SBS hide protection control register
    #[inline(always)]
    pub const fn hdplcr(&self) -> &HDPLCR {
        &self.hdplcr
    }
    ///0x14 - SBS hide protection status register
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
    ///0x34 - SBS RSS command register
    #[inline(always)]
    pub const fn rsscmdr(&self) -> &RSSCMDR {
        &self.rsscmdr
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
    ///0x110 - SBS I/O compensation cell control and status register
    #[inline(always)]
    pub const fn cccsr(&self) -> &CCCSR {
        &self.cccsr
    }
    ///0x114 - SBS compensation cell for I/Os value register
    #[inline(always)]
    pub const fn ccvalr(&self) -> &CCVALR {
        &self.ccvalr
    }
    ///0x118 - SBS compensation cell for I/Os software value register
    #[inline(always)]
    pub const fn ccswvalr(&self) -> &CCSWVALR {
        &self.ccswvalr
    }
    ///0x120 - SBS break lockup register
    #[inline(always)]
    pub const fn bklockr(&self) -> &BKLOCKR {
        &self.bklockr
    }
    ///0x130 - SBS external interrupt configuration register 0
    #[inline(always)]
    pub const fn exticr1(&self) -> &EXTICR1 {
        &self.exticr1
    }
    ///0x134 - SBS external interrupt configuration register 1
    #[inline(always)]
    pub const fn exticr2(&self) -> &EXTICR2 {
        &self.exticr2
    }
    ///0x138 - SBS external interrupt configuration register 2
    #[inline(always)]
    pub const fn exticr3(&self) -> &EXTICR3 {
        &self.exticr3
    }
    ///0x13c - SBS external interrupt configuration register 3
    #[inline(always)]
    pub const fn exticr4(&self) -> &EXTICR4 {
        &self.exticr4
    }
}
/**BOOTSR (r) register accessor: SBS boot status register

You can [`read`](crate::Reg::read) this register and get [`bootsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:BOOTSR)

For information about available fields see [`mod@bootsr`] module*/
pub type BOOTSR = crate::Reg<bootsr::BOOTSRrs>;
///SBS boot status register
pub mod bootsr;
/**HDPLCR (rw) register accessor: SBS hide protection control register

You can [`read`](crate::Reg::read) this register and get [`hdplcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hdplcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:HDPLCR)

For information about available fields see [`mod@hdplcr`] module*/
pub type HDPLCR = crate::Reg<hdplcr::HDPLCRrs>;
///SBS hide protection control register
pub mod hdplcr;
/**HDPLSR (r) register accessor: SBS hide protection status register

You can [`read`](crate::Reg::read) this register and get [`hdplsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:HDPLSR)

For information about available fields see [`mod@hdplsr`] module*/
pub type HDPLSR = crate::Reg<hdplsr::HDPLSRrs>;
///SBS hide protection status register
pub mod hdplsr;
/**DBGCR (rw) register accessor: SBS debug control register

You can [`read`](crate::Reg::read) this register and get [`dbgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:DBGCR)

For information about available fields see [`mod@dbgcr`] module*/
pub type DBGCR = crate::Reg<dbgcr::DBGCRrs>;
///SBS debug control register
pub mod dbgcr;
/**DBGLOCKR (rw) register accessor: SBS debug lock register

You can [`read`](crate::Reg::read) this register and get [`dbglockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbglockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:DBGLOCKR)

For information about available fields see [`mod@dbglockr`] module*/
pub type DBGLOCKR = crate::Reg<dbglockr::DBGLOCKRrs>;
///SBS debug lock register
pub mod dbglockr;
/**RSSCMDR (rw) register accessor: SBS RSS command register

You can [`read`](crate::Reg::read) this register and get [`rsscmdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsscmdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:RSSCMDR)

For information about available fields see [`mod@rsscmdr`] module*/
pub type RSSCMDR = crate::Reg<rsscmdr::RSSCMDRrs>;
///SBS RSS command register
pub mod rsscmdr;
/**PMCR (rw) register accessor: SBS product mode and configuration register

You can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:PMCR)

For information about available fields see [`mod@pmcr`] module*/
pub type PMCR = crate::Reg<pmcr::PMCRrs>;
///SBS product mode and configuration register
pub mod pmcr;
/**FPUIMR (rw) register accessor: SBS FPU interrupt mask register

You can [`read`](crate::Reg::read) this register and get [`fpuimr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpuimr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:FPUIMR)

For information about available fields see [`mod@fpuimr`] module*/
pub type FPUIMR = crate::Reg<fpuimr::FPUIMRrs>;
///SBS FPU interrupt mask register
pub mod fpuimr;
/**MESR (r) register accessor: SBS memory erase status register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:MESR)

For information about available fields see [`mod@mesr`] module*/
pub type MESR = crate::Reg<mesr::MESRrs>;
///SBS memory erase status register
pub mod mesr;
/**CCCSR (rw) register accessor: SBS I/O compensation cell control and status register

You can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:CCCSR)

For information about available fields see [`mod@cccsr`] module*/
pub type CCCSR = crate::Reg<cccsr::CCCSRrs>;
///SBS I/O compensation cell control and status register
pub mod cccsr;
/**CCVALR (r) register accessor: SBS compensation cell for I/Os value register

You can [`read`](crate::Reg::read) this register and get [`ccvalr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:CCVALR)

For information about available fields see [`mod@ccvalr`] module*/
pub type CCVALR = crate::Reg<ccvalr::CCVALRrs>;
///SBS compensation cell for I/Os value register
pub mod ccvalr;
/**CCSWVALR (rw) register accessor: SBS compensation cell for I/Os software value register

You can [`read`](crate::Reg::read) this register and get [`ccswvalr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccswvalr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:CCSWVALR)

For information about available fields see [`mod@ccswvalr`] module*/
pub type CCSWVALR = crate::Reg<ccswvalr::CCSWVALRrs>;
///SBS compensation cell for I/Os software value register
pub mod ccswvalr;
/**BKLOCKR (rw) register accessor: SBS break lockup register

You can [`read`](crate::Reg::read) this register and get [`bklockr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bklockr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:BKLOCKR)

For information about available fields see [`mod@bklockr`] module*/
pub type BKLOCKR = crate::Reg<bklockr::BKLOCKRrs>;
///SBS break lockup register
pub mod bklockr;
/**EXTICR1 (rw) register accessor: SBS external interrupt configuration register 0

You can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR1)

For information about available fields see [`mod@exticr1`] module*/
pub type EXTICR1 = crate::Reg<exticr1::EXTICR1rs>;
///SBS external interrupt configuration register 0
pub mod exticr1;
/**EXTICR2 (rw) register accessor: SBS external interrupt configuration register 1

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR2)

For information about available fields see [`mod@exticr2`] module*/
pub type EXTICR2 = crate::Reg<exticr2::EXTICR2rs>;
///SBS external interrupt configuration register 1
pub mod exticr2;
/**EXTICR3 (rw) register accessor: SBS external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR3)

For information about available fields see [`mod@exticr3`] module*/
pub type EXTICR3 = crate::Reg<exticr3::EXTICR3rs>;
///SBS external interrupt configuration register 2
pub mod exticr3;
/**EXTICR4 (rw) register accessor: SBS external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#SBS:EXTICR4)

For information about available fields see [`mod@exticr4`] module*/
pub type EXTICR4 = crate::Reg<exticr4::EXTICR4rs>;
///SBS external interrupt configuration register 3
pub mod exticr4;
