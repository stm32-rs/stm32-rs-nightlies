#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccr: [CCR; 16],
    _reserved1: [u8; 0x40],
    csr: CSR,
    cfr: CFR,
    _reserved3: [u8; 0x78],
    rgcr: [RGCR; 8],
    _reserved4: [u8; 0x20],
    rgsr: RGSR,
    rgcfr: RGCFR,
}
impl RegisterBlock {
    ///0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x00..0x40 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x00 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c0cr(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x04 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c1cr(&self) -> &CCR {
        self.ccr(1)
    }
    ///0x08 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c2cr(&self) -> &CCR {
        self.ccr(2)
    }
    ///0x0c - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c3cr(&self) -> &CCR {
        self.ccr(3)
    }
    ///0x10 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c4cr(&self) -> &CCR {
        self.ccr(4)
    }
    ///0x14 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c5cr(&self) -> &CCR {
        self.ccr(5)
    }
    ///0x18 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c6cr(&self) -> &CCR {
        self.ccr(6)
    }
    ///0x1c - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c7cr(&self) -> &CCR {
        self.ccr(7)
    }
    ///0x20 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c8cr(&self) -> &CCR {
        self.ccr(8)
    }
    ///0x24 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c9cr(&self) -> &CCR {
        self.ccr(9)
    }
    ///0x28 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c10cr(&self) -> &CCR {
        self.ccr(10)
    }
    ///0x2c - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c11cr(&self) -> &CCR {
        self.ccr(11)
    }
    ///0x30 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c12cr(&self) -> &CCR {
        self.ccr(12)
    }
    ///0x34 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c13cr(&self) -> &CCR {
        self.ccr(13)
    }
    ///0x38 - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c14cr(&self) -> &CCR {
        self.ccr(14)
    }
    ///0x3c - DMAMux - DMA request line multiplexer channel x control register
    #[inline(always)]
    pub const fn c15cr(&self) -> &CCR {
        self.ccr(15)
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
    ///0x100..0x120 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    ///Iterator for array of:
    ///0x100..0x120 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    ///0x100 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg0cr(&self) -> &RGCR {
        self.rgcr(0)
    }
    ///0x104 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg1cr(&self) -> &RGCR {
        self.rgcr(1)
    }
    ///0x108 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg2cr(&self) -> &RGCR {
        self.rgcr(2)
    }
    ///0x10c - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg3cr(&self) -> &RGCR {
        self.rgcr(3)
    }
    ///0x110 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg4cr(&self) -> &RGCR {
        self.rgcr(4)
    }
    ///0x114 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg5cr(&self) -> &RGCR {
        self.rgcr(5)
    }
    ///0x118 - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg6cr(&self) -> &RGCR {
        self.rgcr(6)
    }
    ///0x11c - DMAMux - DMA request generator channel x control register
    #[inline(always)]
    pub const fn rg7cr(&self) -> &RGCR {
        self.rgcr(7)
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
}
/**CCR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:C[0]CR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///DMAMux - DMA request line multiplexer channel x control register
pub mod ccr;
/**RGCR (rw) register accessor: DMAMux - DMA request generator channel x control register

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:RG[0]CR)

For information about available fields see [`mod@rgcr`] module*/
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
///DMAMux - DMA request generator channel x control register
pub mod rgcr;
/**RGSR (r) register accessor: DMAMux - DMA request generator status register

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:RGSR)

For information about available fields see [`mod@rgsr`] module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///DMAMux - DMA request generator status register
pub mod rgsr;
/**RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:RGCFR)

For information about available fields see [`mod@rgcfr`] module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///DMAMux - DMA request generator clear flag register
pub mod rgcfr;
/**CSR (r) register accessor: DMAMUX request line multiplexer interrupt channel status register

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///DMAMUX request line multiplexer interrupt channel status register
pub mod csr;
/**CFR (w) register accessor: DMAMUX request line multiplexer interrupt clear flag register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7A3.html#DMAMUX1:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///DMAMUX request line multiplexer interrupt clear flag register
pub mod cfr;
