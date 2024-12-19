#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccr: [CCR; 5],
    _reserved1: [u8; 0x6c],
    csr: CSR,
    cfr: CFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 4],
    _reserved4: [u8; 0x30],
    rgsr: RGSR,
    rgcfr: RGCFR,
}
impl RegisterBlock {
    ///0x00..0x14 - DMA Multiplexer Channel %s Control register
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x00..0x14 - DMA Multiplexer Channel %s Control register
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
    ///0x100..0x110 - DMAMUX request generator channel %s configuration register
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    ///Iterator for array of:
    ///0x100..0x110 - DMAMUX request generator channel %s configuration register
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    ///0x140 - DMAMUX request generator interrupt status register
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    ///0x144 - DMAMUX request generator interrupt clear flag register
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
}
/**CCR (rw) register accessor: DMA Multiplexer Channel %s Control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:CCR[0])

For information about available fields see [`mod@ccr`]
module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DMA Multiplexer Channel %s Control register
pub mod ccr;
/**CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:CSR)

For information about available fields see [`mod@csr`]
module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
/**CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:CFR)

For information about available fields see [`mod@cfr`]
module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
/**RGCR (rw) register accessor: DMAMUX request generator channel %s configuration register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:RGCR[0])

For information about available fields see [`mod@rgcr`]
module*/
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
///DMAMUX request generator channel %s configuration register
pub mod rgcr;
/**RGSR (r) register accessor: DMAMUX request generator interrupt status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:RGSR)

For information about available fields see [`mod@rgsr`]
module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///DMAMUX request generator interrupt status register
pub mod rgsr;
/**RGCFR (w) register accessor: DMAMUX request generator interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C071.html#DMAMUX:RGCFR)

For information about available fields see [`mod@rgcfr`]
module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///DMAMUX request generator interrupt clear flag register
pub mod rgcfr;
