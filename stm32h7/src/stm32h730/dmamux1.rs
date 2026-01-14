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
    ///0x00..0x40 -
    #[inline(always)]
    pub const fn ccr(&self, n: usize) -> &CCR {
        &self.ccr[n]
    }
    ///Iterator for array of:
    ///0x00..0x40 -
    #[inline(always)]
    pub fn ccr_iter(&self) -> impl Iterator<Item = &CCR> {
        self.ccr.iter()
    }
    ///0x00 - C0CR
    #[inline(always)]
    pub const fn c0cr(&self) -> &CCR {
        self.ccr(0)
    }
    ///0x04 - C1CR
    #[inline(always)]
    pub const fn c1cr(&self) -> &CCR {
        self.ccr(1)
    }
    ///0x08 - C2CR
    #[inline(always)]
    pub const fn c2cr(&self) -> &CCR {
        self.ccr(2)
    }
    ///0x0c - C3CR
    #[inline(always)]
    pub const fn c3cr(&self) -> &CCR {
        self.ccr(3)
    }
    ///0x10 - C4CR
    #[inline(always)]
    pub const fn c4cr(&self) -> &CCR {
        self.ccr(4)
    }
    ///0x14 - C5CR
    #[inline(always)]
    pub const fn c5cr(&self) -> &CCR {
        self.ccr(5)
    }
    ///0x18 - C6CR
    #[inline(always)]
    pub const fn c6cr(&self) -> &CCR {
        self.ccr(6)
    }
    ///0x1c - C7CR
    #[inline(always)]
    pub const fn c7cr(&self) -> &CCR {
        self.ccr(7)
    }
    ///0x20 - C8CR
    #[inline(always)]
    pub const fn c8cr(&self) -> &CCR {
        self.ccr(8)
    }
    ///0x24 - C9CR
    #[inline(always)]
    pub const fn c9cr(&self) -> &CCR {
        self.ccr(9)
    }
    ///0x28 - C10CR
    #[inline(always)]
    pub const fn c10cr(&self) -> &CCR {
        self.ccr(10)
    }
    ///0x2c - C11CR
    #[inline(always)]
    pub const fn c11cr(&self) -> &CCR {
        self.ccr(11)
    }
    ///0x30 - C12CR
    #[inline(always)]
    pub const fn c12cr(&self) -> &CCR {
        self.ccr(12)
    }
    ///0x34 - C13CR
    #[inline(always)]
    pub const fn c13cr(&self) -> &CCR {
        self.ccr(13)
    }
    ///0x38 - C14CR
    #[inline(always)]
    pub const fn c14cr(&self) -> &CCR {
        self.ccr(14)
    }
    ///0x3c - C15CR
    #[inline(always)]
    pub const fn c15cr(&self) -> &CCR {
        self.ccr(15)
    }
    ///0x80 -
    #[inline(always)]
    pub const fn csr(&self) -> &CSR {
        &self.csr
    }
    ///0x84 -
    #[inline(always)]
    pub const fn cfr(&self) -> &CFR {
        &self.cfr
    }
    ///0x100..0x120 -
    #[inline(always)]
    pub const fn rgcr(&self, n: usize) -> &RGCR {
        &self.rgcr[n]
    }
    ///Iterator for array of:
    ///0x100..0x120 -
    #[inline(always)]
    pub fn rgcr_iter(&self) -> impl Iterator<Item = &RGCR> {
        self.rgcr.iter()
    }
    ///0x100 - RG0CR
    #[inline(always)]
    pub const fn rg0cr(&self) -> &RGCR {
        self.rgcr(0)
    }
    ///0x104 - RG1CR
    #[inline(always)]
    pub const fn rg1cr(&self) -> &RGCR {
        self.rgcr(1)
    }
    ///0x108 - RG2CR
    #[inline(always)]
    pub const fn rg2cr(&self) -> &RGCR {
        self.rgcr(2)
    }
    ///0x10c - RG3CR
    #[inline(always)]
    pub const fn rg3cr(&self) -> &RGCR {
        self.rgcr(3)
    }
    ///0x110 - RG4CR
    #[inline(always)]
    pub const fn rg4cr(&self) -> &RGCR {
        self.rgcr(4)
    }
    ///0x114 - RG5CR
    #[inline(always)]
    pub const fn rg5cr(&self) -> &RGCR {
        self.rgcr(5)
    }
    ///0x118 - RG6CR
    #[inline(always)]
    pub const fn rg6cr(&self) -> &RGCR {
        self.rgcr(6)
    }
    ///0x11c - RG7CR
    #[inline(always)]
    pub const fn rg7cr(&self) -> &RGCR {
        self.rgcr(7)
    }
    ///0x140 -
    #[inline(always)]
    pub const fn rgsr(&self) -> &RGSR {
        &self.rgsr
    }
    ///0x144 -
    #[inline(always)]
    pub const fn rgcfr(&self) -> &RGCFR {
        &self.rgcfr
    }
}
/**CCR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:C[0]CR)

For information about available fields see [`mod@ccr`] module*/
pub type CCR = crate::Reg<ccr::CCRrs>;
///
pub mod ccr;
/**CSR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:CSR)

For information about available fields see [`mod@csr`] module*/
pub type CSR = crate::Reg<csr::CSRrs>;
///
pub mod csr;
/**CFR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:CFR)

For information about available fields see [`mod@cfr`] module*/
pub type CFR = crate::Reg<cfr::CFRrs>;
///
pub mod cfr;
/**RGCR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`rgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:RG[0]CR)

For information about available fields see [`mod@rgcr`] module*/
pub type RGCR = crate::Reg<rgcr::RGCRrs>;
///
pub mod rgcr;
/**RGSR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:RGSR)

For information about available fields see [`mod@rgsr`] module*/
pub type RGSR = crate::Reg<rgsr::RGSRrs>;
///
pub mod rgsr;
/**RGCFR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#DMAMUX1:RGCFR)

For information about available fields see [`mod@rgcfr`] module*/
pub type RGCFR = crate::Reg<rgcfr::RGCFRrs>;
///
pub mod rgcfr;
