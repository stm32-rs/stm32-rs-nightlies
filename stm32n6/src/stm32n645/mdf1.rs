#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    ckgcr: CKGCR,
    _reserved2: [u8; 0x78],
    sitf0cr: SITF0CR,
    bsmx0cr: BSMX0CR,
    dflt0cr: DFLT0CR,
    dflt0cicr: DFLT0CICR,
    dflt0rsfr: DFLT0RSFR,
    dflt0intr: DFLT0INTR,
    old0cr: OLD0CR,
    old0thlr: OLD0THLR,
    old0thhr: OLD0THHR,
    dly0cr: DLY0CR,
    scd0cr: SCD0CR,
    dflt0ier: DFLT0IER,
    dflt0isr: DFLT0ISR,
    oec0cr: OEC0CR,
    _reserved16: [u8; 0x34],
    snps0dr: SNPS0DR,
    dflt0dr: DFLT0DR,
    _reserved18: [u8; 0x0c],
    sitf1cr: SITF1CR,
    bsmx1cr: BSMX1CR,
    dflt1cr: DFLT1CR,
    dflt1cicr: DFLT1CICR,
    dflt1rsfr: DFLT1RSFR,
    dflt1intr: DFLT1INTR,
    old1cr: OLD1CR,
    old1thlr: OLD1THLR,
    old1thhr: OLD1THHR,
    dly1cr: DLY1CR,
    scd1cr: SCD1CR,
    dflt1ier: DFLT1IER,
    dflt1isr: DFLT1ISR,
    oec1cr: OEC1CR,
    _reserved32: [u8; 0x34],
    snps1dr: SNPS1DR,
    dflt1dr: DFLT1DR,
    _reserved34: [u8; 0x0c],
    sitf2cr: SITF2CR,
    bsmx2cr: BSMX2CR,
    dflt2cr: DFLT2CR,
    dflt2cicr: DFLT2CICR,
    dflt2rsfr: DFLT2RSFR,
    dflt2intr: DFLT2INTR,
    old2cr: OLD2CR,
    old2thlr: OLD2THLR,
    old2thhr: OLD2THHR,
    dly2cr: DLY2CR,
    scd2cr: SCD2CR,
    dflt2ier: DFLT2IER,
    dflt2isr: DFLT2ISR,
    oec2cr: OEC2CR,
    _reserved48: [u8; 0x34],
    snps2dr: SNPS2DR,
    dflt2dr: DFLT2DR,
    _reserved50: [u8; 0x0c],
    sitf3cr: SITF3CR,
    bsmx3cr: BSMX3CR,
    dflt3cr: DFLT3CR,
    dflt3cicr: DFLT3CICR,
    dflt3rsfr: DFLT3RSFR,
    dflt3intr: DFLT3INTR,
    old3cr: OLD3CR,
    old3thlr: OLD3THLR,
    old3thhr: OLD3THHR,
    dly3cr: DLY3CR,
    scd3cr: SCD3CR,
    dflt3ier: DFLT3IER,
    dflt3isr: DFLT3ISR,
    oec3cr: OEC3CR,
    _reserved64: [u8; 0x34],
    snps3dr: SNPS3DR,
    dflt3dr: DFLT3DR,
    _reserved66: [u8; 0x0c],
    sitf4cr: SITF4CR,
    bsmx4cr: BSMX4CR,
    dflt4cr: DFLT4CR,
    dflt4cicr: DFLT4CICR,
    dflt4rsfr: DFLT4RSFR,
    dflt4intr: DFLT4INTR,
    old4cr: OLD4CR,
    old4thlr: OLD4THLR,
    old4thhr: OLD4THHR,
    dly4cr: DLY4CR,
    scd4cr: SCD4CR,
    dflt4ier: DFLT4IER,
    dflt4isr: DFLT4ISR,
    oec4cr: OEC4CR,
    _reserved80: [u8; 0x34],
    snps4dr: SNPS4DR,
    dflt4dr: DFLT4DR,
    _reserved82: [u8; 0x0c],
    sitf5cr: SITF5CR,
    bsmx5cr: BSMX5CR,
    dflt5cr: DFLT5CR,
    dflt5cicr: DFLT5CICR,
    dflt5rsfr: DFLT5RSFR,
    dflt5intr: DFLT5INTR,
    old5cr: OLD5CR,
    old5thlr: OLD5THLR,
    old5thhr: OLD5THHR,
    dly5cr: DLY5CR,
    scd5cr: SCD5CR,
    dflt5ier: DFLT5IER,
    dflt5isr: DFLT5ISR,
    oec5cr: OEC5CR,
    _reserved96: [u8; 0x34],
    snps5dr: SNPS5DR,
    dflt5dr: DFLT5DR,
}
impl RegisterBlock {
    ///0x00 - MDF global control register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x04 - MDF clock generator control register
    #[inline(always)]
    pub const fn ckgcr(&self) -> &CKGCR {
        &self.ckgcr
    }
    ///0x80 - MDF serial interface control register 0
    #[inline(always)]
    pub const fn sitf0cr(&self) -> &SITF0CR {
        &self.sitf0cr
    }
    ///0x84 - MDF bitstream matrix control register 0
    #[inline(always)]
    pub const fn bsmx0cr(&self) -> &BSMX0CR {
        &self.bsmx0cr
    }
    ///0x88 - MDF digital filter control register 0
    #[inline(always)]
    pub const fn dflt0cr(&self) -> &DFLT0CR {
        &self.dflt0cr
    }
    ///0x8c - MDF digital filter configuration register 0
    #[inline(always)]
    pub const fn dflt0cicr(&self) -> &DFLT0CICR {
        &self.dflt0cicr
    }
    ///0x90 - MDF reshape filter configuration register 0
    #[inline(always)]
    pub const fn dflt0rsfr(&self) -> &DFLT0RSFR {
        &self.dflt0rsfr
    }
    ///0x94 - MDF integrator configuration register 0
    #[inline(always)]
    pub const fn dflt0intr(&self) -> &DFLT0INTR {
        &self.dflt0intr
    }
    ///0x98 - MDF out-of limit detector control register 0
    #[inline(always)]
    pub const fn old0cr(&self) -> &OLD0CR {
        &self.old0cr
    }
    ///0x9c - MDF OLD0 low threshold register 0
    #[inline(always)]
    pub const fn old0thlr(&self) -> &OLD0THLR {
        &self.old0thlr
    }
    ///0xa0 - MDF OLD0 high threshold register 0
    #[inline(always)]
    pub const fn old0thhr(&self) -> &OLD0THHR {
        &self.old0thhr
    }
    ///0xa4 - MDF delay control register 0
    #[inline(always)]
    pub const fn dly0cr(&self) -> &DLY0CR {
        &self.dly0cr
    }
    ///0xa8 - MDF short circuit detector control register 0
    #[inline(always)]
    pub const fn scd0cr(&self) -> &SCD0CR {
        &self.scd0cr
    }
    ///0xac - MDF DFLT0 interrupt enable register 0
    #[inline(always)]
    pub const fn dflt0ier(&self) -> &DFLT0IER {
        &self.dflt0ier
    }
    ///0xb0 - MDF DFLT0 interrupt status register 0
    #[inline(always)]
    pub const fn dflt0isr(&self) -> &DFLT0ISR {
        &self.dflt0isr
    }
    ///0xb4 - MDF offset error compensation control register 0
    #[inline(always)]
    pub const fn oec0cr(&self) -> &OEC0CR {
        &self.oec0cr
    }
    ///0xec - MDF snapshot data register 0
    #[inline(always)]
    pub const fn snps0dr(&self) -> &SNPS0DR {
        &self.snps0dr
    }
    ///0xf0 - MDF digital filter data register 0
    #[inline(always)]
    pub const fn dflt0dr(&self) -> &DFLT0DR {
        &self.dflt0dr
    }
    ///0x100 - MDF serial interface control register 1
    #[inline(always)]
    pub const fn sitf1cr(&self) -> &SITF1CR {
        &self.sitf1cr
    }
    ///0x104 - MDF bitstream matrix control register 1
    #[inline(always)]
    pub const fn bsmx1cr(&self) -> &BSMX1CR {
        &self.bsmx1cr
    }
    ///0x108 - MDF digital filter control register 1
    #[inline(always)]
    pub const fn dflt1cr(&self) -> &DFLT1CR {
        &self.dflt1cr
    }
    ///0x10c - MDF digital filter configuration register 1
    #[inline(always)]
    pub const fn dflt1cicr(&self) -> &DFLT1CICR {
        &self.dflt1cicr
    }
    ///0x110 - MDF reshape filter configuration register 1
    #[inline(always)]
    pub const fn dflt1rsfr(&self) -> &DFLT1RSFR {
        &self.dflt1rsfr
    }
    ///0x114 - MDF integrator configuration register 1
    #[inline(always)]
    pub const fn dflt1intr(&self) -> &DFLT1INTR {
        &self.dflt1intr
    }
    ///0x118 - MDF out-of limit detector control register 1
    #[inline(always)]
    pub const fn old1cr(&self) -> &OLD1CR {
        &self.old1cr
    }
    ///0x11c - MDF OLD1 low threshold register 1
    #[inline(always)]
    pub const fn old1thlr(&self) -> &OLD1THLR {
        &self.old1thlr
    }
    ///0x120 - MDF OLD1 high threshold register 1
    #[inline(always)]
    pub const fn old1thhr(&self) -> &OLD1THHR {
        &self.old1thhr
    }
    ///0x124 - MDF delay control register 1
    #[inline(always)]
    pub const fn dly1cr(&self) -> &DLY1CR {
        &self.dly1cr
    }
    ///0x128 - MDF short circuit detector control register 1
    #[inline(always)]
    pub const fn scd1cr(&self) -> &SCD1CR {
        &self.scd1cr
    }
    ///0x12c - MDF DFLT1 interrupt enable register 1
    #[inline(always)]
    pub const fn dflt1ier(&self) -> &DFLT1IER {
        &self.dflt1ier
    }
    ///0x130 - MDF DFLT1 interrupt status register 1
    #[inline(always)]
    pub const fn dflt1isr(&self) -> &DFLT1ISR {
        &self.dflt1isr
    }
    ///0x134 - MDF offset error compensation control register 1
    #[inline(always)]
    pub const fn oec1cr(&self) -> &OEC1CR {
        &self.oec1cr
    }
    ///0x16c - MDF snapshot data register 1
    #[inline(always)]
    pub const fn snps1dr(&self) -> &SNPS1DR {
        &self.snps1dr
    }
    ///0x170 - MDF digital filter data register 1
    #[inline(always)]
    pub const fn dflt1dr(&self) -> &DFLT1DR {
        &self.dflt1dr
    }
    ///0x180 - MDF serial interface control register 2
    #[inline(always)]
    pub const fn sitf2cr(&self) -> &SITF2CR {
        &self.sitf2cr
    }
    ///0x184 - MDF bitstream matrix control register 2
    #[inline(always)]
    pub const fn bsmx2cr(&self) -> &BSMX2CR {
        &self.bsmx2cr
    }
    ///0x188 - MDF digital filter control register 2
    #[inline(always)]
    pub const fn dflt2cr(&self) -> &DFLT2CR {
        &self.dflt2cr
    }
    ///0x18c - MDF digital filter configuration register 2
    #[inline(always)]
    pub const fn dflt2cicr(&self) -> &DFLT2CICR {
        &self.dflt2cicr
    }
    ///0x190 - MDF reshape filter configuration register 2
    #[inline(always)]
    pub const fn dflt2rsfr(&self) -> &DFLT2RSFR {
        &self.dflt2rsfr
    }
    ///0x194 - MDF integrator configuration register 2
    #[inline(always)]
    pub const fn dflt2intr(&self) -> &DFLT2INTR {
        &self.dflt2intr
    }
    ///0x198 - MDF out-of limit detector control register 2
    #[inline(always)]
    pub const fn old2cr(&self) -> &OLD2CR {
        &self.old2cr
    }
    ///0x19c - MDF OLD2 low threshold register 2
    #[inline(always)]
    pub const fn old2thlr(&self) -> &OLD2THLR {
        &self.old2thlr
    }
    ///0x1a0 - MDF OLD2 high threshold register 2
    #[inline(always)]
    pub const fn old2thhr(&self) -> &OLD2THHR {
        &self.old2thhr
    }
    ///0x1a4 - MDF delay control register 2
    #[inline(always)]
    pub const fn dly2cr(&self) -> &DLY2CR {
        &self.dly2cr
    }
    ///0x1a8 - MDF short circuit detector control register 2
    #[inline(always)]
    pub const fn scd2cr(&self) -> &SCD2CR {
        &self.scd2cr
    }
    ///0x1ac - MDF DFLT2 interrupt enable register 2
    #[inline(always)]
    pub const fn dflt2ier(&self) -> &DFLT2IER {
        &self.dflt2ier
    }
    ///0x1b0 - MDF DFLT2 interrupt status register 2
    #[inline(always)]
    pub const fn dflt2isr(&self) -> &DFLT2ISR {
        &self.dflt2isr
    }
    ///0x1b4 - MDF offset error compensation control register 2
    #[inline(always)]
    pub const fn oec2cr(&self) -> &OEC2CR {
        &self.oec2cr
    }
    ///0x1ec - MDF snapshot data register 2
    #[inline(always)]
    pub const fn snps2dr(&self) -> &SNPS2DR {
        &self.snps2dr
    }
    ///0x1f0 - MDF digital filter data register 2
    #[inline(always)]
    pub const fn dflt2dr(&self) -> &DFLT2DR {
        &self.dflt2dr
    }
    ///0x200 - MDF serial interface control register 3
    #[inline(always)]
    pub const fn sitf3cr(&self) -> &SITF3CR {
        &self.sitf3cr
    }
    ///0x204 - MDF bitstream matrix control register 3
    #[inline(always)]
    pub const fn bsmx3cr(&self) -> &BSMX3CR {
        &self.bsmx3cr
    }
    ///0x208 - MDF digital filter control register 3
    #[inline(always)]
    pub const fn dflt3cr(&self) -> &DFLT3CR {
        &self.dflt3cr
    }
    ///0x20c - MDF digital filter configuration register 3
    #[inline(always)]
    pub const fn dflt3cicr(&self) -> &DFLT3CICR {
        &self.dflt3cicr
    }
    ///0x210 - MDF reshape filter configuration register 3
    #[inline(always)]
    pub const fn dflt3rsfr(&self) -> &DFLT3RSFR {
        &self.dflt3rsfr
    }
    ///0x214 - MDF integrator configuration register 3
    #[inline(always)]
    pub const fn dflt3intr(&self) -> &DFLT3INTR {
        &self.dflt3intr
    }
    ///0x218 - MDF out-of limit detector control register 3
    #[inline(always)]
    pub const fn old3cr(&self) -> &OLD3CR {
        &self.old3cr
    }
    ///0x21c - MDF OLD3 low threshold register 3
    #[inline(always)]
    pub const fn old3thlr(&self) -> &OLD3THLR {
        &self.old3thlr
    }
    ///0x220 - MDF OLD3 high threshold register 3
    #[inline(always)]
    pub const fn old3thhr(&self) -> &OLD3THHR {
        &self.old3thhr
    }
    ///0x224 - MDF delay control register 3
    #[inline(always)]
    pub const fn dly3cr(&self) -> &DLY3CR {
        &self.dly3cr
    }
    ///0x228 - MDF short circuit detector control register 3
    #[inline(always)]
    pub const fn scd3cr(&self) -> &SCD3CR {
        &self.scd3cr
    }
    ///0x22c - MDF DFLT3 interrupt enable register 3
    #[inline(always)]
    pub const fn dflt3ier(&self) -> &DFLT3IER {
        &self.dflt3ier
    }
    ///0x230 - MDF DFLT3 interrupt status register 3
    #[inline(always)]
    pub const fn dflt3isr(&self) -> &DFLT3ISR {
        &self.dflt3isr
    }
    ///0x234 - MDF offset error compensation control register 3
    #[inline(always)]
    pub const fn oec3cr(&self) -> &OEC3CR {
        &self.oec3cr
    }
    ///0x26c - MDF snapshot data register 3
    #[inline(always)]
    pub const fn snps3dr(&self) -> &SNPS3DR {
        &self.snps3dr
    }
    ///0x270 - MDF digital filter data register 3
    #[inline(always)]
    pub const fn dflt3dr(&self) -> &DFLT3DR {
        &self.dflt3dr
    }
    ///0x280 - MDF serial interface control register 4
    #[inline(always)]
    pub const fn sitf4cr(&self) -> &SITF4CR {
        &self.sitf4cr
    }
    ///0x284 - MDF bitstream matrix control register 4
    #[inline(always)]
    pub const fn bsmx4cr(&self) -> &BSMX4CR {
        &self.bsmx4cr
    }
    ///0x288 - MDF digital filter control register 4
    #[inline(always)]
    pub const fn dflt4cr(&self) -> &DFLT4CR {
        &self.dflt4cr
    }
    ///0x28c - MDF digital filter configuration register 4
    #[inline(always)]
    pub const fn dflt4cicr(&self) -> &DFLT4CICR {
        &self.dflt4cicr
    }
    ///0x290 - MDF reshape filter configuration register 4
    #[inline(always)]
    pub const fn dflt4rsfr(&self) -> &DFLT4RSFR {
        &self.dflt4rsfr
    }
    ///0x294 - MDF integrator configuration register 4
    #[inline(always)]
    pub const fn dflt4intr(&self) -> &DFLT4INTR {
        &self.dflt4intr
    }
    ///0x298 - MDF out-of limit detector control register 4
    #[inline(always)]
    pub const fn old4cr(&self) -> &OLD4CR {
        &self.old4cr
    }
    ///0x29c - MDF OLD4 low threshold register 4
    #[inline(always)]
    pub const fn old4thlr(&self) -> &OLD4THLR {
        &self.old4thlr
    }
    ///0x2a0 - MDF OLD4 high threshold register 4
    #[inline(always)]
    pub const fn old4thhr(&self) -> &OLD4THHR {
        &self.old4thhr
    }
    ///0x2a4 - MDF delay control register 4
    #[inline(always)]
    pub const fn dly4cr(&self) -> &DLY4CR {
        &self.dly4cr
    }
    ///0x2a8 - MDF short circuit detector control register 4
    #[inline(always)]
    pub const fn scd4cr(&self) -> &SCD4CR {
        &self.scd4cr
    }
    ///0x2ac - MDF DFLT4 interrupt enable register 4
    #[inline(always)]
    pub const fn dflt4ier(&self) -> &DFLT4IER {
        &self.dflt4ier
    }
    ///0x2b0 - MDF DFLT4 interrupt status register 4
    #[inline(always)]
    pub const fn dflt4isr(&self) -> &DFLT4ISR {
        &self.dflt4isr
    }
    ///0x2b4 - MDF offset error compensation control register 4
    #[inline(always)]
    pub const fn oec4cr(&self) -> &OEC4CR {
        &self.oec4cr
    }
    ///0x2ec - MDF snapshot data register 4
    #[inline(always)]
    pub const fn snps4dr(&self) -> &SNPS4DR {
        &self.snps4dr
    }
    ///0x2f0 - MDF digital filter data register 4
    #[inline(always)]
    pub const fn dflt4dr(&self) -> &DFLT4DR {
        &self.dflt4dr
    }
    ///0x300 - MDF serial interface control register 5
    #[inline(always)]
    pub const fn sitf5cr(&self) -> &SITF5CR {
        &self.sitf5cr
    }
    ///0x304 - MDF bitstream matrix control register 5
    #[inline(always)]
    pub const fn bsmx5cr(&self) -> &BSMX5CR {
        &self.bsmx5cr
    }
    ///0x308 - MDF digital filter control register 5
    #[inline(always)]
    pub const fn dflt5cr(&self) -> &DFLT5CR {
        &self.dflt5cr
    }
    ///0x30c - MDF digital filter configuration register 5
    #[inline(always)]
    pub const fn dflt5cicr(&self) -> &DFLT5CICR {
        &self.dflt5cicr
    }
    ///0x310 - MDF reshape filter configuration register 5
    #[inline(always)]
    pub const fn dflt5rsfr(&self) -> &DFLT5RSFR {
        &self.dflt5rsfr
    }
    ///0x314 - MDF integrator configuration register 5
    #[inline(always)]
    pub const fn dflt5intr(&self) -> &DFLT5INTR {
        &self.dflt5intr
    }
    ///0x318 - MDF out-of limit detector control register 5
    #[inline(always)]
    pub const fn old5cr(&self) -> &OLD5CR {
        &self.old5cr
    }
    ///0x31c - MDF OLD5 low threshold register 5
    #[inline(always)]
    pub const fn old5thlr(&self) -> &OLD5THLR {
        &self.old5thlr
    }
    ///0x320 - MDF OLD5 high threshold register 5
    #[inline(always)]
    pub const fn old5thhr(&self) -> &OLD5THHR {
        &self.old5thhr
    }
    ///0x324 - MDF delay control register 5
    #[inline(always)]
    pub const fn dly5cr(&self) -> &DLY5CR {
        &self.dly5cr
    }
    ///0x328 - MDF short circuit detector control register 5
    #[inline(always)]
    pub const fn scd5cr(&self) -> &SCD5CR {
        &self.scd5cr
    }
    ///0x32c - MDF DFLT5 interrupt enable register 5
    #[inline(always)]
    pub const fn dflt5ier(&self) -> &DFLT5IER {
        &self.dflt5ier
    }
    ///0x330 - MDF DFLT5 interrupt status register 5
    #[inline(always)]
    pub const fn dflt5isr(&self) -> &DFLT5ISR {
        &self.dflt5isr
    }
    ///0x334 - MDF offset error compensation control register 5
    #[inline(always)]
    pub const fn oec5cr(&self) -> &OEC5CR {
        &self.oec5cr
    }
    ///0x36c - MDF snapshot data register 5
    #[inline(always)]
    pub const fn snps5dr(&self) -> &SNPS5DR {
        &self.snps5dr
    }
    ///0x370 - MDF digital filter data register 5
    #[inline(always)]
    pub const fn dflt5dr(&self) -> &DFLT5DR {
        &self.dflt5dr
    }
}
/**GCR (rw) register accessor: MDF global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///MDF global control register
pub mod gcr;
/**CKGCR (rw) register accessor: MDF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`ckgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:CKGCR)

For information about available fields see [`mod@ckgcr`] module*/
pub type CKGCR = crate::Reg<ckgcr::CKGCRrs>;
///MDF clock generator control register
pub mod ckgcr;
/**SITF0CR (rw) register accessor: MDF serial interface control register 0

You can [`read`](crate::Reg::read) this register and get [`sitf0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF0CR)

For information about available fields see [`mod@sitf0cr`] module*/
pub type SITF0CR = crate::Reg<sitf0cr::SITF0CRrs>;
///MDF serial interface control register 0
pub mod sitf0cr;
/**BSMX0CR (rw) register accessor: MDF bitstream matrix control register 0

You can [`read`](crate::Reg::read) this register and get [`bsmx0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX0CR)

For information about available fields see [`mod@bsmx0cr`] module*/
pub type BSMX0CR = crate::Reg<bsmx0cr::BSMX0CRrs>;
///MDF bitstream matrix control register 0
pub mod bsmx0cr;
/**DFLT0CR (rw) register accessor: MDF digital filter control register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0CR)

For information about available fields see [`mod@dflt0cr`] module*/
pub type DFLT0CR = crate::Reg<dflt0cr::DFLT0CRrs>;
///MDF digital filter control register 0
pub mod dflt0cr;
/**DFLT0CICR (rw) register accessor: MDF digital filter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0CICR)

For information about available fields see [`mod@dflt0cicr`] module*/
pub type DFLT0CICR = crate::Reg<dflt0cicr::DFLT0CICRrs>;
///MDF digital filter configuration register 0
pub mod dflt0cicr;
/**DFLT0RSFR (rw) register accessor: MDF reshape filter configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0RSFR)

For information about available fields see [`mod@dflt0rsfr`] module*/
pub type DFLT0RSFR = crate::Reg<dflt0rsfr::DFLT0RSFRrs>;
///MDF reshape filter configuration register 0
pub mod dflt0rsfr;
/**DFLT0INTR (rw) register accessor: MDF integrator configuration register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0INTR)

For information about available fields see [`mod@dflt0intr`] module*/
pub type DFLT0INTR = crate::Reg<dflt0intr::DFLT0INTRrs>;
///MDF integrator configuration register 0
pub mod dflt0intr;
/**OLD0CR (rw) register accessor: MDF out-of limit detector control register 0

You can [`read`](crate::Reg::read) this register and get [`old0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD0CR)

For information about available fields see [`mod@old0cr`] module*/
pub type OLD0CR = crate::Reg<old0cr::OLD0CRrs>;
///MDF out-of limit detector control register 0
pub mod old0cr;
/**OLD0THLR (rw) register accessor: MDF OLD0 low threshold register 0

You can [`read`](crate::Reg::read) this register and get [`old0thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD0THLR)

For information about available fields see [`mod@old0thlr`] module*/
pub type OLD0THLR = crate::Reg<old0thlr::OLD0THLRrs>;
///MDF OLD0 low threshold register 0
pub mod old0thlr;
/**OLD0THHR (rw) register accessor: MDF OLD0 high threshold register 0

You can [`read`](crate::Reg::read) this register and get [`old0thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old0thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD0THHR)

For information about available fields see [`mod@old0thhr`] module*/
pub type OLD0THHR = crate::Reg<old0thhr::OLD0THHRrs>;
///MDF OLD0 high threshold register 0
pub mod old0thhr;
/**DLY0CR (rw) register accessor: MDF delay control register 0

You can [`read`](crate::Reg::read) this register and get [`dly0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY0CR)

For information about available fields see [`mod@dly0cr`] module*/
pub type DLY0CR = crate::Reg<dly0cr::DLY0CRrs>;
///MDF delay control register 0
pub mod dly0cr;
/**SCD0CR (rw) register accessor: MDF short circuit detector control register 0

You can [`read`](crate::Reg::read) this register and get [`scd0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD0CR)

For information about available fields see [`mod@scd0cr`] module*/
pub type SCD0CR = crate::Reg<scd0cr::SCD0CRrs>;
///MDF short circuit detector control register 0
pub mod scd0cr;
/**DFLT0IER (rw) register accessor: MDF DFLT0 interrupt enable register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0IER)

For information about available fields see [`mod@dflt0ier`] module*/
pub type DFLT0IER = crate::Reg<dflt0ier::DFLT0IERrs>;
///MDF DFLT0 interrupt enable register 0
pub mod dflt0ier;
/**DFLT0ISR (rw) register accessor: MDF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt0isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0ISR)

For information about available fields see [`mod@dflt0isr`] module*/
pub type DFLT0ISR = crate::Reg<dflt0isr::DFLT0ISRrs>;
///MDF DFLT0 interrupt status register 0
pub mod dflt0isr;
/**OEC0CR (rw) register accessor: MDF offset error compensation control register 0

You can [`read`](crate::Reg::read) this register and get [`oec0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC0CR)

For information about available fields see [`mod@oec0cr`] module*/
pub type OEC0CR = crate::Reg<oec0cr::OEC0CRrs>;
///MDF offset error compensation control register 0
pub mod oec0cr;
/**SNPS0DR (r) register accessor: MDF snapshot data register 0

You can [`read`](crate::Reg::read) this register and get [`snps0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS0DR)

For information about available fields see [`mod@snps0dr`] module*/
pub type SNPS0DR = crate::Reg<snps0dr::SNPS0DRrs>;
///MDF snapshot data register 0
pub mod snps0dr;
/**DFLT0DR (r) register accessor: MDF digital filter data register 0

You can [`read`](crate::Reg::read) this register and get [`dflt0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT0DR)

For information about available fields see [`mod@dflt0dr`] module*/
pub type DFLT0DR = crate::Reg<dflt0dr::DFLT0DRrs>;
///MDF digital filter data register 0
pub mod dflt0dr;
/**SITF1CR (rw) register accessor: MDF serial interface control register 1

You can [`read`](crate::Reg::read) this register and get [`sitf1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF1CR)

For information about available fields see [`mod@sitf1cr`] module*/
pub type SITF1CR = crate::Reg<sitf1cr::SITF1CRrs>;
///MDF serial interface control register 1
pub mod sitf1cr;
/**BSMX1CR (rw) register accessor: MDF bitstream matrix control register 1

You can [`read`](crate::Reg::read) this register and get [`bsmx1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX1CR)

For information about available fields see [`mod@bsmx1cr`] module*/
pub type BSMX1CR = crate::Reg<bsmx1cr::BSMX1CRrs>;
///MDF bitstream matrix control register 1
pub mod bsmx1cr;
/**DFLT1CR (rw) register accessor: MDF digital filter control register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1CR)

For information about available fields see [`mod@dflt1cr`] module*/
pub type DFLT1CR = crate::Reg<dflt1cr::DFLT1CRrs>;
///MDF digital filter control register 1
pub mod dflt1cr;
/**DFLT1CICR (rw) register accessor: MDF digital filter configuration register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1CICR)

For information about available fields see [`mod@dflt1cicr`] module*/
pub type DFLT1CICR = crate::Reg<dflt1cicr::DFLT1CICRrs>;
///MDF digital filter configuration register 1
pub mod dflt1cicr;
/**DFLT1RSFR (rw) register accessor: MDF reshape filter configuration register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1RSFR)

For information about available fields see [`mod@dflt1rsfr`] module*/
pub type DFLT1RSFR = crate::Reg<dflt1rsfr::DFLT1RSFRrs>;
///MDF reshape filter configuration register 1
pub mod dflt1rsfr;
/**DFLT1INTR (rw) register accessor: MDF integrator configuration register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1INTR)

For information about available fields see [`mod@dflt1intr`] module*/
pub type DFLT1INTR = crate::Reg<dflt1intr::DFLT1INTRrs>;
///MDF integrator configuration register 1
pub mod dflt1intr;
/**OLD1CR (rw) register accessor: MDF out-of limit detector control register 1

You can [`read`](crate::Reg::read) this register and get [`old1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD1CR)

For information about available fields see [`mod@old1cr`] module*/
pub type OLD1CR = crate::Reg<old1cr::OLD1CRrs>;
///MDF out-of limit detector control register 1
pub mod old1cr;
/**OLD1THLR (rw) register accessor: MDF OLD1 low threshold register 1

You can [`read`](crate::Reg::read) this register and get [`old1thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old1thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD1THLR)

For information about available fields see [`mod@old1thlr`] module*/
pub type OLD1THLR = crate::Reg<old1thlr::OLD1THLRrs>;
///MDF OLD1 low threshold register 1
pub mod old1thlr;
/**OLD1THHR (rw) register accessor: MDF OLD1 high threshold register 1

You can [`read`](crate::Reg::read) this register and get [`old1thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old1thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD1THHR)

For information about available fields see [`mod@old1thhr`] module*/
pub type OLD1THHR = crate::Reg<old1thhr::OLD1THHRrs>;
///MDF OLD1 high threshold register 1
pub mod old1thhr;
/**DLY1CR (rw) register accessor: MDF delay control register 1

You can [`read`](crate::Reg::read) this register and get [`dly1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY1CR)

For information about available fields see [`mod@dly1cr`] module*/
pub type DLY1CR = crate::Reg<dly1cr::DLY1CRrs>;
///MDF delay control register 1
pub mod dly1cr;
/**SCD1CR (rw) register accessor: MDF short circuit detector control register 1

You can [`read`](crate::Reg::read) this register and get [`scd1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD1CR)

For information about available fields see [`mod@scd1cr`] module*/
pub type SCD1CR = crate::Reg<scd1cr::SCD1CRrs>;
///MDF short circuit detector control register 1
pub mod scd1cr;
/**DFLT1IER (rw) register accessor: MDF DFLT1 interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1IER)

For information about available fields see [`mod@dflt1ier`] module*/
pub type DFLT1IER = crate::Reg<dflt1ier::DFLT1IERrs>;
///MDF DFLT1 interrupt enable register 1
pub mod dflt1ier;
/**DFLT1ISR (rw) register accessor: MDF DFLT1 interrupt status register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt1isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1ISR)

For information about available fields see [`mod@dflt1isr`] module*/
pub type DFLT1ISR = crate::Reg<dflt1isr::DFLT1ISRrs>;
///MDF DFLT1 interrupt status register 1
pub mod dflt1isr;
/**OEC1CR (rw) register accessor: MDF offset error compensation control register 1

You can [`read`](crate::Reg::read) this register and get [`oec1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC1CR)

For information about available fields see [`mod@oec1cr`] module*/
pub type OEC1CR = crate::Reg<oec1cr::OEC1CRrs>;
///MDF offset error compensation control register 1
pub mod oec1cr;
/**SNPS1DR (r) register accessor: MDF snapshot data register 1

You can [`read`](crate::Reg::read) this register and get [`snps1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS1DR)

For information about available fields see [`mod@snps1dr`] module*/
pub type SNPS1DR = crate::Reg<snps1dr::SNPS1DRrs>;
///MDF snapshot data register 1
pub mod snps1dr;
/**DFLT1DR (r) register accessor: MDF digital filter data register 1

You can [`read`](crate::Reg::read) this register and get [`dflt1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT1DR)

For information about available fields see [`mod@dflt1dr`] module*/
pub type DFLT1DR = crate::Reg<dflt1dr::DFLT1DRrs>;
///MDF digital filter data register 1
pub mod dflt1dr;
/**SITF2CR (rw) register accessor: MDF serial interface control register 2

You can [`read`](crate::Reg::read) this register and get [`sitf2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF2CR)

For information about available fields see [`mod@sitf2cr`] module*/
pub type SITF2CR = crate::Reg<sitf2cr::SITF2CRrs>;
///MDF serial interface control register 2
pub mod sitf2cr;
/**BSMX2CR (rw) register accessor: MDF bitstream matrix control register 2

You can [`read`](crate::Reg::read) this register and get [`bsmx2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX2CR)

For information about available fields see [`mod@bsmx2cr`] module*/
pub type BSMX2CR = crate::Reg<bsmx2cr::BSMX2CRrs>;
///MDF bitstream matrix control register 2
pub mod bsmx2cr;
/**DFLT2CR (rw) register accessor: MDF digital filter control register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2CR)

For information about available fields see [`mod@dflt2cr`] module*/
pub type DFLT2CR = crate::Reg<dflt2cr::DFLT2CRrs>;
///MDF digital filter control register 2
pub mod dflt2cr;
/**DFLT2CICR (rw) register accessor: MDF digital filter configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2CICR)

For information about available fields see [`mod@dflt2cicr`] module*/
pub type DFLT2CICR = crate::Reg<dflt2cicr::DFLT2CICRrs>;
///MDF digital filter configuration register 2
pub mod dflt2cicr;
/**DFLT2RSFR (rw) register accessor: MDF reshape filter configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2RSFR)

For information about available fields see [`mod@dflt2rsfr`] module*/
pub type DFLT2RSFR = crate::Reg<dflt2rsfr::DFLT2RSFRrs>;
///MDF reshape filter configuration register 2
pub mod dflt2rsfr;
/**DFLT2INTR (rw) register accessor: MDF integrator configuration register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2INTR)

For information about available fields see [`mod@dflt2intr`] module*/
pub type DFLT2INTR = crate::Reg<dflt2intr::DFLT2INTRrs>;
///MDF integrator configuration register 2
pub mod dflt2intr;
/**OLD2CR (rw) register accessor: MDF out-of limit detector control register 2

You can [`read`](crate::Reg::read) this register and get [`old2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD2CR)

For information about available fields see [`mod@old2cr`] module*/
pub type OLD2CR = crate::Reg<old2cr::OLD2CRrs>;
///MDF out-of limit detector control register 2
pub mod old2cr;
/**OLD2THLR (rw) register accessor: MDF OLD2 low threshold register 2

You can [`read`](crate::Reg::read) this register and get [`old2thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old2thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD2THLR)

For information about available fields see [`mod@old2thlr`] module*/
pub type OLD2THLR = crate::Reg<old2thlr::OLD2THLRrs>;
///MDF OLD2 low threshold register 2
pub mod old2thlr;
/**OLD2THHR (rw) register accessor: MDF OLD2 high threshold register 2

You can [`read`](crate::Reg::read) this register and get [`old2thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old2thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD2THHR)

For information about available fields see [`mod@old2thhr`] module*/
pub type OLD2THHR = crate::Reg<old2thhr::OLD2THHRrs>;
///MDF OLD2 high threshold register 2
pub mod old2thhr;
/**DLY2CR (rw) register accessor: MDF delay control register 2

You can [`read`](crate::Reg::read) this register and get [`dly2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY2CR)

For information about available fields see [`mod@dly2cr`] module*/
pub type DLY2CR = crate::Reg<dly2cr::DLY2CRrs>;
///MDF delay control register 2
pub mod dly2cr;
/**SCD2CR (rw) register accessor: MDF short circuit detector control register 2

You can [`read`](crate::Reg::read) this register and get [`scd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD2CR)

For information about available fields see [`mod@scd2cr`] module*/
pub type SCD2CR = crate::Reg<scd2cr::SCD2CRrs>;
///MDF short circuit detector control register 2
pub mod scd2cr;
/**DFLT2IER (rw) register accessor: MDF DFLT2 interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2IER)

For information about available fields see [`mod@dflt2ier`] module*/
pub type DFLT2IER = crate::Reg<dflt2ier::DFLT2IERrs>;
///MDF DFLT2 interrupt enable register 2
pub mod dflt2ier;
/**DFLT2ISR (rw) register accessor: MDF DFLT2 interrupt status register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt2isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2ISR)

For information about available fields see [`mod@dflt2isr`] module*/
pub type DFLT2ISR = crate::Reg<dflt2isr::DFLT2ISRrs>;
///MDF DFLT2 interrupt status register 2
pub mod dflt2isr;
/**OEC2CR (rw) register accessor: MDF offset error compensation control register 2

You can [`read`](crate::Reg::read) this register and get [`oec2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC2CR)

For information about available fields see [`mod@oec2cr`] module*/
pub type OEC2CR = crate::Reg<oec2cr::OEC2CRrs>;
///MDF offset error compensation control register 2
pub mod oec2cr;
/**SNPS2DR (r) register accessor: MDF snapshot data register 2

You can [`read`](crate::Reg::read) this register and get [`snps2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS2DR)

For information about available fields see [`mod@snps2dr`] module*/
pub type SNPS2DR = crate::Reg<snps2dr::SNPS2DRrs>;
///MDF snapshot data register 2
pub mod snps2dr;
/**DFLT2DR (r) register accessor: MDF digital filter data register 2

You can [`read`](crate::Reg::read) this register and get [`dflt2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT2DR)

For information about available fields see [`mod@dflt2dr`] module*/
pub type DFLT2DR = crate::Reg<dflt2dr::DFLT2DRrs>;
///MDF digital filter data register 2
pub mod dflt2dr;
/**SITF3CR (rw) register accessor: MDF serial interface control register 3

You can [`read`](crate::Reg::read) this register and get [`sitf3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF3CR)

For information about available fields see [`mod@sitf3cr`] module*/
pub type SITF3CR = crate::Reg<sitf3cr::SITF3CRrs>;
///MDF serial interface control register 3
pub mod sitf3cr;
/**BSMX3CR (rw) register accessor: MDF bitstream matrix control register 3

You can [`read`](crate::Reg::read) this register and get [`bsmx3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX3CR)

For information about available fields see [`mod@bsmx3cr`] module*/
pub type BSMX3CR = crate::Reg<bsmx3cr::BSMX3CRrs>;
///MDF bitstream matrix control register 3
pub mod bsmx3cr;
/**DFLT3CR (rw) register accessor: MDF digital filter control register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3CR)

For information about available fields see [`mod@dflt3cr`] module*/
pub type DFLT3CR = crate::Reg<dflt3cr::DFLT3CRrs>;
///MDF digital filter control register 3
pub mod dflt3cr;
/**DFLT3CICR (rw) register accessor: MDF digital filter configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3CICR)

For information about available fields see [`mod@dflt3cicr`] module*/
pub type DFLT3CICR = crate::Reg<dflt3cicr::DFLT3CICRrs>;
///MDF digital filter configuration register 3
pub mod dflt3cicr;
/**DFLT3RSFR (rw) register accessor: MDF reshape filter configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3RSFR)

For information about available fields see [`mod@dflt3rsfr`] module*/
pub type DFLT3RSFR = crate::Reg<dflt3rsfr::DFLT3RSFRrs>;
///MDF reshape filter configuration register 3
pub mod dflt3rsfr;
/**DFLT3INTR (rw) register accessor: MDF integrator configuration register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3INTR)

For information about available fields see [`mod@dflt3intr`] module*/
pub type DFLT3INTR = crate::Reg<dflt3intr::DFLT3INTRrs>;
///MDF integrator configuration register 3
pub mod dflt3intr;
/**OLD3CR (rw) register accessor: MDF out-of limit detector control register 3

You can [`read`](crate::Reg::read) this register and get [`old3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD3CR)

For information about available fields see [`mod@old3cr`] module*/
pub type OLD3CR = crate::Reg<old3cr::OLD3CRrs>;
///MDF out-of limit detector control register 3
pub mod old3cr;
/**OLD3THLR (rw) register accessor: MDF OLD3 low threshold register 3

You can [`read`](crate::Reg::read) this register and get [`old3thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old3thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD3THLR)

For information about available fields see [`mod@old3thlr`] module*/
pub type OLD3THLR = crate::Reg<old3thlr::OLD3THLRrs>;
///MDF OLD3 low threshold register 3
pub mod old3thlr;
/**OLD3THHR (rw) register accessor: MDF OLD3 high threshold register 3

You can [`read`](crate::Reg::read) this register and get [`old3thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old3thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD3THHR)

For information about available fields see [`mod@old3thhr`] module*/
pub type OLD3THHR = crate::Reg<old3thhr::OLD3THHRrs>;
///MDF OLD3 high threshold register 3
pub mod old3thhr;
/**DLY3CR (rw) register accessor: MDF delay control register 3

You can [`read`](crate::Reg::read) this register and get [`dly3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY3CR)

For information about available fields see [`mod@dly3cr`] module*/
pub type DLY3CR = crate::Reg<dly3cr::DLY3CRrs>;
///MDF delay control register 3
pub mod dly3cr;
/**SCD3CR (rw) register accessor: MDF short circuit detector control register 3

You can [`read`](crate::Reg::read) this register and get [`scd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD3CR)

For information about available fields see [`mod@scd3cr`] module*/
pub type SCD3CR = crate::Reg<scd3cr::SCD3CRrs>;
///MDF short circuit detector control register 3
pub mod scd3cr;
/**DFLT3IER (rw) register accessor: MDF DFLT3 interrupt enable register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3IER)

For information about available fields see [`mod@dflt3ier`] module*/
pub type DFLT3IER = crate::Reg<dflt3ier::DFLT3IERrs>;
///MDF DFLT3 interrupt enable register 3
pub mod dflt3ier;
/**DFLT3ISR (rw) register accessor: MDF DFLT3 interrupt status register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt3isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3ISR)

For information about available fields see [`mod@dflt3isr`] module*/
pub type DFLT3ISR = crate::Reg<dflt3isr::DFLT3ISRrs>;
///MDF DFLT3 interrupt status register 3
pub mod dflt3isr;
/**OEC3CR (rw) register accessor: MDF offset error compensation control register 3

You can [`read`](crate::Reg::read) this register and get [`oec3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC3CR)

For information about available fields see [`mod@oec3cr`] module*/
pub type OEC3CR = crate::Reg<oec3cr::OEC3CRrs>;
///MDF offset error compensation control register 3
pub mod oec3cr;
/**SNPS3DR (r) register accessor: MDF snapshot data register 3

You can [`read`](crate::Reg::read) this register and get [`snps3dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS3DR)

For information about available fields see [`mod@snps3dr`] module*/
pub type SNPS3DR = crate::Reg<snps3dr::SNPS3DRrs>;
///MDF snapshot data register 3
pub mod snps3dr;
/**DFLT3DR (r) register accessor: MDF digital filter data register 3

You can [`read`](crate::Reg::read) this register and get [`dflt3dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT3DR)

For information about available fields see [`mod@dflt3dr`] module*/
pub type DFLT3DR = crate::Reg<dflt3dr::DFLT3DRrs>;
///MDF digital filter data register 3
pub mod dflt3dr;
/**SITF4CR (rw) register accessor: MDF serial interface control register 4

You can [`read`](crate::Reg::read) this register and get [`sitf4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF4CR)

For information about available fields see [`mod@sitf4cr`] module*/
pub type SITF4CR = crate::Reg<sitf4cr::SITF4CRrs>;
///MDF serial interface control register 4
pub mod sitf4cr;
/**BSMX4CR (rw) register accessor: MDF bitstream matrix control register 4

You can [`read`](crate::Reg::read) this register and get [`bsmx4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX4CR)

For information about available fields see [`mod@bsmx4cr`] module*/
pub type BSMX4CR = crate::Reg<bsmx4cr::BSMX4CRrs>;
///MDF bitstream matrix control register 4
pub mod bsmx4cr;
/**DFLT4CR (rw) register accessor: MDF digital filter control register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4CR)

For information about available fields see [`mod@dflt4cr`] module*/
pub type DFLT4CR = crate::Reg<dflt4cr::DFLT4CRrs>;
///MDF digital filter control register 4
pub mod dflt4cr;
/**DFLT4CICR (rw) register accessor: MDF digital filter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4CICR)

For information about available fields see [`mod@dflt4cicr`] module*/
pub type DFLT4CICR = crate::Reg<dflt4cicr::DFLT4CICRrs>;
///MDF digital filter configuration register 4
pub mod dflt4cicr;
/**DFLT4RSFR (rw) register accessor: MDF reshape filter configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4RSFR)

For information about available fields see [`mod@dflt4rsfr`] module*/
pub type DFLT4RSFR = crate::Reg<dflt4rsfr::DFLT4RSFRrs>;
///MDF reshape filter configuration register 4
pub mod dflt4rsfr;
/**DFLT4INTR (rw) register accessor: MDF integrator configuration register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4INTR)

For information about available fields see [`mod@dflt4intr`] module*/
pub type DFLT4INTR = crate::Reg<dflt4intr::DFLT4INTRrs>;
///MDF integrator configuration register 4
pub mod dflt4intr;
/**OLD4CR (rw) register accessor: MDF out-of limit detector control register 4

You can [`read`](crate::Reg::read) this register and get [`old4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD4CR)

For information about available fields see [`mod@old4cr`] module*/
pub type OLD4CR = crate::Reg<old4cr::OLD4CRrs>;
///MDF out-of limit detector control register 4
pub mod old4cr;
/**OLD4THLR (rw) register accessor: MDF OLD4 low threshold register 4

You can [`read`](crate::Reg::read) this register and get [`old4thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old4thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD4THLR)

For information about available fields see [`mod@old4thlr`] module*/
pub type OLD4THLR = crate::Reg<old4thlr::OLD4THLRrs>;
///MDF OLD4 low threshold register 4
pub mod old4thlr;
/**OLD4THHR (rw) register accessor: MDF OLD4 high threshold register 4

You can [`read`](crate::Reg::read) this register and get [`old4thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old4thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD4THHR)

For information about available fields see [`mod@old4thhr`] module*/
pub type OLD4THHR = crate::Reg<old4thhr::OLD4THHRrs>;
///MDF OLD4 high threshold register 4
pub mod old4thhr;
/**DLY4CR (rw) register accessor: MDF delay control register 4

You can [`read`](crate::Reg::read) this register and get [`dly4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY4CR)

For information about available fields see [`mod@dly4cr`] module*/
pub type DLY4CR = crate::Reg<dly4cr::DLY4CRrs>;
///MDF delay control register 4
pub mod dly4cr;
/**SCD4CR (rw) register accessor: MDF short circuit detector control register 4

You can [`read`](crate::Reg::read) this register and get [`scd4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD4CR)

For information about available fields see [`mod@scd4cr`] module*/
pub type SCD4CR = crate::Reg<scd4cr::SCD4CRrs>;
///MDF short circuit detector control register 4
pub mod scd4cr;
/**DFLT4IER (rw) register accessor: MDF DFLT4 interrupt enable register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4IER)

For information about available fields see [`mod@dflt4ier`] module*/
pub type DFLT4IER = crate::Reg<dflt4ier::DFLT4IERrs>;
///MDF DFLT4 interrupt enable register 4
pub mod dflt4ier;
/**DFLT4ISR (rw) register accessor: MDF DFLT4 interrupt status register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt4isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4ISR)

For information about available fields see [`mod@dflt4isr`] module*/
pub type DFLT4ISR = crate::Reg<dflt4isr::DFLT4ISRrs>;
///MDF DFLT4 interrupt status register 4
pub mod dflt4isr;
/**OEC4CR (rw) register accessor: MDF offset error compensation control register 4

You can [`read`](crate::Reg::read) this register and get [`oec4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC4CR)

For information about available fields see [`mod@oec4cr`] module*/
pub type OEC4CR = crate::Reg<oec4cr::OEC4CRrs>;
///MDF offset error compensation control register 4
pub mod oec4cr;
/**SNPS4DR (r) register accessor: MDF snapshot data register 4

You can [`read`](crate::Reg::read) this register and get [`snps4dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS4DR)

For information about available fields see [`mod@snps4dr`] module*/
pub type SNPS4DR = crate::Reg<snps4dr::SNPS4DRrs>;
///MDF snapshot data register 4
pub mod snps4dr;
/**DFLT4DR (r) register accessor: MDF digital filter data register 4

You can [`read`](crate::Reg::read) this register and get [`dflt4dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT4DR)

For information about available fields see [`mod@dflt4dr`] module*/
pub type DFLT4DR = crate::Reg<dflt4dr::DFLT4DRrs>;
///MDF digital filter data register 4
pub mod dflt4dr;
/**SITF5CR (rw) register accessor: MDF serial interface control register 5

You can [`read`](crate::Reg::read) this register and get [`sitf5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sitf5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SITF5CR)

For information about available fields see [`mod@sitf5cr`] module*/
pub type SITF5CR = crate::Reg<sitf5cr::SITF5CRrs>;
///MDF serial interface control register 5
pub mod sitf5cr;
/**BSMX5CR (rw) register accessor: MDF bitstream matrix control register 5

You can [`read`](crate::Reg::read) this register and get [`bsmx5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bsmx5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:BSMX5CR)

For information about available fields see [`mod@bsmx5cr`] module*/
pub type BSMX5CR = crate::Reg<bsmx5cr::BSMX5CRrs>;
///MDF bitstream matrix control register 5
pub mod bsmx5cr;
/**DFLT5CR (rw) register accessor: MDF digital filter control register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5CR)

For information about available fields see [`mod@dflt5cr`] module*/
pub type DFLT5CR = crate::Reg<dflt5cr::DFLT5CRrs>;
///MDF digital filter control register 5
pub mod dflt5cr;
/**DFLT5CICR (rw) register accessor: MDF digital filter configuration register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5CICR)

For information about available fields see [`mod@dflt5cicr`] module*/
pub type DFLT5CICR = crate::Reg<dflt5cicr::DFLT5CICRrs>;
///MDF digital filter configuration register 5
pub mod dflt5cicr;
/**DFLT5RSFR (rw) register accessor: MDF reshape filter configuration register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5RSFR)

For information about available fields see [`mod@dflt5rsfr`] module*/
pub type DFLT5RSFR = crate::Reg<dflt5rsfr::DFLT5RSFRrs>;
///MDF reshape filter configuration register 5
pub mod dflt5rsfr;
/**DFLT5INTR (rw) register accessor: MDF integrator configuration register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5INTR)

For information about available fields see [`mod@dflt5intr`] module*/
pub type DFLT5INTR = crate::Reg<dflt5intr::DFLT5INTRrs>;
///MDF integrator configuration register 5
pub mod dflt5intr;
/**OLD5CR (rw) register accessor: MDF out-of limit detector control register 5

You can [`read`](crate::Reg::read) this register and get [`old5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD5CR)

For information about available fields see [`mod@old5cr`] module*/
pub type OLD5CR = crate::Reg<old5cr::OLD5CRrs>;
///MDF out-of limit detector control register 5
pub mod old5cr;
/**OLD5THLR (rw) register accessor: MDF OLD5 low threshold register 5

You can [`read`](crate::Reg::read) this register and get [`old5thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old5thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD5THLR)

For information about available fields see [`mod@old5thlr`] module*/
pub type OLD5THLR = crate::Reg<old5thlr::OLD5THLRrs>;
///MDF OLD5 low threshold register 5
pub mod old5thlr;
/**OLD5THHR (rw) register accessor: MDF OLD5 high threshold register 5

You can [`read`](crate::Reg::read) this register and get [`old5thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`old5thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OLD5THHR)

For information about available fields see [`mod@old5thhr`] module*/
pub type OLD5THHR = crate::Reg<old5thhr::OLD5THHRrs>;
///MDF OLD5 high threshold register 5
pub mod old5thhr;
/**DLY5CR (rw) register accessor: MDF delay control register 5

You can [`read`](crate::Reg::read) this register and get [`dly5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dly5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DLY5CR)

For information about available fields see [`mod@dly5cr`] module*/
pub type DLY5CR = crate::Reg<dly5cr::DLY5CRrs>;
///MDF delay control register 5
pub mod dly5cr;
/**SCD5CR (rw) register accessor: MDF short circuit detector control register 5

You can [`read`](crate::Reg::read) this register and get [`scd5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scd5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SCD5CR)

For information about available fields see [`mod@scd5cr`] module*/
pub type SCD5CR = crate::Reg<scd5cr::SCD5CRrs>;
///MDF short circuit detector control register 5
pub mod scd5cr;
/**DFLT5IER (rw) register accessor: MDF DFLT5 interrupt enable register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5IER)

For information about available fields see [`mod@dflt5ier`] module*/
pub type DFLT5IER = crate::Reg<dflt5ier::DFLT5IERrs>;
///MDF DFLT5 interrupt enable register 5
pub mod dflt5ier;
/**DFLT5ISR (rw) register accessor: MDF DFLT5 interrupt status register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dflt5isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5ISR)

For information about available fields see [`mod@dflt5isr`] module*/
pub type DFLT5ISR = crate::Reg<dflt5isr::DFLT5ISRrs>;
///MDF DFLT5 interrupt status register 5
pub mod dflt5isr;
/**OEC5CR (rw) register accessor: MDF offset error compensation control register 5

You can [`read`](crate::Reg::read) this register and get [`oec5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oec5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:OEC5CR)

For information about available fields see [`mod@oec5cr`] module*/
pub type OEC5CR = crate::Reg<oec5cr::OEC5CRrs>;
///MDF offset error compensation control register 5
pub mod oec5cr;
/**SNPS5DR (r) register accessor: MDF snapshot data register 5

You can [`read`](crate::Reg::read) this register and get [`snps5dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:SNPS5DR)

For information about available fields see [`mod@snps5dr`] module*/
pub type SNPS5DR = crate::Reg<snps5dr::SNPS5DRrs>;
///MDF snapshot data register 5
pub mod snps5dr;
/**DFLT5DR (r) register accessor: MDF digital filter data register 5

You can [`read`](crate::Reg::read) this register and get [`dflt5dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#MDF1:DFLT5DR)

For information about available fields see [`mod@dflt5dr`] module*/
pub type DFLT5DR = crate::Reg<dflt5dr::DFLT5DRrs>;
///MDF digital filter data register 5
pub mod dflt5dr;
