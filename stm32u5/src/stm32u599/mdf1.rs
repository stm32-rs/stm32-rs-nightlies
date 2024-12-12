#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gcr: GCR,
    ckgcr: CKGCR,
    _reserved2: [u8; 0x78],
    mdf_sitf0cr: MDF_SITF0CR,
    mdf_bsmx0cr: MDF_BSMX0CR,
    mdf_dflt0cr: MDF_DFLT0CR,
    mdf_dflt0cicr: MDF_DFLT0CICR,
    mdf_dflt0rsfr: MDF_DFLT0RSFR,
    mdf_dflt0intr: MDF_DFLT0INTR,
    mdf_old0cr: MDF_OLD0CR,
    mdf_old0thlr: MDF_OLD0THLR,
    mdf_old0thhr: MDF_OLD0THHR,
    mdf_dly0cr: MDF_DLY0CR,
    mdf_scd0cr: MDF_SCD0CR,
    mdf_dflt0ier: MDF_DFLT0IER,
    mdf_dflt0isr: MDF_DFLT0ISR,
    mdf_oec0cr: MDF_OEC0CR,
    _reserved16: [u8; 0x34],
    mdf_snps0dr: MDF_SNPS0DR,
    mdf_dflt0dr: MDF_DFLT0DR,
    _reserved18: [u8; 0x0c],
    mdf_sitf1cr: MDF_SITF1CR,
    mdf_bsmx1cr: MDF_BSMX1CR,
    mdf_dflt1cr: MDF_DFLT1CR,
    mdf_dflt1cicr: MDF_DFLT1CICR,
    mdf_dflt1rsfr: MDF_DFLT1RSFR,
    mdf_dflt1intr: MDF_DFLT1INTR,
    mdf_old1cr: MDF_OLD1CR,
    mdf_old1thlr: MDF_OLD1THLR,
    mdf_old1thhr: MDF_OLD1THHR,
    mdf_dly1cr: MDF_DLY1CR,
    mdf_scd1cr: MDF_SCD1CR,
    mdf_dflt1ier: MDF_DFLT1IER,
    mdf_dflt1isr: MDF_DFLT1ISR,
    mdf_oec1cr: MDF_OEC1CR,
    _reserved32: [u8; 0x34],
    mdf_snps1dr: MDF_SNPS1DR,
    mdf_dflt1dr: MDF_DFLT1DR,
    _reserved34: [u8; 0x0c],
    mdf_sitf2cr: MDF_SITF2CR,
    mdf_bsmx2cr: MDF_BSMX2CR,
    mdf_dflt2cr: MDF_DFLT2CR,
    mdf_dflt2cicr: MDF_DFLT2CICR,
    mdf_dflt2rsfr: MDF_DFLT2RSFR,
    mdf_dflt2intr: MDF_DFLT2INTR,
    mdf_old2cr: MDF_OLD2CR,
    mdf_old2thlr: MDF_OLD2THLR,
    mdf_old2thhr: MDF_OLD2THHR,
    mdf_dly2cr: MDF_DLY2CR,
    mdf_scd2cr: MDF_SCD2CR,
    mdf_dflt2ier: MDF_DFLT2IER,
    mdf_dflt2isr: MDF_DFLT2ISR,
    mdf_oec2cr: MDF_OEC2CR,
    _reserved48: [u8; 0x34],
    mdf_snps2dr: MDF_SNPS2DR,
    mdf_dflt2dr: MDF_DFLT2DR,
    _reserved50: [u8; 0x0c],
    mdf_sitf3cr: MDF_SITF3CR,
    mdf_bsmx3cr: MDF_BSMX3CR,
    mdf_dflt3cr: MDF_DFLT3CR,
    mdf_dflt3cicr: MDF_DFLT3CICR,
    mdf_dflt3rsfr: MDF_DFLT3RSFR,
    mdf_dflt3intr: MDF_DFLT3INTR,
    mdf_old3cr: MDF_OLD3CR,
    mdf_old3thlr: MDF_OLD3THLR,
    mdf_old3thhr: MDF_OLD3THHR,
    mdf_dly3cr: MDF_DLY3CR,
    mdf_scd3cr: MDF_SCD3CR,
    mdf_dflt3ier: MDF_DFLT3IER,
    mdf_dflt3isr: MDF_DFLT3ISR,
    mdf_oec3cr: MDF_OEC3CR,
    _reserved64: [u8; 0x34],
    mdf_snps3dr: MDF_SNPS3DR,
    mdf_dflt3dr: MDF_DFLT3DR,
    _reserved66: [u8; 0x0c],
    mdf_sitf4cr: MDF_SITF4CR,
    mdf_bsmx4cr: MDF_BSMX4CR,
    mdf_dflt4cr: MDF_DFLT4CR,
    mdf_dflt4cicr: MDF_DFLT4CICR,
    mdf_dflt4rsfr: MDF_DFLT4RSFR,
    mdf_dflt4intr: MDF_DFLT4INTR,
    mdf_old4cr: MDF_OLD4CR,
    mdf_old4thlr: MDF_OLD4THLR,
    mdf_old4thhr: MDF_OLD4THHR,
    mdf_dly4cr: MDF_DLY4CR,
    mdf_scd4cr: MDF_SCD4CR,
    mdf_dflt4ier: MDF_DFLT4IER,
    mdf_dflt4isr: MDF_DFLT4ISR,
    mdf_oec4cr: MDF_OEC4CR,
    _reserved80: [u8; 0x34],
    mdf_snps4dr: MDF_SNPS4DR,
    mdf_dflt4dr: MDF_DFLT4DR,
    _reserved82: [u8; 0x0c],
    mdf_sitf5cr: MDF_SITF5CR,
    mdf_bsmx5cr: MDF_BSMX5CR,
    mdf_dflt5cr: MDF_DFLT5CR,
    mdf_dflt5cicr: MDF_DFLT5CICR,
    mdf_dflt5rsfr: MDF_DFLT5RSFR,
    mdf_dflt5intr: MDF_DFLT5INTR,
    mdf_old5cr: MDF_OLD5CR,
    mdf_old5thlr: MDF_OLD5THLR,
    mdf_old5thhr: MDF_OLD5THHR,
    mdf_dly5cr: MDF_DLY5CR,
    mdf_scd5cr: MDF_SCD5CR,
    mdf_dflt5ier: MDF_DFLT5IER,
    mdf_dflt5isr: MDF_DFLT5ISR,
    mdf_oec5cr: MDF_OEC5CR,
    _reserved96: [u8; 0x34],
    mdf_snps5dr: MDF_SNPS5DR,
    mdf_dflt5dr: MDF_DFLT5DR,
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
    ///0x80 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf0cr(&self) -> &MDF_SITF0CR {
        &self.mdf_sitf0cr
    }
    ///0x84 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx0cr(&self) -> &MDF_BSMX0CR {
        &self.mdf_bsmx0cr
    }
    ///0x88 - This register is used to control the digital filter x.
    #[inline(always)]
    pub const fn mdf_dflt0cr(&self) -> &MDF_DFLT0CR {
        &self.mdf_dflt0cr
    }
    ///0x8c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt0cicr(&self) -> &MDF_DFLT0CICR {
        &self.mdf_dflt0cicr
    }
    ///0x90 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt0rsfr(&self) -> &MDF_DFLT0RSFR {
        &self.mdf_dflt0rsfr
    }
    ///0x94 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt0intr(&self) -> &MDF_DFLT0INTR {
        &self.mdf_dflt0intr
    }
    ///0x98 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old0cr(&self) -> &MDF_OLD0CR {
        &self.mdf_old0cr
    }
    ///0x9c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old0thlr(&self) -> &MDF_OLD0THLR {
        &self.mdf_old0thlr
    }
    ///0xa0 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old0thhr(&self) -> &MDF_OLD0THHR {
        &self.mdf_old0thhr
    }
    ///0xa4 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly0cr(&self) -> &MDF_DLY0CR {
        &self.mdf_dly0cr
    }
    ///0xa8 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd0cr(&self) -> &MDF_SCD0CR {
        &self.mdf_scd0cr
    }
    ///0xac - This register is used for allowing or not the events to generate an interrupt.
    #[inline(always)]
    pub const fn mdf_dflt0ier(&self) -> &MDF_DFLT0IER {
        &self.mdf_dflt0ier
    }
    ///0xb0 - MDF DFLT0 interrupt status register 0
    #[inline(always)]
    pub const fn mdf_dflt0isr(&self) -> &MDF_DFLT0ISR {
        &self.mdf_dflt0isr
    }
    ///0xb4 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec0cr(&self) -> &MDF_OEC0CR {
        &self.mdf_oec0cr
    }
    ///0xec - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps0dr(&self) -> &MDF_SNPS0DR {
        &self.mdf_snps0dr
    }
    ///0xf0 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt0dr(&self) -> &MDF_DFLT0DR {
        &self.mdf_dflt0dr
    }
    ///0x100 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf1cr(&self) -> &MDF_SITF1CR {
        &self.mdf_sitf1cr
    }
    ///0x104 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx1cr(&self) -> &MDF_BSMX1CR {
        &self.mdf_bsmx1cr
    }
    ///0x108 - This register is used to control the digital filter x.
    #[inline(always)]
    pub const fn mdf_dflt1cr(&self) -> &MDF_DFLT1CR {
        &self.mdf_dflt1cr
    }
    ///0x10c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt1cicr(&self) -> &MDF_DFLT1CICR {
        &self.mdf_dflt1cicr
    }
    ///0x110 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt1rsfr(&self) -> &MDF_DFLT1RSFR {
        &self.mdf_dflt1rsfr
    }
    ///0x114 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt1intr(&self) -> &MDF_DFLT1INTR {
        &self.mdf_dflt1intr
    }
    ///0x118 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old1cr(&self) -> &MDF_OLD1CR {
        &self.mdf_old1cr
    }
    ///0x11c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old1thlr(&self) -> &MDF_OLD1THLR {
        &self.mdf_old1thlr
    }
    ///0x120 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old1thhr(&self) -> &MDF_OLD1THHR {
        &self.mdf_old1thhr
    }
    ///0x124 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly1cr(&self) -> &MDF_DLY1CR {
        &self.mdf_dly1cr
    }
    ///0x128 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd1cr(&self) -> &MDF_SCD1CR {
        &self.mdf_scd1cr
    }
    ///0x12c - MDF DFLTx interrupt enable register x
    #[inline(always)]
    pub const fn mdf_dflt1ier(&self) -> &MDF_DFLT1IER {
        &self.mdf_dflt1ier
    }
    ///0x130 - This register contains the status flags for each digital filter path.
    #[inline(always)]
    pub const fn mdf_dflt1isr(&self) -> &MDF_DFLT1ISR {
        &self.mdf_dflt1isr
    }
    ///0x134 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec1cr(&self) -> &MDF_OEC1CR {
        &self.mdf_oec1cr
    }
    ///0x16c - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps1dr(&self) -> &MDF_SNPS1DR {
        &self.mdf_snps1dr
    }
    ///0x170 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt1dr(&self) -> &MDF_DFLT1DR {
        &self.mdf_dflt1dr
    }
    ///0x180 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf2cr(&self) -> &MDF_SITF2CR {
        &self.mdf_sitf2cr
    }
    ///0x184 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx2cr(&self) -> &MDF_BSMX2CR {
        &self.mdf_bsmx2cr
    }
    ///0x188 - This register is used to control the digital filter 2.
    #[inline(always)]
    pub const fn mdf_dflt2cr(&self) -> &MDF_DFLT2CR {
        &self.mdf_dflt2cr
    }
    ///0x18c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt2cicr(&self) -> &MDF_DFLT2CICR {
        &self.mdf_dflt2cicr
    }
    ///0x190 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt2rsfr(&self) -> &MDF_DFLT2RSFR {
        &self.mdf_dflt2rsfr
    }
    ///0x194 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt2intr(&self) -> &MDF_DFLT2INTR {
        &self.mdf_dflt2intr
    }
    ///0x198 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old2cr(&self) -> &MDF_OLD2CR {
        &self.mdf_old2cr
    }
    ///0x19c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old2thlr(&self) -> &MDF_OLD2THLR {
        &self.mdf_old2thlr
    }
    ///0x1a0 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old2thhr(&self) -> &MDF_OLD2THHR {
        &self.mdf_old2thhr
    }
    ///0x1a4 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly2cr(&self) -> &MDF_DLY2CR {
        &self.mdf_dly2cr
    }
    ///0x1a8 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd2cr(&self) -> &MDF_SCD2CR {
        &self.mdf_scd2cr
    }
    ///0x1ac - MDF DFLTx interrupt enable register x
    #[inline(always)]
    pub const fn mdf_dflt2ier(&self) -> &MDF_DFLT2IER {
        &self.mdf_dflt2ier
    }
    ///0x1b0 - This register contains the status flags for each digital filter path.
    #[inline(always)]
    pub const fn mdf_dflt2isr(&self) -> &MDF_DFLT2ISR {
        &self.mdf_dflt2isr
    }
    ///0x1b4 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec2cr(&self) -> &MDF_OEC2CR {
        &self.mdf_oec2cr
    }
    ///0x1ec - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps2dr(&self) -> &MDF_SNPS2DR {
        &self.mdf_snps2dr
    }
    ///0x1f0 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt2dr(&self) -> &MDF_DFLT2DR {
        &self.mdf_dflt2dr
    }
    ///0x200 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf3cr(&self) -> &MDF_SITF3CR {
        &self.mdf_sitf3cr
    }
    ///0x204 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx3cr(&self) -> &MDF_BSMX3CR {
        &self.mdf_bsmx3cr
    }
    ///0x208 - This register is used to control the digital filter 3.
    #[inline(always)]
    pub const fn mdf_dflt3cr(&self) -> &MDF_DFLT3CR {
        &self.mdf_dflt3cr
    }
    ///0x20c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt3cicr(&self) -> &MDF_DFLT3CICR {
        &self.mdf_dflt3cicr
    }
    ///0x210 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt3rsfr(&self) -> &MDF_DFLT3RSFR {
        &self.mdf_dflt3rsfr
    }
    ///0x214 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt3intr(&self) -> &MDF_DFLT3INTR {
        &self.mdf_dflt3intr
    }
    ///0x218 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old3cr(&self) -> &MDF_OLD3CR {
        &self.mdf_old3cr
    }
    ///0x21c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old3thlr(&self) -> &MDF_OLD3THLR {
        &self.mdf_old3thlr
    }
    ///0x220 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old3thhr(&self) -> &MDF_OLD3THHR {
        &self.mdf_old3thhr
    }
    ///0x224 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly3cr(&self) -> &MDF_DLY3CR {
        &self.mdf_dly3cr
    }
    ///0x228 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd3cr(&self) -> &MDF_SCD3CR {
        &self.mdf_scd3cr
    }
    ///0x22c - MDF DFLTx interrupt enable register x
    #[inline(always)]
    pub const fn mdf_dflt3ier(&self) -> &MDF_DFLT3IER {
        &self.mdf_dflt3ier
    }
    ///0x230 - This register contains the status flags for each digital filter path.
    #[inline(always)]
    pub const fn mdf_dflt3isr(&self) -> &MDF_DFLT3ISR {
        &self.mdf_dflt3isr
    }
    ///0x234 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec3cr(&self) -> &MDF_OEC3CR {
        &self.mdf_oec3cr
    }
    ///0x26c - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps3dr(&self) -> &MDF_SNPS3DR {
        &self.mdf_snps3dr
    }
    ///0x270 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt3dr(&self) -> &MDF_DFLT3DR {
        &self.mdf_dflt3dr
    }
    ///0x280 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf4cr(&self) -> &MDF_SITF4CR {
        &self.mdf_sitf4cr
    }
    ///0x284 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx4cr(&self) -> &MDF_BSMX4CR {
        &self.mdf_bsmx4cr
    }
    ///0x288 - This register is used to control the digital filter 4.
    #[inline(always)]
    pub const fn mdf_dflt4cr(&self) -> &MDF_DFLT4CR {
        &self.mdf_dflt4cr
    }
    ///0x28c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt4cicr(&self) -> &MDF_DFLT4CICR {
        &self.mdf_dflt4cicr
    }
    ///0x290 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt4rsfr(&self) -> &MDF_DFLT4RSFR {
        &self.mdf_dflt4rsfr
    }
    ///0x294 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt4intr(&self) -> &MDF_DFLT4INTR {
        &self.mdf_dflt4intr
    }
    ///0x298 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old4cr(&self) -> &MDF_OLD4CR {
        &self.mdf_old4cr
    }
    ///0x29c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old4thlr(&self) -> &MDF_OLD4THLR {
        &self.mdf_old4thlr
    }
    ///0x2a0 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old4thhr(&self) -> &MDF_OLD4THHR {
        &self.mdf_old4thhr
    }
    ///0x2a4 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly4cr(&self) -> &MDF_DLY4CR {
        &self.mdf_dly4cr
    }
    ///0x2a8 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd4cr(&self) -> &MDF_SCD4CR {
        &self.mdf_scd4cr
    }
    ///0x2ac - MDF DFLTx interrupt enable register x
    #[inline(always)]
    pub const fn mdf_dflt4ier(&self) -> &MDF_DFLT4IER {
        &self.mdf_dflt4ier
    }
    ///0x2b0 - This register contains the status flags for each digital filter path.
    #[inline(always)]
    pub const fn mdf_dflt4isr(&self) -> &MDF_DFLT4ISR {
        &self.mdf_dflt4isr
    }
    ///0x2b4 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec4cr(&self) -> &MDF_OEC4CR {
        &self.mdf_oec4cr
    }
    ///0x2ec - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps4dr(&self) -> &MDF_SNPS4DR {
        &self.mdf_snps4dr
    }
    ///0x2f0 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt4dr(&self) -> &MDF_DFLT4DR {
        &self.mdf_dflt4dr
    }
    ///0x300 - This register is used to control the serial interfaces (SITFx).
    #[inline(always)]
    pub const fn mdf_sitf5cr(&self) -> &MDF_SITF5CR {
        &self.mdf_sitf5cr
    }
    ///0x304 - This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
    #[inline(always)]
    pub const fn mdf_bsmx5cr(&self) -> &MDF_BSMX5CR {
        &self.mdf_bsmx5cr
    }
    ///0x308 - This register is used to control the digital filter x.
    #[inline(always)]
    pub const fn mdf_dflt5cr(&self) -> &MDF_DFLT5CR {
        &self.mdf_dflt5cr
    }
    ///0x30c - This register is used to control the main CIC filter.
    #[inline(always)]
    pub const fn mdf_dflt5cicr(&self) -> &MDF_DFLT5CICR {
        &self.mdf_dflt5cicr
    }
    ///0x310 - This register is used to control the reshape and HPF filters.
    #[inline(always)]
    pub const fn mdf_dflt5rsfr(&self) -> &MDF_DFLT5RSFR {
        &self.mdf_dflt5rsfr
    }
    ///0x314 - This register is used to the integrator (INT) settings.
    #[inline(always)]
    pub const fn mdf_dflt5intr(&self) -> &MDF_DFLT5INTR {
        &self.mdf_dflt5intr
    }
    ///0x318 - This register is used to configure the Out-of Limit Detector function.
    #[inline(always)]
    pub const fn mdf_old5cr(&self) -> &MDF_OLD5CR {
        &self.mdf_old5cr
    }
    ///0x31c - This register is used for the adjustment of the Out-off Limit low threshold.
    #[inline(always)]
    pub const fn mdf_old5thlr(&self) -> &MDF_OLD5THLR {
        &self.mdf_old5thlr
    }
    ///0x320 - This register is used for the adjustment of the Out-off Limit high threshold.
    #[inline(always)]
    pub const fn mdf_old5thhr(&self) -> &MDF_OLD5THHR {
        &self.mdf_old5thhr
    }
    ///0x324 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_dly5cr(&self) -> &MDF_DLY5CR {
        &self.mdf_dly5cr
    }
    ///0x328 - This register is used for the adjustment stream delays.
    #[inline(always)]
    pub const fn mdf_scd5cr(&self) -> &MDF_SCD5CR {
        &self.mdf_scd5cr
    }
    ///0x32c - MDF DFLTx interrupt enable register x
    #[inline(always)]
    pub const fn mdf_dflt5ier(&self) -> &MDF_DFLT5IER {
        &self.mdf_dflt5ier
    }
    ///0x330 - This register contains the status flags for each digital filter path.
    #[inline(always)]
    pub const fn mdf_dflt5isr(&self) -> &MDF_DFLT5ISR {
        &self.mdf_dflt5isr
    }
    ///0x334 - This register contains the offset compensation value.
    #[inline(always)]
    pub const fn mdf_oec5cr(&self) -> &MDF_OEC5CR {
        &self.mdf_oec5cr
    }
    ///0x36c - This register is used to read the data processed by each digital filter in snapshot mode.
    #[inline(always)]
    pub const fn mdf_snps5dr(&self) -> &MDF_SNPS5DR {
        &self.mdf_snps5dr
    }
    ///0x370 - This register is used to read the data processed by each digital filter.
    #[inline(always)]
    pub const fn mdf_dflt5dr(&self) -> &MDF_DFLT5DR {
        &self.mdf_dflt5dr
    }
}
/**GCR (rw) register accessor: MDF global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:GCR)

For information about available fields see [`mod@gcr`]
module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///MDF global control register
pub mod gcr;
/**CKGCR (rw) register accessor: MDF clock generator control register

You can [`read`](crate::Reg::read) this register and get [`ckgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:CKGCR)

For information about available fields see [`mod@ckgcr`]
module*/
pub type CKGCR = crate::Reg<ckgcr::CKGCRrs>;
///MDF clock generator control register
pub mod ckgcr;
/**MDF_SITF0CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF0CR)

For information about available fields see [`mod@mdf_sitf0cr`]
module*/
pub type MDF_SITF0CR = crate::Reg<mdf_sitf0cr::MDF_SITF0CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf0cr;
/**MDF_SITF1CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF1CR)

For information about available fields see [`mod@mdf_sitf1cr`]
module*/
pub type MDF_SITF1CR = crate::Reg<mdf_sitf1cr::MDF_SITF1CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf1cr;
/**MDF_SITF2CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF2CR)

For information about available fields see [`mod@mdf_sitf2cr`]
module*/
pub type MDF_SITF2CR = crate::Reg<mdf_sitf2cr::MDF_SITF2CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf2cr;
/**MDF_SITF3CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF3CR)

For information about available fields see [`mod@mdf_sitf3cr`]
module*/
pub type MDF_SITF3CR = crate::Reg<mdf_sitf3cr::MDF_SITF3CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf3cr;
/**MDF_SITF4CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF4CR)

For information about available fields see [`mod@mdf_sitf4cr`]
module*/
pub type MDF_SITF4CR = crate::Reg<mdf_sitf4cr::MDF_SITF4CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf4cr;
/**MDF_SITF5CR (rw) register accessor: This register is used to control the serial interfaces (SITFx).

You can [`read`](crate::Reg::read) this register and get [`mdf_sitf5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_sitf5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SITF5CR)

For information about available fields see [`mod@mdf_sitf5cr`]
module*/
pub type MDF_SITF5CR = crate::Reg<mdf_sitf5cr::MDF_SITF5CRrs>;
///This register is used to control the serial interfaces (SITFx).
pub mod mdf_sitf5cr;
/**MDF_BSMX0CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX0CR)

For information about available fields see [`mod@mdf_bsmx0cr`]
module*/
pub type MDF_BSMX0CR = crate::Reg<mdf_bsmx0cr::MDF_BSMX0CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx0cr;
/**MDF_BSMX1CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX1CR)

For information about available fields see [`mod@mdf_bsmx1cr`]
module*/
pub type MDF_BSMX1CR = crate::Reg<mdf_bsmx1cr::MDF_BSMX1CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx1cr;
/**MDF_BSMX2CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX2CR)

For information about available fields see [`mod@mdf_bsmx2cr`]
module*/
pub type MDF_BSMX2CR = crate::Reg<mdf_bsmx2cr::MDF_BSMX2CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx2cr;
/**MDF_BSMX3CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX3CR)

For information about available fields see [`mod@mdf_bsmx3cr`]
module*/
pub type MDF_BSMX3CR = crate::Reg<mdf_bsmx3cr::MDF_BSMX3CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx3cr;
/**MDF_BSMX4CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX4CR)

For information about available fields see [`mod@mdf_bsmx4cr`]
module*/
pub type MDF_BSMX4CR = crate::Reg<mdf_bsmx4cr::MDF_BSMX4CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx4cr;
/**MDF_BSMX5CR (rw) register accessor: This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.

You can [`read`](crate::Reg::read) this register and get [`mdf_bsmx5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_bsmx5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_BSMX5CR)

For information about available fields see [`mod@mdf_bsmx5cr`]
module*/
pub type MDF_BSMX5CR = crate::Reg<mdf_bsmx5cr::MDF_BSMX5CRrs>;
///This register is used to select the bitstream to be provided to the corresponding digital filter and to the SCD.
pub mod mdf_bsmx5cr;
/**MDF_DFLT0CR (rw) register accessor: This register is used to control the digital filter x.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0CR)

For information about available fields see [`mod@mdf_dflt0cr`]
module*/
pub type MDF_DFLT0CR = crate::Reg<mdf_dflt0cr::MDF_DFLT0CRrs>;
///This register is used to control the digital filter x.
pub mod mdf_dflt0cr;
/**MDF_DFLT1CR (rw) register accessor: This register is used to control the digital filter x.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1CR)

For information about available fields see [`mod@mdf_dflt1cr`]
module*/
pub type MDF_DFLT1CR = crate::Reg<mdf_dflt1cr::MDF_DFLT1CRrs>;
///This register is used to control the digital filter x.
pub mod mdf_dflt1cr;
/**MDF_DFLT2CR (rw) register accessor: This register is used to control the digital filter 2.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2CR)

For information about available fields see [`mod@mdf_dflt2cr`]
module*/
pub type MDF_DFLT2CR = crate::Reg<mdf_dflt2cr::MDF_DFLT2CRrs>;
///This register is used to control the digital filter 2.
pub mod mdf_dflt2cr;
/**MDF_DFLT3CR (rw) register accessor: This register is used to control the digital filter 3.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3CR)

For information about available fields see [`mod@mdf_dflt3cr`]
module*/
pub type MDF_DFLT3CR = crate::Reg<mdf_dflt3cr::MDF_DFLT3CRrs>;
///This register is used to control the digital filter 3.
pub mod mdf_dflt3cr;
/**MDF_DFLT4CR (rw) register accessor: This register is used to control the digital filter 4.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4CR)

For information about available fields see [`mod@mdf_dflt4cr`]
module*/
pub type MDF_DFLT4CR = crate::Reg<mdf_dflt4cr::MDF_DFLT4CRrs>;
///This register is used to control the digital filter 4.
pub mod mdf_dflt4cr;
/**MDF_DFLT5CR (rw) register accessor: This register is used to control the digital filter x.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5CR)

For information about available fields see [`mod@mdf_dflt5cr`]
module*/
pub type MDF_DFLT5CR = crate::Reg<mdf_dflt5cr::MDF_DFLT5CRrs>;
///This register is used to control the digital filter x.
pub mod mdf_dflt5cr;
/**MDF_DFLT0CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0CICR)

For information about available fields see [`mod@mdf_dflt0cicr`]
module*/
pub type MDF_DFLT0CICR = crate::Reg<mdf_dflt0cicr::MDF_DFLT0CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt0cicr;
/**MDF_DFLT1CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1CICR)

For information about available fields see [`mod@mdf_dflt1cicr`]
module*/
pub type MDF_DFLT1CICR = crate::Reg<mdf_dflt1cicr::MDF_DFLT1CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt1cicr;
/**MDF_DFLT2CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2CICR)

For information about available fields see [`mod@mdf_dflt2cicr`]
module*/
pub type MDF_DFLT2CICR = crate::Reg<mdf_dflt2cicr::MDF_DFLT2CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt2cicr;
/**MDF_DFLT3CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3CICR)

For information about available fields see [`mod@mdf_dflt3cicr`]
module*/
pub type MDF_DFLT3CICR = crate::Reg<mdf_dflt3cicr::MDF_DFLT3CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt3cicr;
/**MDF_DFLT4CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4CICR)

For information about available fields see [`mod@mdf_dflt4cicr`]
module*/
pub type MDF_DFLT4CICR = crate::Reg<mdf_dflt4cicr::MDF_DFLT4CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt4cicr;
/**MDF_DFLT5CICR (rw) register accessor: This register is used to control the main CIC filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5CICR)

For information about available fields see [`mod@mdf_dflt5cicr`]
module*/
pub type MDF_DFLT5CICR = crate::Reg<mdf_dflt5cicr::MDF_DFLT5CICRrs>;
///This register is used to control the main CIC filter.
pub mod mdf_dflt5cicr;
/**MDF_DFLT0RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0RSFR)

For information about available fields see [`mod@mdf_dflt0rsfr`]
module*/
pub type MDF_DFLT0RSFR = crate::Reg<mdf_dflt0rsfr::MDF_DFLT0RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt0rsfr;
/**MDF_DFLT1RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1RSFR)

For information about available fields see [`mod@mdf_dflt1rsfr`]
module*/
pub type MDF_DFLT1RSFR = crate::Reg<mdf_dflt1rsfr::MDF_DFLT1RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt1rsfr;
/**MDF_DFLT2RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2RSFR)

For information about available fields see [`mod@mdf_dflt2rsfr`]
module*/
pub type MDF_DFLT2RSFR = crate::Reg<mdf_dflt2rsfr::MDF_DFLT2RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt2rsfr;
/**MDF_DFLT3RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3RSFR)

For information about available fields see [`mod@mdf_dflt3rsfr`]
module*/
pub type MDF_DFLT3RSFR = crate::Reg<mdf_dflt3rsfr::MDF_DFLT3RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt3rsfr;
/**MDF_DFLT4RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4RSFR)

For information about available fields see [`mod@mdf_dflt4rsfr`]
module*/
pub type MDF_DFLT4RSFR = crate::Reg<mdf_dflt4rsfr::MDF_DFLT4RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt4rsfr;
/**MDF_DFLT5RSFR (rw) register accessor: This register is used to control the reshape and HPF filters.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5rsfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5rsfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5RSFR)

For information about available fields see [`mod@mdf_dflt5rsfr`]
module*/
pub type MDF_DFLT5RSFR = crate::Reg<mdf_dflt5rsfr::MDF_DFLT5RSFRrs>;
///This register is used to control the reshape and HPF filters.
pub mod mdf_dflt5rsfr;
/**MDF_DFLT0INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0INTR)

For information about available fields see [`mod@mdf_dflt0intr`]
module*/
pub type MDF_DFLT0INTR = crate::Reg<mdf_dflt0intr::MDF_DFLT0INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt0intr;
/**MDF_DFLT1INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1INTR)

For information about available fields see [`mod@mdf_dflt1intr`]
module*/
pub type MDF_DFLT1INTR = crate::Reg<mdf_dflt1intr::MDF_DFLT1INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt1intr;
/**MDF_DFLT2INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2INTR)

For information about available fields see [`mod@mdf_dflt2intr`]
module*/
pub type MDF_DFLT2INTR = crate::Reg<mdf_dflt2intr::MDF_DFLT2INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt2intr;
/**MDF_DFLT3INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3INTR)

For information about available fields see [`mod@mdf_dflt3intr`]
module*/
pub type MDF_DFLT3INTR = crate::Reg<mdf_dflt3intr::MDF_DFLT3INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt3intr;
/**MDF_DFLT4INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4INTR)

For information about available fields see [`mod@mdf_dflt4intr`]
module*/
pub type MDF_DFLT4INTR = crate::Reg<mdf_dflt4intr::MDF_DFLT4INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt4intr;
/**MDF_DFLT5INTR (rw) register accessor: This register is used to the integrator (INT) settings.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5intr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5intr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5INTR)

For information about available fields see [`mod@mdf_dflt5intr`]
module*/
pub type MDF_DFLT5INTR = crate::Reg<mdf_dflt5intr::MDF_DFLT5INTRrs>;
///This register is used to the integrator (INT) settings.
pub mod mdf_dflt5intr;
/**MDF_OLD0CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD0CR)

For information about available fields see [`mod@mdf_old0cr`]
module*/
pub type MDF_OLD0CR = crate::Reg<mdf_old0cr::MDF_OLD0CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old0cr;
/**MDF_OLD1CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD1CR)

For information about available fields see [`mod@mdf_old1cr`]
module*/
pub type MDF_OLD1CR = crate::Reg<mdf_old1cr::MDF_OLD1CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old1cr;
/**MDF_OLD2CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD2CR)

For information about available fields see [`mod@mdf_old2cr`]
module*/
pub type MDF_OLD2CR = crate::Reg<mdf_old2cr::MDF_OLD2CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old2cr;
/**MDF_OLD3CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD3CR)

For information about available fields see [`mod@mdf_old3cr`]
module*/
pub type MDF_OLD3CR = crate::Reg<mdf_old3cr::MDF_OLD3CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old3cr;
/**MDF_OLD4CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD4CR)

For information about available fields see [`mod@mdf_old4cr`]
module*/
pub type MDF_OLD4CR = crate::Reg<mdf_old4cr::MDF_OLD4CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old4cr;
/**MDF_OLD5CR (rw) register accessor: This register is used to configure the Out-of Limit Detector function.

You can [`read`](crate::Reg::read) this register and get [`mdf_old5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD5CR)

For information about available fields see [`mod@mdf_old5cr`]
module*/
pub type MDF_OLD5CR = crate::Reg<mdf_old5cr::MDF_OLD5CRrs>;
///This register is used to configure the Out-of Limit Detector function.
pub mod mdf_old5cr;
/**MDF_OLD0THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old0thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old0thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD0THLR)

For information about available fields see [`mod@mdf_old0thlr`]
module*/
pub type MDF_OLD0THLR = crate::Reg<mdf_old0thlr::MDF_OLD0THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old0thlr;
/**MDF_OLD1THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old1thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old1thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD1THLR)

For information about available fields see [`mod@mdf_old1thlr`]
module*/
pub type MDF_OLD1THLR = crate::Reg<mdf_old1thlr::MDF_OLD1THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old1thlr;
/**MDF_OLD2THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old2thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old2thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD2THLR)

For information about available fields see [`mod@mdf_old2thlr`]
module*/
pub type MDF_OLD2THLR = crate::Reg<mdf_old2thlr::MDF_OLD2THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old2thlr;
/**MDF_OLD3THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old3thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old3thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD3THLR)

For information about available fields see [`mod@mdf_old3thlr`]
module*/
pub type MDF_OLD3THLR = crate::Reg<mdf_old3thlr::MDF_OLD3THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old3thlr;
/**MDF_OLD4THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old4thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old4thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD4THLR)

For information about available fields see [`mod@mdf_old4thlr`]
module*/
pub type MDF_OLD4THLR = crate::Reg<mdf_old4thlr::MDF_OLD4THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old4thlr;
/**MDF_OLD5THLR (rw) register accessor: This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old5thlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old5thlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD5THLR)

For information about available fields see [`mod@mdf_old5thlr`]
module*/
pub type MDF_OLD5THLR = crate::Reg<mdf_old5thlr::MDF_OLD5THLRrs>;
///This register is used for the adjustment of the Out-off Limit low threshold.
pub mod mdf_old5thlr;
/**MDF_OLD0THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old0thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old0thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD0THHR)

For information about available fields see [`mod@mdf_old0thhr`]
module*/
pub type MDF_OLD0THHR = crate::Reg<mdf_old0thhr::MDF_OLD0THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old0thhr;
/**MDF_OLD1THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old1thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old1thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD1THHR)

For information about available fields see [`mod@mdf_old1thhr`]
module*/
pub type MDF_OLD1THHR = crate::Reg<mdf_old1thhr::MDF_OLD1THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old1thhr;
/**MDF_OLD2THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old2thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old2thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD2THHR)

For information about available fields see [`mod@mdf_old2thhr`]
module*/
pub type MDF_OLD2THHR = crate::Reg<mdf_old2thhr::MDF_OLD2THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old2thhr;
/**MDF_OLD3THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old3thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old3thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD3THHR)

For information about available fields see [`mod@mdf_old3thhr`]
module*/
pub type MDF_OLD3THHR = crate::Reg<mdf_old3thhr::MDF_OLD3THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old3thhr;
/**MDF_OLD4THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old4thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old4thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD4THHR)

For information about available fields see [`mod@mdf_old4thhr`]
module*/
pub type MDF_OLD4THHR = crate::Reg<mdf_old4thhr::MDF_OLD4THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old4thhr;
/**MDF_OLD5THHR (rw) register accessor: This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old5thhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old5thhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OLD5THHR)

For information about available fields see [`mod@mdf_old5thhr`]
module*/
pub type MDF_OLD5THHR = crate::Reg<mdf_old5thhr::MDF_OLD5THHRrs>;
///This register is used for the adjustment of the Out-off Limit high threshold.
pub mod mdf_old5thhr;
/**MDF_DLY0CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY0CR)

For information about available fields see [`mod@mdf_dly0cr`]
module*/
pub type MDF_DLY0CR = crate::Reg<mdf_dly0cr::MDF_DLY0CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly0cr;
/**MDF_DLY1CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY1CR)

For information about available fields see [`mod@mdf_dly1cr`]
module*/
pub type MDF_DLY1CR = crate::Reg<mdf_dly1cr::MDF_DLY1CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly1cr;
/**MDF_DLY2CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY2CR)

For information about available fields see [`mod@mdf_dly2cr`]
module*/
pub type MDF_DLY2CR = crate::Reg<mdf_dly2cr::MDF_DLY2CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly2cr;
/**MDF_DLY3CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY3CR)

For information about available fields see [`mod@mdf_dly3cr`]
module*/
pub type MDF_DLY3CR = crate::Reg<mdf_dly3cr::MDF_DLY3CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly3cr;
/**MDF_DLY4CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY4CR)

For information about available fields see [`mod@mdf_dly4cr`]
module*/
pub type MDF_DLY4CR = crate::Reg<mdf_dly4cr::MDF_DLY4CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly4cr;
/**MDF_DLY5CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_dly5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dly5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DLY5CR)

For information about available fields see [`mod@mdf_dly5cr`]
module*/
pub type MDF_DLY5CR = crate::Reg<mdf_dly5cr::MDF_DLY5CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_dly5cr;
/**MDF_SCD0CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD0CR)

For information about available fields see [`mod@mdf_scd0cr`]
module*/
pub type MDF_SCD0CR = crate::Reg<mdf_scd0cr::MDF_SCD0CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd0cr;
/**MDF_SCD1CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD1CR)

For information about available fields see [`mod@mdf_scd1cr`]
module*/
pub type MDF_SCD1CR = crate::Reg<mdf_scd1cr::MDF_SCD1CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd1cr;
/**MDF_SCD2CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD2CR)

For information about available fields see [`mod@mdf_scd2cr`]
module*/
pub type MDF_SCD2CR = crate::Reg<mdf_scd2cr::MDF_SCD2CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd2cr;
/**MDF_SCD3CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD3CR)

For information about available fields see [`mod@mdf_scd3cr`]
module*/
pub type MDF_SCD3CR = crate::Reg<mdf_scd3cr::MDF_SCD3CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd3cr;
/**MDF_SCD4CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD4CR)

For information about available fields see [`mod@mdf_scd4cr`]
module*/
pub type MDF_SCD4CR = crate::Reg<mdf_scd4cr::MDF_SCD4CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd4cr;
/**MDF_SCD5CR (rw) register accessor: This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SCD5CR)

For information about available fields see [`mod@mdf_scd5cr`]
module*/
pub type MDF_SCD5CR = crate::Reg<mdf_scd5cr::MDF_SCD5CRrs>;
///This register is used for the adjustment stream delays.
pub mod mdf_scd5cr;
/**MDF_DFLT0IER (rw) register accessor: This register is used for allowing or not the events to generate an interrupt.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0IER)

For information about available fields see [`mod@mdf_dflt0ier`]
module*/
pub type MDF_DFLT0IER = crate::Reg<mdf_dflt0ier::MDF_DFLT0IERrs>;
///This register is used for allowing or not the events to generate an interrupt.
pub mod mdf_dflt0ier;
/**MDF_DFLT0ISR (rw) register accessor: MDF DFLT0 interrupt status register 0

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt0isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0ISR)

For information about available fields see [`mod@mdf_dflt0isr`]
module*/
pub type MDF_DFLT0ISR = crate::Reg<mdf_dflt0isr::MDF_DFLT0ISRrs>;
///MDF DFLT0 interrupt status register 0
pub mod mdf_dflt0isr;
/**MDF_DFLT1IER (rw) register accessor: MDF DFLTx interrupt enable register x

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1IER)

For information about available fields see [`mod@mdf_dflt1ier`]
module*/
pub type MDF_DFLT1IER = crate::Reg<mdf_dflt1ier::MDF_DFLT1IERrs>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt1ier;
/**MDF_DFLT2IER (rw) register accessor: MDF DFLTx interrupt enable register x

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2IER)

For information about available fields see [`mod@mdf_dflt2ier`]
module*/
pub type MDF_DFLT2IER = crate::Reg<mdf_dflt2ier::MDF_DFLT2IERrs>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt2ier;
/**MDF_DFLT3IER (rw) register accessor: MDF DFLTx interrupt enable register x

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3IER)

For information about available fields see [`mod@mdf_dflt3ier`]
module*/
pub type MDF_DFLT3IER = crate::Reg<mdf_dflt3ier::MDF_DFLT3IERrs>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt3ier;
/**MDF_DFLT4IER (rw) register accessor: MDF DFLTx interrupt enable register x

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4IER)

For information about available fields see [`mod@mdf_dflt4ier`]
module*/
pub type MDF_DFLT4IER = crate::Reg<mdf_dflt4ier::MDF_DFLT4IERrs>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt4ier;
/**MDF_DFLT5IER (rw) register accessor: MDF DFLTx interrupt enable register x

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5IER)

For information about available fields see [`mod@mdf_dflt5ier`]
module*/
pub type MDF_DFLT5IER = crate::Reg<mdf_dflt5ier::MDF_DFLT5IERrs>;
///MDF DFLTx interrupt enable register x
pub mod mdf_dflt5ier;
/**MDF_DFLT1ISR (rw) register accessor: This register contains the status flags for each digital filter path.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt1isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1ISR)

For information about available fields see [`mod@mdf_dflt1isr`]
module*/
pub type MDF_DFLT1ISR = crate::Reg<mdf_dflt1isr::MDF_DFLT1ISRrs>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt1isr;
/**MDF_DFLT2ISR (rw) register accessor: This register contains the status flags for each digital filter path.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt2isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2ISR)

For information about available fields see [`mod@mdf_dflt2isr`]
module*/
pub type MDF_DFLT2ISR = crate::Reg<mdf_dflt2isr::MDF_DFLT2ISRrs>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt2isr;
/**MDF_DFLT3ISR (rw) register accessor: This register contains the status flags for each digital filter path.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt3isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3ISR)

For information about available fields see [`mod@mdf_dflt3isr`]
module*/
pub type MDF_DFLT3ISR = crate::Reg<mdf_dflt3isr::MDF_DFLT3ISRrs>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt3isr;
/**MDF_DFLT4ISR (rw) register accessor: This register contains the status flags for each digital filter path.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt4isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4ISR)

For information about available fields see [`mod@mdf_dflt4isr`]
module*/
pub type MDF_DFLT4ISR = crate::Reg<mdf_dflt4isr::MDF_DFLT4ISRrs>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt4isr;
/**MDF_DFLT5ISR (rw) register accessor: This register contains the status flags for each digital filter path.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_dflt5isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5ISR)

For information about available fields see [`mod@mdf_dflt5isr`]
module*/
pub type MDF_DFLT5ISR = crate::Reg<mdf_dflt5isr::MDF_DFLT5ISRrs>;
///This register contains the status flags for each digital filter path.
pub mod mdf_dflt5isr;
/**MDF_OEC0CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC0CR)

For information about available fields see [`mod@mdf_oec0cr`]
module*/
pub type MDF_OEC0CR = crate::Reg<mdf_oec0cr::MDF_OEC0CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec0cr;
/**MDF_OEC1CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC1CR)

For information about available fields see [`mod@mdf_oec1cr`]
module*/
pub type MDF_OEC1CR = crate::Reg<mdf_oec1cr::MDF_OEC1CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec1cr;
/**MDF_OEC2CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC2CR)

For information about available fields see [`mod@mdf_oec2cr`]
module*/
pub type MDF_OEC2CR = crate::Reg<mdf_oec2cr::MDF_OEC2CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec2cr;
/**MDF_OEC3CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC3CR)

For information about available fields see [`mod@mdf_oec3cr`]
module*/
pub type MDF_OEC3CR = crate::Reg<mdf_oec3cr::MDF_OEC3CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec3cr;
/**MDF_OEC4CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC4CR)

For information about available fields see [`mod@mdf_oec4cr`]
module*/
pub type MDF_OEC4CR = crate::Reg<mdf_oec4cr::MDF_OEC4CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec4cr;
/**MDF_OEC5CR (rw) register accessor: This register contains the offset compensation value.

You can [`read`](crate::Reg::read) this register and get [`mdf_oec5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_oec5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_OEC5CR)

For information about available fields see [`mod@mdf_oec5cr`]
module*/
pub type MDF_OEC5CR = crate::Reg<mdf_oec5cr::MDF_OEC5CRrs>;
///This register contains the offset compensation value.
pub mod mdf_oec5cr;
/**MDF_SNPS0DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS0DR)

For information about available fields see [`mod@mdf_snps0dr`]
module*/
pub type MDF_SNPS0DR = crate::Reg<mdf_snps0dr::MDF_SNPS0DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps0dr;
/**MDF_SNPS1DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS1DR)

For information about available fields see [`mod@mdf_snps1dr`]
module*/
pub type MDF_SNPS1DR = crate::Reg<mdf_snps1dr::MDF_SNPS1DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps1dr;
/**MDF_SNPS2DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS2DR)

For information about available fields see [`mod@mdf_snps2dr`]
module*/
pub type MDF_SNPS2DR = crate::Reg<mdf_snps2dr::MDF_SNPS2DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps2dr;
/**MDF_SNPS3DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps3dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS3DR)

For information about available fields see [`mod@mdf_snps3dr`]
module*/
pub type MDF_SNPS3DR = crate::Reg<mdf_snps3dr::MDF_SNPS3DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps3dr;
/**MDF_SNPS4DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps4dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS4DR)

For information about available fields see [`mod@mdf_snps4dr`]
module*/
pub type MDF_SNPS4DR = crate::Reg<mdf_snps4dr::MDF_SNPS4DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps4dr;
/**MDF_SNPS5DR (r) register accessor: This register is used to read the data processed by each digital filter in snapshot mode.

You can [`read`](crate::Reg::read) this register and get [`mdf_snps5dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_SNPS5DR)

For information about available fields see [`mod@mdf_snps5dr`]
module*/
pub type MDF_SNPS5DR = crate::Reg<mdf_snps5dr::MDF_SNPS5DRrs>;
///This register is used to read the data processed by each digital filter in snapshot mode.
pub mod mdf_snps5dr;
/**MDF_DFLT0DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt0dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT0DR)

For information about available fields see [`mod@mdf_dflt0dr`]
module*/
pub type MDF_DFLT0DR = crate::Reg<mdf_dflt0dr::MDF_DFLT0DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt0dr;
/**MDF_DFLT1DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt1dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT1DR)

For information about available fields see [`mod@mdf_dflt1dr`]
module*/
pub type MDF_DFLT1DR = crate::Reg<mdf_dflt1dr::MDF_DFLT1DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt1dr;
/**MDF_DFLT2DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT2DR)

For information about available fields see [`mod@mdf_dflt2dr`]
module*/
pub type MDF_DFLT2DR = crate::Reg<mdf_dflt2dr::MDF_DFLT2DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt2dr;
/**MDF_DFLT3DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT3DR)

For information about available fields see [`mod@mdf_dflt3dr`]
module*/
pub type MDF_DFLT3DR = crate::Reg<mdf_dflt3dr::MDF_DFLT3DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt3dr;
/**MDF_DFLT4DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt4dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT4DR)

For information about available fields see [`mod@mdf_dflt4dr`]
module*/
pub type MDF_DFLT4DR = crate::Reg<mdf_dflt4dr::MDF_DFLT4DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt4dr;
/**MDF_DFLT5DR (r) register accessor: This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt5dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#MDF1:MDF_DFLT5DR)

For information about available fields see [`mod@mdf_dflt5dr`]
module*/
pub type MDF_DFLT5DR = crate::Reg<mdf_dflt5dr::MDF_DFLT5DRrs>;
///This register is used to read the data processed by each digital filter.
pub mod mdf_dflt5dr;
