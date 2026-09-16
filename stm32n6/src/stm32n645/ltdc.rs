#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    sscr: SSCR,
    bpcr: BPCR,
    awcr: AWCR,
    twcr: TWCR,
    gcr: GCR,
    gc1r: GC1R,
    gc2r: GC2R,
    srcr: SRCR,
    gccr: GCCR,
    bccr: BCCR,
    _reserved10: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    lipcr: LIPCR,
    cpsr: CPSR,
    cdsr: CDSR,
    _reserved16: [u8; 0x14],
    edcr: EDCR,
    ier2: IER2,
    isr2: ISR2,
    icr2: ICR2,
    lipcr2: LIPCR2,
    _reserved21: [u8; 0x04],
    ecrcr: ECRCR,
    ccrcr: CCRCR,
    _reserved23: [u8; 0x10],
    futr: FUTR,
    _reserved24: [u8; 0x6c],
    l1c0r: L1C0R,
    l1c1r: L1C1R,
    l1rcr: L1RCR,
    l1cr: L1CR,
    l1whpcr: L1WHPCR,
    l1wvpcr: L1WVPCR,
    l1ckcr: L1CKCR,
    l1pfcr: L1PFCR,
    l1cacr: L1CACR,
    l1dccr: L1DCCR,
    l1bfcr: L1BFCR,
    l1blcr: L1BLCR,
    l1pcr: L1PCR,
    l1cfbar: L1CFBAR,
    l1cfblr: L1CFBLR,
    l1cfblnr: L1CFBLNR,
    l1afba0r: L1AFBA0R,
    l1afba1r: L1AFBA1R,
    l1afblr: L1AFBLR,
    l1afblnr: L1AFBLNR,
    l1clutwr: L1CLUTWR,
    _reserved45: [u8; 0x18],
    l1cyr0r: L1CYR0R,
    l1cyr1r: L1CYR1R,
    l1fpf0r: L1FPF0R,
    l1fpf1r: L1FPF1R,
    _reserved49: [u8; 0x84],
    l2c0r: L2C0R,
    l2c1r: L2C1R,
    l2rcr: L2RCR,
    l2cr: L2CR,
    l2whpcr: L2WHPCR,
    l2wvpcr: L2WVPCR,
    l2ckcr: L2CKCR,
    l2pfcr: L2PFCR,
    l2cacr: L2CACR,
    l2dccr: L2DCCR,
    l2bfcr: L2BFCR,
    l2blcr: L2BLCR,
    l2pcr: L2PCR,
    l2cfbar: L2CFBAR,
    l2cfblr: L2CFBLR,
    l2cfblnr: L2CFBLNR,
    _reserved65: [u8; 0x10],
    l2clutwr: L2CLUTWR,
    _reserved66: [u8; 0x18],
    l2cyr0r: L2CYR0R,
    l2cyr1r: L2CYR1R,
    l2fpf0r: L2FPF0R,
    l2fpf1r: L2FPF1R,
}
impl RegisterBlock {
    ///0x08 - LTDC synchronization size configuration register
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    ///0x0c - LTDC back porch configuration register
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    ///0x10 - LTDC active width configuration register
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    ///0x14 - LTDC total width configuration register
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    ///0x18 - LTDC global control register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x1c - LTDC global configuration 1 register
    #[inline(always)]
    pub const fn gc1r(&self) -> &GC1R {
        &self.gc1r
    }
    ///0x20 - LTDC global configuration 2 register
    #[inline(always)]
    pub const fn gc2r(&self) -> &GC2R {
        &self.gc2r
    }
    ///0x24 - LTDC shadow reload configuration register
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    ///0x28 - LTDC gamma correction configuration register
    #[inline(always)]
    pub const fn gccr(&self) -> &GCCR {
        &self.gccr
    }
    ///0x2c - LTDC background color configuration register
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    ///0x34 - LTDC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x38 - LTDC interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x3c - LTDC interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x40 - LTDC line interrupt position configuration register
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    ///0x44 - LTDC current position status register
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    ///0x48 - LTDC current display status register
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    ///0x60 - LTDC external display control register
    #[inline(always)]
    pub const fn edcr(&self) -> &EDCR {
        &self.edcr
    }
    ///0x64 - LTDC interrupt enable register 2
    #[inline(always)]
    pub const fn ier2(&self) -> &IER2 {
        &self.ier2
    }
    ///0x68 - LTDC interrupt status register 2
    #[inline(always)]
    pub const fn isr2(&self) -> &ISR2 {
        &self.isr2
    }
    ///0x6c - LTDC interrupt clear register 2
    #[inline(always)]
    pub const fn icr2(&self) -> &ICR2 {
        &self.icr2
    }
    ///0x70 - LTDC line interrupt position configuration register 2
    #[inline(always)]
    pub const fn lipcr2(&self) -> &LIPCR2 {
        &self.lipcr2
    }
    ///0x78 - LTDC expected CRC register
    #[inline(always)]
    pub const fn ecrcr(&self) -> &ECRCR {
        &self.ecrcr
    }
    ///0x7c - LTDC computed CRC register
    #[inline(always)]
    pub const fn ccrcr(&self) -> &CCRCR {
        &self.ccrcr
    }
    ///0x90 - LTDC FIFO underrun threshold register
    #[inline(always)]
    pub const fn futr(&self) -> &FUTR {
        &self.futr
    }
    ///0x100 - LTDC layerx configuration 0 register
    #[inline(always)]
    pub const fn l1c0r(&self) -> &L1C0R {
        &self.l1c0r
    }
    ///0x104 - LTDC layerx configuration 1 register
    #[inline(always)]
    pub const fn l1c1r(&self) -> &L1C1R {
        &self.l1c1r
    }
    ///0x108 - LTDC layerx reload control register
    #[inline(always)]
    pub const fn l1rcr(&self) -> &L1RCR {
        &self.l1rcr
    }
    ///0x10c - LTDC layerx control register
    #[inline(always)]
    pub const fn l1cr(&self) -> &L1CR {
        &self.l1cr
    }
    ///0x110 - LTDC layerx window horizontal position configuration register
    #[inline(always)]
    pub const fn l1whpcr(&self) -> &L1WHPCR {
        &self.l1whpcr
    }
    ///0x114 - LTDC layerx window vertical position configuration register
    #[inline(always)]
    pub const fn l1wvpcr(&self) -> &L1WVPCR {
        &self.l1wvpcr
    }
    ///0x118 - LTDC layerx color keying configuration register
    #[inline(always)]
    pub const fn l1ckcr(&self) -> &L1CKCR {
        &self.l1ckcr
    }
    ///0x11c - LTDC layerx pixel format configuration register
    #[inline(always)]
    pub const fn l1pfcr(&self) -> &L1PFCR {
        &self.l1pfcr
    }
    ///0x120 - LTDC layerx constant alpha configuration register
    #[inline(always)]
    pub const fn l1cacr(&self) -> &L1CACR {
        &self.l1cacr
    }
    ///0x124 - LTDC layerx default color configuration register
    #[inline(always)]
    pub const fn l1dccr(&self) -> &L1DCCR {
        &self.l1dccr
    }
    ///0x128 - LTDC layerx blending factors configuration register
    #[inline(always)]
    pub const fn l1bfcr(&self) -> &L1BFCR {
        &self.l1bfcr
    }
    ///0x12c - LTDC layerx burst length configuration register
    #[inline(always)]
    pub const fn l1blcr(&self) -> &L1BLCR {
        &self.l1blcr
    }
    ///0x130 - LTDC layerx planar configuration register
    #[inline(always)]
    pub const fn l1pcr(&self) -> &L1PCR {
        &self.l1pcr
    }
    ///0x134 - LTDC layerx color frame buffer address register
    #[inline(always)]
    pub const fn l1cfbar(&self) -> &L1CFBAR {
        &self.l1cfbar
    }
    ///0x138 - LTDC layerx color frame buffer length register
    #[inline(always)]
    pub const fn l1cfblr(&self) -> &L1CFBLR {
        &self.l1cfblr
    }
    ///0x13c - LTDC layerx color frame buffer line number register
    #[inline(always)]
    pub const fn l1cfblnr(&self) -> &L1CFBLNR {
        &self.l1cfblnr
    }
    ///0x140 - LTDC layer1 auxiliary frame buffer address 0 register
    #[inline(always)]
    pub const fn l1afba0r(&self) -> &L1AFBA0R {
        &self.l1afba0r
    }
    ///0x144 - LTDC layer1 auxiliary frame buffer address 1 register
    #[inline(always)]
    pub const fn l1afba1r(&self) -> &L1AFBA1R {
        &self.l1afba1r
    }
    ///0x148 - LTDC layer1 auxiliary frame buffer length register
    #[inline(always)]
    pub const fn l1afblr(&self) -> &L1AFBLR {
        &self.l1afblr
    }
    ///0x14c - LTDC layer1 auxiliary frame buffer line number register
    #[inline(always)]
    pub const fn l1afblnr(&self) -> &L1AFBLNR {
        &self.l1afblnr
    }
    ///0x150 - LTDC layerx CLUT write register
    #[inline(always)]
    pub const fn l1clutwr(&self) -> &L1CLUTWR {
        &self.l1clutwr
    }
    ///0x16c - LTDC layerx conversion YCbCr RGB 0 register
    #[inline(always)]
    pub const fn l1cyr0r(&self) -> &L1CYR0R {
        &self.l1cyr0r
    }
    ///0x170 - LTDC layerx conversion YCbCr RGB 1 register
    #[inline(always)]
    pub const fn l1cyr1r(&self) -> &L1CYR1R {
        &self.l1cyr1r
    }
    ///0x174 - LTDC layerx flexible pixel format 0 register
    #[inline(always)]
    pub const fn l1fpf0r(&self) -> &L1FPF0R {
        &self.l1fpf0r
    }
    ///0x178 - LTDC layerx flexible pixel format 1 register
    #[inline(always)]
    pub const fn l1fpf1r(&self) -> &L1FPF1R {
        &self.l1fpf1r
    }
    ///0x200 - LTDC layerx configuration 0 register
    #[inline(always)]
    pub const fn l2c0r(&self) -> &L2C0R {
        &self.l2c0r
    }
    ///0x204 - LTDC layerx configuration 1 register
    #[inline(always)]
    pub const fn l2c1r(&self) -> &L2C1R {
        &self.l2c1r
    }
    ///0x208 - LTDC layerx reload control register
    #[inline(always)]
    pub const fn l2rcr(&self) -> &L2RCR {
        &self.l2rcr
    }
    ///0x20c - LTDC layerx control register
    #[inline(always)]
    pub const fn l2cr(&self) -> &L2CR {
        &self.l2cr
    }
    ///0x210 - LTDC layerx window horizontal position configuration register
    #[inline(always)]
    pub const fn l2whpcr(&self) -> &L2WHPCR {
        &self.l2whpcr
    }
    ///0x214 - LTDC layerx window vertical position configuration register
    #[inline(always)]
    pub const fn l2wvpcr(&self) -> &L2WVPCR {
        &self.l2wvpcr
    }
    ///0x218 - LTDC layerx color keying configuration register
    #[inline(always)]
    pub const fn l2ckcr(&self) -> &L2CKCR {
        &self.l2ckcr
    }
    ///0x21c - LTDC layerx pixel format configuration register
    #[inline(always)]
    pub const fn l2pfcr(&self) -> &L2PFCR {
        &self.l2pfcr
    }
    ///0x220 - LTDC layerx constant alpha configuration register
    #[inline(always)]
    pub const fn l2cacr(&self) -> &L2CACR {
        &self.l2cacr
    }
    ///0x224 - LTDC layerx default color configuration register
    #[inline(always)]
    pub const fn l2dccr(&self) -> &L2DCCR {
        &self.l2dccr
    }
    ///0x228 - LTDC layerx blending factors configuration register
    #[inline(always)]
    pub const fn l2bfcr(&self) -> &L2BFCR {
        &self.l2bfcr
    }
    ///0x22c - LTDC layerx burst length configuration register
    #[inline(always)]
    pub const fn l2blcr(&self) -> &L2BLCR {
        &self.l2blcr
    }
    ///0x230 - LTDC layerx planar configuration register
    #[inline(always)]
    pub const fn l2pcr(&self) -> &L2PCR {
        &self.l2pcr
    }
    ///0x234 - LTDC layerx color frame buffer address register
    #[inline(always)]
    pub const fn l2cfbar(&self) -> &L2CFBAR {
        &self.l2cfbar
    }
    ///0x238 - LTDC layerx color frame buffer length register
    #[inline(always)]
    pub const fn l2cfblr(&self) -> &L2CFBLR {
        &self.l2cfblr
    }
    ///0x23c - LTDC layerx color frame buffer line number register
    #[inline(always)]
    pub const fn l2cfblnr(&self) -> &L2CFBLNR {
        &self.l2cfblnr
    }
    ///0x250 - LTDC layerx CLUT write register
    #[inline(always)]
    pub const fn l2clutwr(&self) -> &L2CLUTWR {
        &self.l2clutwr
    }
    ///0x26c - LTDC layerx conversion YCbCr RGB 0 register
    #[inline(always)]
    pub const fn l2cyr0r(&self) -> &L2CYR0R {
        &self.l2cyr0r
    }
    ///0x270 - LTDC layerx conversion YCbCr RGB 1 register
    #[inline(always)]
    pub const fn l2cyr1r(&self) -> &L2CYR1R {
        &self.l2cyr1r
    }
    ///0x274 - LTDC layerx flexible pixel format 0 register
    #[inline(always)]
    pub const fn l2fpf0r(&self) -> &L2FPF0R {
        &self.l2fpf0r
    }
    ///0x278 - LTDC layerx flexible pixel format 1 register
    #[inline(always)]
    pub const fn l2fpf1r(&self) -> &L2FPF1R {
        &self.l2fpf1r
    }
}
/**SSCR (rw) register accessor: LTDC synchronization size configuration register

You can [`read`](crate::Reg::read) this register and get [`sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:SSCR)

For information about available fields see [`mod@sscr`] module*/
pub type SSCR = crate::Reg<sscr::SSCRrs>;
///LTDC synchronization size configuration register
pub mod sscr;
/**BPCR (rw) register accessor: LTDC back porch configuration register

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:BPCR)

For information about available fields see [`mod@bpcr`] module*/
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
///LTDC back porch configuration register
pub mod bpcr;
/**AWCR (rw) register accessor: LTDC active width configuration register

You can [`read`](crate::Reg::read) this register and get [`awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:AWCR)

For information about available fields see [`mod@awcr`] module*/
pub type AWCR = crate::Reg<awcr::AWCRrs>;
///LTDC active width configuration register
pub mod awcr;
/**TWCR (rw) register accessor: LTDC total width configuration register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:TWCR)

For information about available fields see [`mod@twcr`] module*/
pub type TWCR = crate::Reg<twcr::TWCRrs>;
///LTDC total width configuration register
pub mod twcr;
/**GCR (rw) register accessor: LTDC global control register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:GCR)

For information about available fields see [`mod@gcr`] module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///LTDC global control register
pub mod gcr;
/**GC1R (r) register accessor: LTDC global configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`gc1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:GC1R)

For information about available fields see [`mod@gc1r`] module*/
pub type GC1R = crate::Reg<gc1r::GC1Rrs>;
///LTDC global configuration 1 register
pub mod gc1r;
/**GC2R (r) register accessor: LTDC global configuration 2 register

You can [`read`](crate::Reg::read) this register and get [`gc2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:GC2R)

For information about available fields see [`mod@gc2r`] module*/
pub type GC2R = crate::Reg<gc2r::GC2Rrs>;
///LTDC global configuration 2 register
pub mod gc2r;
/**SRCR (rw) register accessor: LTDC shadow reload configuration register

You can [`read`](crate::Reg::read) this register and get [`srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:SRCR)

For information about available fields see [`mod@srcr`] module*/
pub type SRCR = crate::Reg<srcr::SRCRrs>;
///LTDC shadow reload configuration register
pub mod srcr;
/**GCCR (w) register accessor: LTDC gamma correction configuration register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:GCCR)

For information about available fields see [`mod@gccr`] module*/
pub type GCCR = crate::Reg<gccr::GCCRrs>;
///LTDC gamma correction configuration register
pub mod gccr;
/**BCCR (rw) register accessor: LTDC background color configuration register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:BCCR)

For information about available fields see [`mod@bccr`] module*/
pub type BCCR = crate::Reg<bccr::BCCRrs>;
///LTDC background color configuration register
pub mod bccr;
/**IER (rw) register accessor: LTDC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///LTDC interrupt enable register
pub mod ier;
/**ISR (r) register accessor: LTDC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LTDC interrupt status register
pub mod isr;
/**ICR (w) register accessor: LTDC interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LTDC interrupt clear register
pub mod icr;
/**LIPCR (rw) register accessor: LTDC line interrupt position configuration register

You can [`read`](crate::Reg::read) this register and get [`lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:LIPCR)

For information about available fields see [`mod@lipcr`] module*/
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
///LTDC line interrupt position configuration register
pub mod lipcr;
/**CPSR (r) register accessor: LTDC current position status register

You can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:CPSR)

For information about available fields see [`mod@cpsr`] module*/
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
///LTDC current position status register
pub mod cpsr;
/**CDSR (r) register accessor: LTDC current display status register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:CDSR)

For information about available fields see [`mod@cdsr`] module*/
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
///LTDC current display status register
pub mod cdsr;
/**EDCR (rw) register accessor: LTDC external display control register

You can [`read`](crate::Reg::read) this register and get [`edcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:EDCR)

For information about available fields see [`mod@edcr`] module*/
pub type EDCR = crate::Reg<edcr::EDCRrs>;
///LTDC external display control register
pub mod edcr;
/**IER2 (rw) register accessor: LTDC interrupt enable register 2

You can [`read`](crate::Reg::read) this register and get [`ier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:IER2)

For information about available fields see [`mod@ier2`] module*/
pub type IER2 = crate::Reg<ier2::IER2rs>;
///LTDC interrupt enable register 2
pub mod ier2;
/**ISR2 (r) register accessor: LTDC interrupt status register 2

You can [`read`](crate::Reg::read) this register and get [`isr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ISR2)

For information about available fields see [`mod@isr2`] module*/
pub type ISR2 = crate::Reg<isr2::ISR2rs>;
///LTDC interrupt status register 2
pub mod isr2;
/**ICR2 (w) register accessor: LTDC interrupt clear register 2

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr2::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ICR2)

For information about available fields see [`mod@icr2`] module*/
pub type ICR2 = crate::Reg<icr2::ICR2rs>;
///LTDC interrupt clear register 2
pub mod icr2;
/**LIPCR2 (rw) register accessor: LTDC line interrupt position configuration register 2

You can [`read`](crate::Reg::read) this register and get [`lipcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:LIPCR2)

For information about available fields see [`mod@lipcr2`] module*/
pub type LIPCR2 = crate::Reg<lipcr2::LIPCR2rs>;
///LTDC line interrupt position configuration register 2
pub mod lipcr2;
/**ECRCR (rw) register accessor: LTDC expected CRC register

You can [`read`](crate::Reg::read) this register and get [`ecrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:ECRCR)

For information about available fields see [`mod@ecrcr`] module*/
pub type ECRCR = crate::Reg<ecrcr::ECRCRrs>;
///LTDC expected CRC register
pub mod ecrcr;
/**CCRCR (r) register accessor: LTDC computed CRC register

You can [`read`](crate::Reg::read) this register and get [`ccrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:CCRCR)

For information about available fields see [`mod@ccrcr`] module*/
pub type CCRCR = crate::Reg<ccrcr::CCRCRrs>;
///LTDC computed CRC register
pub mod ccrcr;
/**FUTR (rw) register accessor: LTDC FIFO underrun threshold register

You can [`read`](crate::Reg::read) this register and get [`futr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`futr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:FUTR)

For information about available fields see [`mod@futr`] module*/
pub type FUTR = crate::Reg<futr::FUTRrs>;
///LTDC FIFO underrun threshold register
pub mod futr;
/**L1C0R (r) register accessor: LTDC layerx configuration 0 register

You can [`read`](crate::Reg::read) this register and get [`l1c0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1C0R)

For information about available fields see [`mod@l1c0r`] module*/
pub type L1C0R = crate::Reg<l1c0r::L1C0Rrs>;
///LTDC layerx configuration 0 register
pub mod l1c0r;
/**L1C1R (rw) register accessor: LTDC layerx configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`l1c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1C1R)

For information about available fields see [`mod@l1c1r`] module*/
pub type L1C1R = crate::Reg<l1c1r::L1C1Rrs>;
///LTDC layerx configuration 1 register
pub mod l1c1r;
/**L1RCR (rw) register accessor: LTDC layerx reload control register

You can [`read`](crate::Reg::read) this register and get [`l1rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1RCR)

For information about available fields see [`mod@l1rcr`] module*/
pub type L1RCR = crate::Reg<l1rcr::L1RCRrs>;
///LTDC layerx reload control register
pub mod l1rcr;
/**L1CR (rw) register accessor: LTDC layerx control register

You can [`read`](crate::Reg::read) this register and get [`l1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CR)

For information about available fields see [`mod@l1cr`] module*/
pub type L1CR = crate::Reg<l1cr::L1CRrs>;
///LTDC layerx control register
pub mod l1cr;
/**L1WHPCR (rw) register accessor: LTDC layerx window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`l1whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1WHPCR)

For information about available fields see [`mod@l1whpcr`] module*/
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCRrs>;
///LTDC layerx window horizontal position configuration register
pub mod l1whpcr;
/**L1WVPCR (rw) register accessor: LTDC layerx window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`l1wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1WVPCR)

For information about available fields see [`mod@l1wvpcr`] module*/
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCRrs>;
///LTDC layerx window vertical position configuration register
pub mod l1wvpcr;
/**L1CKCR (rw) register accessor: LTDC layerx color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`l1ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CKCR)

For information about available fields see [`mod@l1ckcr`] module*/
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCRrs>;
///LTDC layerx color keying configuration register
pub mod l1ckcr;
/**L1PFCR (rw) register accessor: LTDC layerx pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`l1pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1PFCR)

For information about available fields see [`mod@l1pfcr`] module*/
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCRrs>;
///LTDC layerx pixel format configuration register
pub mod l1pfcr;
/**L1CACR (rw) register accessor: LTDC layerx constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`l1cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CACR)

For information about available fields see [`mod@l1cacr`] module*/
pub type L1CACR = crate::Reg<l1cacr::L1CACRrs>;
///LTDC layerx constant alpha configuration register
pub mod l1cacr;
/**L1DCCR (rw) register accessor: LTDC layerx default color configuration register

You can [`read`](crate::Reg::read) this register and get [`l1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1DCCR)

For information about available fields see [`mod@l1dccr`] module*/
pub type L1DCCR = crate::Reg<l1dccr::L1DCCRrs>;
///LTDC layerx default color configuration register
pub mod l1dccr;
/**L1BFCR (rw) register accessor: LTDC layerx blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`l1bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1BFCR)

For information about available fields see [`mod@l1bfcr`] module*/
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCRrs>;
///LTDC layerx blending factors configuration register
pub mod l1bfcr;
/**L1BLCR (rw) register accessor: LTDC layerx burst length configuration register

You can [`read`](crate::Reg::read) this register and get [`l1blcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1blcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1BLCR)

For information about available fields see [`mod@l1blcr`] module*/
pub type L1BLCR = crate::Reg<l1blcr::L1BLCRrs>;
///LTDC layerx burst length configuration register
pub mod l1blcr;
/**L1PCR (rw) register accessor: LTDC layerx planar configuration register

You can [`read`](crate::Reg::read) this register and get [`l1pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1PCR)

For information about available fields see [`mod@l1pcr`] module*/
pub type L1PCR = crate::Reg<l1pcr::L1PCRrs>;
///LTDC layerx planar configuration register
pub mod l1pcr;
/**L1CFBAR (rw) register accessor: LTDC layerx color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`l1cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CFBAR)

For information about available fields see [`mod@l1cfbar`] module*/
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBARrs>;
///LTDC layerx color frame buffer address register
pub mod l1cfbar;
/**L1CFBLR (rw) register accessor: LTDC layerx color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`l1cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CFBLR)

For information about available fields see [`mod@l1cfblr`] module*/
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLRrs>;
///LTDC layerx color frame buffer length register
pub mod l1cfblr;
/**L1CFBLNR (rw) register accessor: LTDC layerx color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`l1cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CFBLNR)

For information about available fields see [`mod@l1cfblnr`] module*/
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNRrs>;
///LTDC layerx color frame buffer line number register
pub mod l1cfblnr;
/**L1AFBA0R (rw) register accessor: LTDC layer1 auxiliary frame buffer address 0 register

You can [`read`](crate::Reg::read) this register and get [`l1afba0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afba0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBA0R)

For information about available fields see [`mod@l1afba0r`] module*/
pub type L1AFBA0R = crate::Reg<l1afba0r::L1AFBA0Rrs>;
///LTDC layer1 auxiliary frame buffer address 0 register
pub mod l1afba0r;
/**L1AFBA1R (rw) register accessor: LTDC layer1 auxiliary frame buffer address 1 register

You can [`read`](crate::Reg::read) this register and get [`l1afba1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afba1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBA1R)

For information about available fields see [`mod@l1afba1r`] module*/
pub type L1AFBA1R = crate::Reg<l1afba1r::L1AFBA1Rrs>;
///LTDC layer1 auxiliary frame buffer address 1 register
pub mod l1afba1r;
/**L1AFBLR (rw) register accessor: LTDC layer1 auxiliary frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`l1afblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBLR)

For information about available fields see [`mod@l1afblr`] module*/
pub type L1AFBLR = crate::Reg<l1afblr::L1AFBLRrs>;
///LTDC layer1 auxiliary frame buffer length register
pub mod l1afblr;
/**L1AFBLNR (rw) register accessor: LTDC layer1 auxiliary frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`l1afblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1afblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1AFBLNR)

For information about available fields see [`mod@l1afblnr`] module*/
pub type L1AFBLNR = crate::Reg<l1afblnr::L1AFBLNRrs>;
///LTDC layer1 auxiliary frame buffer line number register
pub mod l1afblnr;
/**L1CLUTWR (w) register accessor: LTDC layerx CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CLUTWR)

For information about available fields see [`mod@l1clutwr`] module*/
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWRrs>;
///LTDC layerx CLUT write register
pub mod l1clutwr;
/**L1CYR0R (rw) register accessor: LTDC layerx conversion YCbCr RGB 0 register

You can [`read`](crate::Reg::read) this register and get [`l1cyr0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cyr0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CYR0R)

For information about available fields see [`mod@l1cyr0r`] module*/
pub type L1CYR0R = crate::Reg<l1cyr0r::L1CYR0Rrs>;
///LTDC layerx conversion YCbCr RGB 0 register
pub mod l1cyr0r;
/**L1CYR1R (rw) register accessor: LTDC layerx conversion YCbCr RGB 1 register

You can [`read`](crate::Reg::read) this register and get [`l1cyr1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cyr1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1CYR1R)

For information about available fields see [`mod@l1cyr1r`] module*/
pub type L1CYR1R = crate::Reg<l1cyr1r::L1CYR1Rrs>;
///LTDC layerx conversion YCbCr RGB 1 register
pub mod l1cyr1r;
/**L1FPF0R (rw) register accessor: LTDC layerx flexible pixel format 0 register

You can [`read`](crate::Reg::read) this register and get [`l1fpf0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1fpf0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1FPF0R)

For information about available fields see [`mod@l1fpf0r`] module*/
pub type L1FPF0R = crate::Reg<l1fpf0r::L1FPF0Rrs>;
///LTDC layerx flexible pixel format 0 register
pub mod l1fpf0r;
/**L1FPF1R (rw) register accessor: LTDC layerx flexible pixel format 1 register

You can [`read`](crate::Reg::read) this register and get [`l1fpf1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1fpf1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L1FPF1R)

For information about available fields see [`mod@l1fpf1r`] module*/
pub type L1FPF1R = crate::Reg<l1fpf1r::L1FPF1Rrs>;
///LTDC layerx flexible pixel format 1 register
pub mod l1fpf1r;
/**L2C0R (r) register accessor: LTDC layerx configuration 0 register

You can [`read`](crate::Reg::read) this register and get [`l2c0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2C0R)

For information about available fields see [`mod@l2c0r`] module*/
pub type L2C0R = crate::Reg<l2c0r::L2C0Rrs>;
///LTDC layerx configuration 0 register
pub mod l2c0r;
/**L2C1R (rw) register accessor: LTDC layerx configuration 1 register

You can [`read`](crate::Reg::read) this register and get [`l2c1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2c1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2C1R)

For information about available fields see [`mod@l2c1r`] module*/
pub type L2C1R = crate::Reg<l2c1r::L2C1Rrs>;
///LTDC layerx configuration 1 register
pub mod l2c1r;
/**L2RCR (rw) register accessor: LTDC layerx reload control register

You can [`read`](crate::Reg::read) this register and get [`l2rcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2rcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2RCR)

For information about available fields see [`mod@l2rcr`] module*/
pub type L2RCR = crate::Reg<l2rcr::L2RCRrs>;
///LTDC layerx reload control register
pub mod l2rcr;
/**L2CR (rw) register accessor: LTDC layerx control register

You can [`read`](crate::Reg::read) this register and get [`l2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CR)

For information about available fields see [`mod@l2cr`] module*/
pub type L2CR = crate::Reg<l2cr::L2CRrs>;
///LTDC layerx control register
pub mod l2cr;
/**L2WHPCR (rw) register accessor: LTDC layerx window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`l2whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2WHPCR)

For information about available fields see [`mod@l2whpcr`] module*/
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCRrs>;
///LTDC layerx window horizontal position configuration register
pub mod l2whpcr;
/**L2WVPCR (rw) register accessor: LTDC layerx window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2WVPCR)

For information about available fields see [`mod@l2wvpcr`] module*/
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCRrs>;
///LTDC layerx window vertical position configuration register
pub mod l2wvpcr;
/**L2CKCR (rw) register accessor: LTDC layerx color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`l2ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CKCR)

For information about available fields see [`mod@l2ckcr`] module*/
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCRrs>;
///LTDC layerx color keying configuration register
pub mod l2ckcr;
/**L2PFCR (rw) register accessor: LTDC layerx pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`l2pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2PFCR)

For information about available fields see [`mod@l2pfcr`] module*/
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCRrs>;
///LTDC layerx pixel format configuration register
pub mod l2pfcr;
/**L2CACR (rw) register accessor: LTDC layerx constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`l2cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CACR)

For information about available fields see [`mod@l2cacr`] module*/
pub type L2CACR = crate::Reg<l2cacr::L2CACRrs>;
///LTDC layerx constant alpha configuration register
pub mod l2cacr;
/**L2DCCR (rw) register accessor: LTDC layerx default color configuration register

You can [`read`](crate::Reg::read) this register and get [`l2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2DCCR)

For information about available fields see [`mod@l2dccr`] module*/
pub type L2DCCR = crate::Reg<l2dccr::L2DCCRrs>;
///LTDC layerx default color configuration register
pub mod l2dccr;
/**L2BFCR (rw) register accessor: LTDC layerx blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`l2bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2BFCR)

For information about available fields see [`mod@l2bfcr`] module*/
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCRrs>;
///LTDC layerx blending factors configuration register
pub mod l2bfcr;
/**L2BLCR (rw) register accessor: LTDC layerx burst length configuration register

You can [`read`](crate::Reg::read) this register and get [`l2blcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2blcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2BLCR)

For information about available fields see [`mod@l2blcr`] module*/
pub type L2BLCR = crate::Reg<l2blcr::L2BLCRrs>;
///LTDC layerx burst length configuration register
pub mod l2blcr;
/**L2PCR (rw) register accessor: LTDC layerx planar configuration register

You can [`read`](crate::Reg::read) this register and get [`l2pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2PCR)

For information about available fields see [`mod@l2pcr`] module*/
pub type L2PCR = crate::Reg<l2pcr::L2PCRrs>;
///LTDC layerx planar configuration register
pub mod l2pcr;
/**L2CFBAR (rw) register accessor: LTDC layerx color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`l2cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CFBAR)

For information about available fields see [`mod@l2cfbar`] module*/
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBARrs>;
///LTDC layerx color frame buffer address register
pub mod l2cfbar;
/**L2CFBLR (rw) register accessor: LTDC layerx color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`l2cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CFBLR)

For information about available fields see [`mod@l2cfblr`] module*/
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLRrs>;
///LTDC layerx color frame buffer length register
pub mod l2cfblr;
/**L2CFBLNR (rw) register accessor: LTDC layerx color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`l2cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CFBLNR)

For information about available fields see [`mod@l2cfblnr`] module*/
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNRrs>;
///LTDC layerx color frame buffer line number register
pub mod l2cfblnr;
/**L2CLUTWR (w) register accessor: LTDC layerx CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CLUTWR)

For information about available fields see [`mod@l2clutwr`] module*/
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWRrs>;
///LTDC layerx CLUT write register
pub mod l2clutwr;
/**L2CYR0R (rw) register accessor: LTDC layerx conversion YCbCr RGB 0 register

You can [`read`](crate::Reg::read) this register and get [`l2cyr0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cyr0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CYR0R)

For information about available fields see [`mod@l2cyr0r`] module*/
pub type L2CYR0R = crate::Reg<l2cyr0r::L2CYR0Rrs>;
///LTDC layerx conversion YCbCr RGB 0 register
pub mod l2cyr0r;
/**L2CYR1R (rw) register accessor: LTDC layerx conversion YCbCr RGB 1 register

You can [`read`](crate::Reg::read) this register and get [`l2cyr1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cyr1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2CYR1R)

For information about available fields see [`mod@l2cyr1r`] module*/
pub type L2CYR1R = crate::Reg<l2cyr1r::L2CYR1Rrs>;
///LTDC layerx conversion YCbCr RGB 1 register
pub mod l2cyr1r;
/**L2FPF0R (rw) register accessor: LTDC layerx flexible pixel format 0 register

You can [`read`](crate::Reg::read) this register and get [`l2fpf0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2fpf0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2FPF0R)

For information about available fields see [`mod@l2fpf0r`] module*/
pub type L2FPF0R = crate::Reg<l2fpf0r::L2FPF0Rrs>;
///LTDC layerx flexible pixel format 0 register
pub mod l2fpf0r;
/**L2FPF1R (rw) register accessor: LTDC layerx flexible pixel format 1 register

You can [`read`](crate::Reg::read) this register and get [`l2fpf1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2fpf1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#LTDC:L2FPF1R)

For information about available fields see [`mod@l2fpf1r`] module*/
pub type L2FPF1R = crate::Reg<l2fpf1r::L2FPF1Rrs>;
///LTDC layerx flexible pixel format 1 register
pub mod l2fpf1r;
