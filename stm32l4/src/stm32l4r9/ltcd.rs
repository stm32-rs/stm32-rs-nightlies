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
    _reserved5: [u8; 0x08],
    srcr: SRCR,
    _reserved6: [u8; 0x04],
    bccr: BCCR,
    _reserved7: [u8; 0x04],
    ier: IER,
    isr: ISR,
    icr: ICR,
    lipcr: LIPCR,
    cpsr: CPSR,
    cdsr: CDSR,
    _reserved13: [u8; 0x38],
    l1cr: L1CR,
    l1whpcr: L1WHPCR,
    l1wvpcr: L1WVPCR,
    l1ckcr: L1CKCR,
    l1pfcr: L1PFCR,
    l1cacr: L1CACR,
    l1dccr: L1DCCR,
    l1bfcr: L1BFCR,
    _reserved21: [u8; 0x08],
    l1cfbar: L1CFBAR,
    l1cfblr: L1CFBLR,
    l1cfblnr: L1CFBLNR,
    _reserved24: [u8; 0x0c],
    l1clutwr: L1CLUTWR,
    _reserved25: [u8; 0x3c],
    l2cr: L2CR,
    l2whpcr: L2WHPCR,
    l2wvpcr: L2WVPCR,
    l2ckcr: L2CKCR,
    l2pfcr: L2PFCR,
    l2cacr: L2CACR,
    l2dccr: L2DCCR,
    _reserved32: [u8; 0x04],
    l2bfcr: L2BFCR,
    _reserved33: [u8; 0x04],
    l2cfbar: L2CFBAR,
    l2cfblr: L2CFBLR,
    l2cfblnr: L2CFBLNR,
    _reserved36: [u8; 0x0c],
    l2clutwr: L2CLUTWR,
}
impl RegisterBlock {
    ///0x08 - LTDC Synchronization Size Configuration Register
    #[inline(always)]
    pub const fn sscr(&self) -> &SSCR {
        &self.sscr
    }
    ///0x0c - LTDC Back Porch Configuration Register
    #[inline(always)]
    pub const fn bpcr(&self) -> &BPCR {
        &self.bpcr
    }
    ///0x10 - LTDC Active Width Configuration Register
    #[inline(always)]
    pub const fn awcr(&self) -> &AWCR {
        &self.awcr
    }
    ///0x14 - LTDC Total Width Configuration Register
    #[inline(always)]
    pub const fn twcr(&self) -> &TWCR {
        &self.twcr
    }
    ///0x18 - LTDC Global Control Register
    #[inline(always)]
    pub const fn gcr(&self) -> &GCR {
        &self.gcr
    }
    ///0x24 - LTDC Shadow Reload Configuration Register
    #[inline(always)]
    pub const fn srcr(&self) -> &SRCR {
        &self.srcr
    }
    ///0x2c - LTDC Background Color Configuration Register
    #[inline(always)]
    pub const fn bccr(&self) -> &BCCR {
        &self.bccr
    }
    ///0x34 - LTDC Interrupt Enable Register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x38 - LTDC Interrupt Status Register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x3c - LTDC Interrupt Clear Register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x40 - LTDC Line Interrupt Position Configuration Register
    #[inline(always)]
    pub const fn lipcr(&self) -> &LIPCR {
        &self.lipcr
    }
    ///0x44 - LTDC Current Position Status Register
    #[inline(always)]
    pub const fn cpsr(&self) -> &CPSR {
        &self.cpsr
    }
    ///0x48 - LTDC Current Display Status Register
    #[inline(always)]
    pub const fn cdsr(&self) -> &CDSR {
        &self.cdsr
    }
    ///0x84 - LTDC Layer Control Register
    #[inline(always)]
    pub const fn l1cr(&self) -> &L1CR {
        &self.l1cr
    }
    ///0x88 - LTDC Layer Window Horizontal Position Configuration Register
    #[inline(always)]
    pub const fn l1whpcr(&self) -> &L1WHPCR {
        &self.l1whpcr
    }
    ///0x8c - LTDC Layer Window Vertical Position Configuration Register
    #[inline(always)]
    pub const fn l1wvpcr(&self) -> &L1WVPCR {
        &self.l1wvpcr
    }
    ///0x90 - LTDC Layer Color Keying Configuration Register
    #[inline(always)]
    pub const fn l1ckcr(&self) -> &L1CKCR {
        &self.l1ckcr
    }
    ///0x94 - LTDC Layer Pixel Format Configuration Register
    #[inline(always)]
    pub const fn l1pfcr(&self) -> &L1PFCR {
        &self.l1pfcr
    }
    ///0x98 - LTDC Layer Constant Alpha Configuration Register
    #[inline(always)]
    pub const fn l1cacr(&self) -> &L1CACR {
        &self.l1cacr
    }
    ///0x9c - LTDC Layer Default Color Configuration Register
    #[inline(always)]
    pub const fn l1dccr(&self) -> &L1DCCR {
        &self.l1dccr
    }
    ///0xa0 - LTDC Layer Blending Factors Configuration Register
    #[inline(always)]
    pub const fn l1bfcr(&self) -> &L1BFCR {
        &self.l1bfcr
    }
    ///0xac - LTDC Layer Color Frame Buffer Address Register
    #[inline(always)]
    pub const fn l1cfbar(&self) -> &L1CFBAR {
        &self.l1cfbar
    }
    ///0xb0 - LTDC Layer Color Frame Buffer Length Register
    #[inline(always)]
    pub const fn l1cfblr(&self) -> &L1CFBLR {
        &self.l1cfblr
    }
    ///0xb4 - LTDC Layer ColorFrame Buffer Line Number Register
    #[inline(always)]
    pub const fn l1cfblnr(&self) -> &L1CFBLNR {
        &self.l1cfblnr
    }
    ///0xc4 - LTDC Layerx CLUT Write Register
    #[inline(always)]
    pub const fn l1clutwr(&self) -> &L1CLUTWR {
        &self.l1clutwr
    }
    ///0x104 - LTDC Layer Control Register
    #[inline(always)]
    pub const fn l2cr(&self) -> &L2CR {
        &self.l2cr
    }
    ///0x108 - LTDC Layerx Window Horizontal Position Configuration Register
    #[inline(always)]
    pub const fn l2whpcr(&self) -> &L2WHPCR {
        &self.l2whpcr
    }
    ///0x10c - LTDC Layer Window Vertical Position Configuration Register
    #[inline(always)]
    pub const fn l2wvpcr(&self) -> &L2WVPCR {
        &self.l2wvpcr
    }
    ///0x110 - LTDC Layer Color Keying Configuration Register
    #[inline(always)]
    pub const fn l2ckcr(&self) -> &L2CKCR {
        &self.l2ckcr
    }
    ///0x114 - LTDC Layer Pixel Format Configuration Register
    #[inline(always)]
    pub const fn l2pfcr(&self) -> &L2PFCR {
        &self.l2pfcr
    }
    ///0x118 - LTDC Layer Constant Alpha Configuration Register
    #[inline(always)]
    pub const fn l2cacr(&self) -> &L2CACR {
        &self.l2cacr
    }
    ///0x11c - LTDC Layer Default Color Configuration Register
    #[inline(always)]
    pub const fn l2dccr(&self) -> &L2DCCR {
        &self.l2dccr
    }
    ///0x124 - LTDC Layer Blending Factors Configuration Register
    #[inline(always)]
    pub const fn l2bfcr(&self) -> &L2BFCR {
        &self.l2bfcr
    }
    ///0x12c - LTDC Layer Color Frame Buffer Address Register
    #[inline(always)]
    pub const fn l2cfbar(&self) -> &L2CFBAR {
        &self.l2cfbar
    }
    ///0x130 - LTDC Layer Color Frame Buffer Length Register
    #[inline(always)]
    pub const fn l2cfblr(&self) -> &L2CFBLR {
        &self.l2cfblr
    }
    ///0x134 - LTDC Layer ColorFrame Buffer Line Number Register
    #[inline(always)]
    pub const fn l2cfblnr(&self) -> &L2CFBLNR {
        &self.l2cfblnr
    }
    ///0x144 - LTDC Layerx CLUT Write Register
    #[inline(always)]
    pub const fn l2clutwr(&self) -> &L2CLUTWR {
        &self.l2clutwr
    }
}
/**SSCR (rw) register accessor: LTDC Synchronization Size Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:SSCR)

For information about available fields see [`mod@sscr`]
module*/
pub type SSCR = crate::Reg<sscr::SSCRrs>;
///LTDC Synchronization Size Configuration Register
pub mod sscr;
/**BPCR (rw) register accessor: LTDC Back Porch Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:BPCR)

For information about available fields see [`mod@bpcr`]
module*/
pub type BPCR = crate::Reg<bpcr::BPCRrs>;
///LTDC Back Porch Configuration Register
pub mod bpcr;
/**AWCR (rw) register accessor: LTDC Active Width Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:AWCR)

For information about available fields see [`mod@awcr`]
module*/
pub type AWCR = crate::Reg<awcr::AWCRrs>;
///LTDC Active Width Configuration Register
pub mod awcr;
/**TWCR (rw) register accessor: LTDC Total Width Configuration Register

You can [`read`](crate::Reg::read) this register and get [`twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:TWCR)

For information about available fields see [`mod@twcr`]
module*/
pub type TWCR = crate::Reg<twcr::TWCRrs>;
///LTDC Total Width Configuration Register
pub mod twcr;
/**GCR (rw) register accessor: LTDC Global Control Register

You can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:GCR)

For information about available fields see [`mod@gcr`]
module*/
pub type GCR = crate::Reg<gcr::GCRrs>;
///LTDC Global Control Register
pub mod gcr;
/**SRCR (rw) register accessor: LTDC Shadow Reload Configuration Register

You can [`read`](crate::Reg::read) this register and get [`srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:SRCR)

For information about available fields see [`mod@srcr`]
module*/
pub type SRCR = crate::Reg<srcr::SRCRrs>;
///LTDC Shadow Reload Configuration Register
pub mod srcr;
/**BCCR (rw) register accessor: LTDC Background Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:BCCR)

For information about available fields see [`mod@bccr`]
module*/
pub type BCCR = crate::Reg<bccr::BCCRrs>;
///LTDC Background Color Configuration Register
pub mod bccr;
/**IER (rw) register accessor: LTDC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:IER)

For information about available fields see [`mod@ier`]
module*/
pub type IER = crate::Reg<ier::IERrs>;
///LTDC Interrupt Enable Register
pub mod ier;
/**ISR (r) register accessor: LTDC Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:ISR)

For information about available fields see [`mod@isr`]
module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///LTDC Interrupt Status Register
pub mod isr;
/**ICR (w) register accessor: LTDC Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:ICR)

For information about available fields see [`mod@icr`]
module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///LTDC Interrupt Clear Register
pub mod icr;
/**LIPCR (rw) register accessor: LTDC Line Interrupt Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:LIPCR)

For information about available fields see [`mod@lipcr`]
module*/
pub type LIPCR = crate::Reg<lipcr::LIPCRrs>;
///LTDC Line Interrupt Position Configuration Register
pub mod lipcr;
/**CPSR (r) register accessor: LTDC Current Position Status Register

You can [`read`](crate::Reg::read) this register and get [`cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:CPSR)

For information about available fields see [`mod@cpsr`]
module*/
pub type CPSR = crate::Reg<cpsr::CPSRrs>;
///LTDC Current Position Status Register
pub mod cpsr;
/**CDSR (r) register accessor: LTDC Current Display Status Register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:CDSR)

For information about available fields see [`mod@cdsr`]
module*/
pub type CDSR = crate::Reg<cdsr::CDSRrs>;
///LTDC Current Display Status Register
pub mod cdsr;
/**L1CR (rw) register accessor: LTDC Layer Control Register

You can [`read`](crate::Reg::read) this register and get [`l1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CR)

For information about available fields see [`mod@l1cr`]
module*/
pub type L1CR = crate::Reg<l1cr::L1CRrs>;
///LTDC Layer Control Register
pub mod l1cr;
/**L2CR (rw) register accessor: LTDC Layer Control Register

You can [`read`](crate::Reg::read) this register and get [`l2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CR)

For information about available fields see [`mod@l2cr`]
module*/
pub type L2CR = crate::Reg<l2cr::L2CRrs>;
///LTDC Layer Control Register
pub mod l2cr;
/**L1WHPCR (rw) register accessor: LTDC Layer Window Horizontal Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1WHPCR)

For information about available fields see [`mod@l1whpcr`]
module*/
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCRrs>;
///LTDC Layer Window Horizontal Position Configuration Register
pub mod l1whpcr;
/**L2WHPCR (rw) register accessor: LTDC Layerx Window Horizontal Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2WHPCR)

For information about available fields see [`mod@l2whpcr`]
module*/
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCRrs>;
///LTDC Layerx Window Horizontal Position Configuration Register
pub mod l2whpcr;
/**L1WVPCR (rw) register accessor: LTDC Layer Window Vertical Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1WVPCR)

For information about available fields see [`mod@l1wvpcr`]
module*/
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCRrs>;
///LTDC Layer Window Vertical Position Configuration Register
pub mod l1wvpcr;
/**L2WVPCR (rw) register accessor: LTDC Layer Window Vertical Position Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2WVPCR)

For information about available fields see [`mod@l2wvpcr`]
module*/
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCRrs>;
///LTDC Layer Window Vertical Position Configuration Register
pub mod l2wvpcr;
/**L1CKCR (rw) register accessor: LTDC Layer Color Keying Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CKCR)

For information about available fields see [`mod@l1ckcr`]
module*/
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCRrs>;
///LTDC Layer Color Keying Configuration Register
pub mod l1ckcr;
/**L2CKCR (rw) register accessor: LTDC Layer Color Keying Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CKCR)

For information about available fields see [`mod@l2ckcr`]
module*/
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCRrs>;
///LTDC Layer Color Keying Configuration Register
pub mod l2ckcr;
/**L1PFCR (rw) register accessor: LTDC Layer Pixel Format Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1PFCR)

For information about available fields see [`mod@l1pfcr`]
module*/
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCRrs>;
///LTDC Layer Pixel Format Configuration Register
pub mod l1pfcr;
/**L2PFCR (rw) register accessor: LTDC Layer Pixel Format Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2PFCR)

For information about available fields see [`mod@l2pfcr`]
module*/
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCRrs>;
///LTDC Layer Pixel Format Configuration Register
pub mod l2pfcr;
/**L1CACR (rw) register accessor: LTDC Layer Constant Alpha Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CACR)

For information about available fields see [`mod@l1cacr`]
module*/
pub type L1CACR = crate::Reg<l1cacr::L1CACRrs>;
///LTDC Layer Constant Alpha Configuration Register
pub mod l1cacr;
/**L2CACR (rw) register accessor: LTDC Layer Constant Alpha Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CACR)

For information about available fields see [`mod@l2cacr`]
module*/
pub type L2CACR = crate::Reg<l2cacr::L2CACRrs>;
///LTDC Layer Constant Alpha Configuration Register
pub mod l2cacr;
/**L1DCCR (rw) register accessor: LTDC Layer Default Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1DCCR)

For information about available fields see [`mod@l1dccr`]
module*/
pub type L1DCCR = crate::Reg<l1dccr::L1DCCRrs>;
///LTDC Layer Default Color Configuration Register
pub mod l1dccr;
/**L2DCCR (rw) register accessor: LTDC Layer Default Color Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2DCCR)

For information about available fields see [`mod@l2dccr`]
module*/
pub type L2DCCR = crate::Reg<l2dccr::L2DCCRrs>;
///LTDC Layer Default Color Configuration Register
pub mod l2dccr;
/**L1BFCR (rw) register accessor: LTDC Layer Blending Factors Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l1bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1BFCR)

For information about available fields see [`mod@l1bfcr`]
module*/
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCRrs>;
///LTDC Layer Blending Factors Configuration Register
pub mod l1bfcr;
/**L2BFCR (rw) register accessor: LTDC Layer Blending Factors Configuration Register

You can [`read`](crate::Reg::read) this register and get [`l2bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2BFCR)

For information about available fields see [`mod@l2bfcr`]
module*/
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCRrs>;
///LTDC Layer Blending Factors Configuration Register
pub mod l2bfcr;
/**L1CFBAR (rw) register accessor: LTDC Layer Color Frame Buffer Address Register

You can [`read`](crate::Reg::read) this register and get [`l1cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CFBAR)

For information about available fields see [`mod@l1cfbar`]
module*/
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBARrs>;
///LTDC Layer Color Frame Buffer Address Register
pub mod l1cfbar;
/**L2CFBAR (rw) register accessor: LTDC Layer Color Frame Buffer Address Register

You can [`read`](crate::Reg::read) this register and get [`l2cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CFBAR)

For information about available fields see [`mod@l2cfbar`]
module*/
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBARrs>;
///LTDC Layer Color Frame Buffer Address Register
pub mod l2cfbar;
/**L1CFBLR (rw) register accessor: LTDC Layer Color Frame Buffer Length Register

You can [`read`](crate::Reg::read) this register and get [`l1cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CFBLR)

For information about available fields see [`mod@l1cfblr`]
module*/
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLRrs>;
///LTDC Layer Color Frame Buffer Length Register
pub mod l1cfblr;
/**L2CFBLR (rw) register accessor: LTDC Layer Color Frame Buffer Length Register

You can [`read`](crate::Reg::read) this register and get [`l2cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CFBLR)

For information about available fields see [`mod@l2cfblr`]
module*/
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLRrs>;
///LTDC Layer Color Frame Buffer Length Register
pub mod l2cfblr;
/**L1CFBLNR (rw) register accessor: LTDC Layer ColorFrame Buffer Line Number Register

You can [`read`](crate::Reg::read) this register and get [`l1cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CFBLNR)

For information about available fields see [`mod@l1cfblnr`]
module*/
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNRrs>;
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l1cfblnr;
/**L2CFBLNR (rw) register accessor: LTDC Layer ColorFrame Buffer Line Number Register

You can [`read`](crate::Reg::read) this register and get [`l2cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CFBLNR)

For information about available fields see [`mod@l2cfblnr`]
module*/
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNRrs>;
///LTDC Layer ColorFrame Buffer Line Number Register
pub mod l2cfblnr;
/**L1CLUTWR (w) register accessor: LTDC Layerx CLUT Write Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L1CLUTWR)

For information about available fields see [`mod@l1clutwr`]
module*/
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWRrs>;
///LTDC Layerx CLUT Write Register
pub mod l1clutwr;
/**L2CLUTWR (w) register accessor: LTDC Layerx CLUT Write Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R9.html#LTCD:L2CLUTWR)

For information about available fields see [`mod@l2clutwr`]
module*/
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWRrs>;
///LTDC Layerx CLUT Write Register
pub mod l2clutwr;
