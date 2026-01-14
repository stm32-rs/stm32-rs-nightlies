#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccr: [CCR; 7],
    _reserved1: [u8; 0x64],
    csr: CSR,
    cfr: CFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 4],
    _reserved4: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
    _reserved6: [u8; 0x02a4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    ///0x00..0x1c - DMA Multiplexer Channel %s Control register
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x00..0x1c - DMA Multiplexer Channel %s Control register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x80 - DMAMUX request line multiplexer interrupt channel status register
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x84 - DMAMUX request line multiplexer interrupt clear flag register
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x100..0x110 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    ///Iterator for array of:
    ///0x100..0x110 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    ///0x140 - DMAMux - DMA request generator status register
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    ///0x144 - DMAMux - DMA request generator clear flag register
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
    ///0x3ec - DMAMUX hardware configuration 2 register
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    ///0x3f0 - DMAMUX hardware configuration 1 register
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    ///0x3f4 - DMAMUX version register
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    ///0x3f8 - DMAMUX IP identification register
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    ///0x3fc - DMAMUX size identification register
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
/**CCR (rw) register accessor: DMA Multiplexer Channel %s Control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:CCR[0])

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DMA Multiplexer Channel %s Control register
pub mod ccr;
/**RGCR (rw) register accessor: DMAMux - DMA request generator channel x control register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:RGCR[0])

For information about available fields see [`mod@rgcr`] module*/
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
///DMAMux - DMA request generator channel x control register
pub mod rgcr;
/**RGSR (r) register accessor: DMAMux - DMA request generator status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:RGSR)

For information about available fields see [`mod@rgsr`] module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///DMAMux - DMA request generator status register
pub mod rgsr;
/**RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:RGCFR)

For information about available fields see [`mod@rgcfr`] module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///DMAMux - DMA request generator clear flag register
pub mod rgcfr;
/**CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
/**CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
/**SIDR (r) register accessor: DMAMUX size identification register

You can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:SIDR)

For information about available fields see [`mod@sidr`] module*/
pub type SIDR = crate::Reg<sidr::SIDRrs>;
///DMAMUX size identification register
pub mod sidr;
/**IPIDR (r) register accessor: DMAMUX IP identification register

You can [`read`](crate::Reg::read) this register and get [`ipidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:IPIDR)

For information about available fields see [`mod@ipidr`] module*/
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
///DMAMUX IP identification register
pub mod ipidr;
/**VERR (r) register accessor: DMAMUX version register

You can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:VERR)

For information about available fields see [`mod@verr`] module*/
pub type VERR = crate::Reg<verr::VERRrs>;
///DMAMUX version register
pub mod verr;
/**HWCFGR1 (r) register accessor: DMAMUX hardware configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:HWCFGR1)

For information about available fields see [`mod@hwcfgr1`] module*/
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
///DMAMUX hardware configuration 1 register
pub mod hwcfgr1;
/**HWCFGR2 (r) register accessor: DMAMUX hardware configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`hwcfgr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G070.html#DMAMUX:HWCFGR2)

For information about available fields see [`mod@hwcfgr2`] module*/
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
///DMAMUX hardware configuration 2 register
pub mod hwcfgr2;
