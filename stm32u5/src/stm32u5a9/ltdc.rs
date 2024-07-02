#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    ltdc_sscr: LTDC_SSCR,
    ltdc_bpcr: LTDC_BPCR,
    ltdc_awcr: LTDC_AWCR,
    ltdc_twcr: LTDC_TWCR,
    ltdc_gcr: LTDC_GCR,
    _reserved5: [u8; 0x08],
    ltdc_srcr: LTDC_SRCR,
    _reserved6: [u8; 0x04],
    ltdc_bccr: LTDC_BCCR,
    _reserved7: [u8; 0x04],
    ltdc_ier: LTDC_IER,
    ltdc_isr: LTDC_ISR,
    ltdc_icr: LTDC_ICR,
    ltdc_lipcr: LTDC_LIPCR,
    ltdc_cpsr: LTDC_CPSR,
    ltdc_cdsr: LTDC_CDSR,
    _reserved13: [u8; 0x38],
    ltdc_l1cr: LTDC_L1CR,
    ltdc_l1whpcr: LTDC_L1WHPCR,
    ltdc_l1wvpcr: LTDC_L1WVPCR,
    ltdc_l1ckcr: LTDC_L1CKCR,
    ltdc_l1pfcr: LTDC_L1PFCR,
    ltdc_l1cacr: LTDC_L1CACR,
    ltdc_l1dccr: LTDC_L1DCCR,
    ltdc_l1bfcr: LTDC_L1BFCR,
    _reserved21: [u8; 0x08],
    ltdc_l1cfbar: LTDC_L1CFBAR,
    ltdc_l1cfblr: LTDC_L1CFBLR,
    ltdc_l1cfblnr: LTDC_L1CFBLNR,
    _reserved24: [u8; 0x0c],
    ltdc_l1clutwr: LTDC_L1CLUTWR,
    _reserved25: [u8; 0x3c],
    ltdc_l2cr: LTDC_L2CR,
    ltdc_l2whpcr: LTDC_L2WHPCR,
    ltdc_l2wvpcr: LTDC_L2WVPCR,
    ltdc_l2ckcr: LTDC_L2CKCR,
    ltdc_l2pfcr: LTDC_L2PFCR,
    ltdc_l2cacr: LTDC_L2CACR,
    ltdc_l2dccr: LTDC_L2DCCR,
    ltdc_l2bfcr: LTDC_L2BFCR,
    _reserved33: [u8; 0x08],
    ltdc_l2cfbar: LTDC_L2CFBAR,
    ltdc_l2cfblr: LTDC_L2CFBLR,
    ltdc_l2cfblnr: LTDC_L2CFBLNR,
    _reserved36: [u8; 0x0c],
    ltdc_l2clutwr: LTDC_L2CLUTWR,
}
impl RegisterBlock {
    ///0x08 - LTDC synchronization size configuration register
    #[inline(always)]
    pub const fn ltdc_sscr(&self) -> &LTDC_SSCR {
        &self.ltdc_sscr
    }
    ///0x0c - LTDC back porch configuration register
    #[inline(always)]
    pub const fn ltdc_bpcr(&self) -> &LTDC_BPCR {
        &self.ltdc_bpcr
    }
    ///0x10 - LTDC active width configuration register
    #[inline(always)]
    pub const fn ltdc_awcr(&self) -> &LTDC_AWCR {
        &self.ltdc_awcr
    }
    ///0x14 - LTDC total width configuration register
    #[inline(always)]
    pub const fn ltdc_twcr(&self) -> &LTDC_TWCR {
        &self.ltdc_twcr
    }
    ///0x18 - LTDC global control register
    #[inline(always)]
    pub const fn ltdc_gcr(&self) -> &LTDC_GCR {
        &self.ltdc_gcr
    }
    ///0x24 - LTDC shadow reload configuration register
    #[inline(always)]
    pub const fn ltdc_srcr(&self) -> &LTDC_SRCR {
        &self.ltdc_srcr
    }
    ///0x2c - LTDC background color configuration register
    #[inline(always)]
    pub const fn ltdc_bccr(&self) -> &LTDC_BCCR {
        &self.ltdc_bccr
    }
    ///0x34 - LTDC interrupt enable register
    #[inline(always)]
    pub const fn ltdc_ier(&self) -> &LTDC_IER {
        &self.ltdc_ier
    }
    ///0x38 - LTDC interrupt status register
    #[inline(always)]
    pub const fn ltdc_isr(&self) -> &LTDC_ISR {
        &self.ltdc_isr
    }
    ///0x3c -
    #[inline(always)]
    pub const fn ltdc_icr(&self) -> &LTDC_ICR {
        &self.ltdc_icr
    }
    ///0x40 - LTDC line interrupt position configuration register
    #[inline(always)]
    pub const fn ltdc_lipcr(&self) -> &LTDC_LIPCR {
        &self.ltdc_lipcr
    }
    ///0x44 -
    #[inline(always)]
    pub const fn ltdc_cpsr(&self) -> &LTDC_CPSR {
        &self.ltdc_cpsr
    }
    ///0x48 - LTDC current display status register
    #[inline(always)]
    pub const fn ltdc_cdsr(&self) -> &LTDC_CDSR {
        &self.ltdc_cdsr
    }
    ///0x84 -
    #[inline(always)]
    pub const fn ltdc_l1cr(&self) -> &LTDC_L1CR {
        &self.ltdc_l1cr
    }
    ///0x88 - LTDC layer 1 window horizontal position configuration register
    #[inline(always)]
    pub const fn ltdc_l1whpcr(&self) -> &LTDC_L1WHPCR {
        &self.ltdc_l1whpcr
    }
    ///0x8c - LTDC layer 1 window vertical position configuration register
    #[inline(always)]
    pub const fn ltdc_l1wvpcr(&self) -> &LTDC_L1WVPCR {
        &self.ltdc_l1wvpcr
    }
    ///0x90 - LTDC layer 1 color keying configuration register
    #[inline(always)]
    pub const fn ltdc_l1ckcr(&self) -> &LTDC_L1CKCR {
        &self.ltdc_l1ckcr
    }
    ///0x94 - LTDC layer 1 pixel format configuration register
    #[inline(always)]
    pub const fn ltdc_l1pfcr(&self) -> &LTDC_L1PFCR {
        &self.ltdc_l1pfcr
    }
    ///0x98 - LTDC layer 1 constant alpha configuration register
    #[inline(always)]
    pub const fn ltdc_l1cacr(&self) -> &LTDC_L1CACR {
        &self.ltdc_l1cacr
    }
    ///0x9c - LTDC layer 1 default color configuration register
    #[inline(always)]
    pub const fn ltdc_l1dccr(&self) -> &LTDC_L1DCCR {
        &self.ltdc_l1dccr
    }
    ///0xa0 - LTDC layer 1 blending factors configuration register
    #[inline(always)]
    pub const fn ltdc_l1bfcr(&self) -> &LTDC_L1BFCR {
        &self.ltdc_l1bfcr
    }
    ///0xac - LTDC layer 1 color frame buffer address register
    #[inline(always)]
    pub const fn ltdc_l1cfbar(&self) -> &LTDC_L1CFBAR {
        &self.ltdc_l1cfbar
    }
    ///0xb0 - LTDC layer 1 color frame buffer length register
    #[inline(always)]
    pub const fn ltdc_l1cfblr(&self) -> &LTDC_L1CFBLR {
        &self.ltdc_l1cfblr
    }
    ///0xb4 - LTDC layer 1 color frame buffer line number register
    #[inline(always)]
    pub const fn ltdc_l1cfblnr(&self) -> &LTDC_L1CFBLNR {
        &self.ltdc_l1cfblnr
    }
    ///0xc4 - LTDC layer 1 CLUT write register
    #[inline(always)]
    pub const fn ltdc_l1clutwr(&self) -> &LTDC_L1CLUTWR {
        &self.ltdc_l1clutwr
    }
    ///0x104 -
    #[inline(always)]
    pub const fn ltdc_l2cr(&self) -> &LTDC_L2CR {
        &self.ltdc_l2cr
    }
    ///0x108 - LTDC layer 2 window horizontal position configuration register
    #[inline(always)]
    pub const fn ltdc_l2whpcr(&self) -> &LTDC_L2WHPCR {
        &self.ltdc_l2whpcr
    }
    ///0x10c - LTDC layer 2 window vertical position configuration register
    #[inline(always)]
    pub const fn ltdc_l2wvpcr(&self) -> &LTDC_L2WVPCR {
        &self.ltdc_l2wvpcr
    }
    ///0x110 - LTDC layer 2 color keying configuration register
    #[inline(always)]
    pub const fn ltdc_l2ckcr(&self) -> &LTDC_L2CKCR {
        &self.ltdc_l2ckcr
    }
    ///0x114 - LTDC layer 2 pixel format configuration register
    #[inline(always)]
    pub const fn ltdc_l2pfcr(&self) -> &LTDC_L2PFCR {
        &self.ltdc_l2pfcr
    }
    ///0x118 - LTDC layer 2 constant alpha configuration register
    #[inline(always)]
    pub const fn ltdc_l2cacr(&self) -> &LTDC_L2CACR {
        &self.ltdc_l2cacr
    }
    ///0x11c - LTDC layer 2 default color configuration register
    #[inline(always)]
    pub const fn ltdc_l2dccr(&self) -> &LTDC_L2DCCR {
        &self.ltdc_l2dccr
    }
    ///0x120 - LTDC layer 2 blending factors configuration register
    #[inline(always)]
    pub const fn ltdc_l2bfcr(&self) -> &LTDC_L2BFCR {
        &self.ltdc_l2bfcr
    }
    ///0x12c - LTDC layer 2 color frame buffer address register
    #[inline(always)]
    pub const fn ltdc_l2cfbar(&self) -> &LTDC_L2CFBAR {
        &self.ltdc_l2cfbar
    }
    ///0x130 - LTDC layer 2 color frame buffer length register
    #[inline(always)]
    pub const fn ltdc_l2cfblr(&self) -> &LTDC_L2CFBLR {
        &self.ltdc_l2cfblr
    }
    ///0x134 - LTDC layer 2 color frame buffer line number register
    #[inline(always)]
    pub const fn ltdc_l2cfblnr(&self) -> &LTDC_L2CFBLNR {
        &self.ltdc_l2cfblnr
    }
    ///0x144 - LTDC layer 2 CLUT write register
    #[inline(always)]
    pub const fn ltdc_l2clutwr(&self) -> &LTDC_L2CLUTWR {
        &self.ltdc_l2clutwr
    }
}
/**LTDC_SSCR (rw) register accessor: LTDC synchronization size configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_sscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_sscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_SSCR)

For information about available fields see [`mod@ltdc_sscr`]
module*/
pub type LTDC_SSCR = crate::Reg<ltdc_sscr::LTDC_SSCRrs>;
///LTDC synchronization size configuration register
pub mod ltdc_sscr;
/**LTDC_BPCR (rw) register accessor: LTDC back porch configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_bpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_bpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_BPCR)

For information about available fields see [`mod@ltdc_bpcr`]
module*/
pub type LTDC_BPCR = crate::Reg<ltdc_bpcr::LTDC_BPCRrs>;
///LTDC back porch configuration register
pub mod ltdc_bpcr;
/**LTDC_AWCR (rw) register accessor: LTDC active width configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_awcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_awcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_AWCR)

For information about available fields see [`mod@ltdc_awcr`]
module*/
pub type LTDC_AWCR = crate::Reg<ltdc_awcr::LTDC_AWCRrs>;
///LTDC active width configuration register
pub mod ltdc_awcr;
/**LTDC_TWCR (rw) register accessor: LTDC total width configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_twcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_twcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_TWCR)

For information about available fields see [`mod@ltdc_twcr`]
module*/
pub type LTDC_TWCR = crate::Reg<ltdc_twcr::LTDC_TWCRrs>;
///LTDC total width configuration register
pub mod ltdc_twcr;
/**LTDC_GCR (rw) register accessor: LTDC global control register

You can [`read`](crate::Reg::read) this register and get [`ltdc_gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_GCR)

For information about available fields see [`mod@ltdc_gcr`]
module*/
pub type LTDC_GCR = crate::Reg<ltdc_gcr::LTDC_GCRrs>;
///LTDC global control register
pub mod ltdc_gcr;
/**LTDC_SRCR (rw) register accessor: LTDC shadow reload configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_srcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_srcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_SRCR)

For information about available fields see [`mod@ltdc_srcr`]
module*/
pub type LTDC_SRCR = crate::Reg<ltdc_srcr::LTDC_SRCRrs>;
///LTDC shadow reload configuration register
pub mod ltdc_srcr;
/**LTDC_BCCR (rw) register accessor: LTDC background color configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_bccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_bccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_BCCR)

For information about available fields see [`mod@ltdc_bccr`]
module*/
pub type LTDC_BCCR = crate::Reg<ltdc_bccr::LTDC_BCCRrs>;
///LTDC background color configuration register
pub mod ltdc_bccr;
/**LTDC_IER (rw) register accessor: LTDC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ltdc_ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_IER)

For information about available fields see [`mod@ltdc_ier`]
module*/
pub type LTDC_IER = crate::Reg<ltdc_ier::LTDC_IERrs>;
///LTDC interrupt enable register
pub mod ltdc_ier;
/**LTDC_ISR (r) register accessor: LTDC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ltdc_isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_ISR)

For information about available fields see [`mod@ltdc_isr`]
module*/
pub type LTDC_ISR = crate::Reg<ltdc_isr::LTDC_ISRrs>;
///LTDC interrupt status register
pub mod ltdc_isr;
/**LTDC_ICR (w) register accessor:

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_ICR)

For information about available fields see [`mod@ltdc_icr`]
module*/
pub type LTDC_ICR = crate::Reg<ltdc_icr::LTDC_ICRrs>;
///
pub mod ltdc_icr;
/**LTDC_LIPCR (rw) register accessor: LTDC line interrupt position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_lipcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_lipcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_LIPCR)

For information about available fields see [`mod@ltdc_lipcr`]
module*/
pub type LTDC_LIPCR = crate::Reg<ltdc_lipcr::LTDC_LIPCRrs>;
///LTDC line interrupt position configuration register
pub mod ltdc_lipcr;
/**LTDC_CPSR (r) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ltdc_cpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_CPSR)

For information about available fields see [`mod@ltdc_cpsr`]
module*/
pub type LTDC_CPSR = crate::Reg<ltdc_cpsr::LTDC_CPSRrs>;
///
pub mod ltdc_cpsr;
/**LTDC_CDSR (r) register accessor: LTDC current display status register

You can [`read`](crate::Reg::read) this register and get [`ltdc_cdsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_CDSR)

For information about available fields see [`mod@ltdc_cdsr`]
module*/
pub type LTDC_CDSR = crate::Reg<ltdc_cdsr::LTDC_CDSRrs>;
///LTDC current display status register
pub mod ltdc_cdsr;
/**LTDC_L1CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CR)

For information about available fields see [`mod@ltdc_l1cr`]
module*/
pub type LTDC_L1CR = crate::Reg<ltdc_l1cr::LTDC_L1CRrs>;
///
pub mod ltdc_l1cr;
/**LTDC_L1WHPCR (rw) register accessor: LTDC layer 1 window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1WHPCR)

For information about available fields see [`mod@ltdc_l1whpcr`]
module*/
pub type LTDC_L1WHPCR = crate::Reg<ltdc_l1whpcr::LTDC_L1WHPCRrs>;
///LTDC layer 1 window horizontal position configuration register
pub mod ltdc_l1whpcr;
/**LTDC_L1WVPCR (rw) register accessor: LTDC layer 1 window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1WVPCR)

For information about available fields see [`mod@ltdc_l1wvpcr`]
module*/
pub type LTDC_L1WVPCR = crate::Reg<ltdc_l1wvpcr::LTDC_L1WVPCRrs>;
///LTDC layer 1 window vertical position configuration register
pub mod ltdc_l1wvpcr;
/**LTDC_L1CKCR (rw) register accessor: LTDC layer 1 color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CKCR)

For information about available fields see [`mod@ltdc_l1ckcr`]
module*/
pub type LTDC_L1CKCR = crate::Reg<ltdc_l1ckcr::LTDC_L1CKCRrs>;
///LTDC layer 1 color keying configuration register
pub mod ltdc_l1ckcr;
/**LTDC_L1PFCR (rw) register accessor: LTDC layer 1 pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1PFCR)

For information about available fields see [`mod@ltdc_l1pfcr`]
module*/
pub type LTDC_L1PFCR = crate::Reg<ltdc_l1pfcr::LTDC_L1PFCRrs>;
///LTDC layer 1 pixel format configuration register
pub mod ltdc_l1pfcr;
/**LTDC_L1CACR (rw) register accessor: LTDC layer 1 constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CACR)

For information about available fields see [`mod@ltdc_l1cacr`]
module*/
pub type LTDC_L1CACR = crate::Reg<ltdc_l1cacr::LTDC_L1CACRrs>;
///LTDC layer 1 constant alpha configuration register
pub mod ltdc_l1cacr;
/**LTDC_L1DCCR (rw) register accessor: LTDC layer 1 default color configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1DCCR)

For information about available fields see [`mod@ltdc_l1dccr`]
module*/
pub type LTDC_L1DCCR = crate::Reg<ltdc_l1dccr::LTDC_L1DCCRrs>;
///LTDC layer 1 default color configuration register
pub mod ltdc_l1dccr;
/**LTDC_L1BFCR (rw) register accessor: LTDC layer 1 blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1BFCR)

For information about available fields see [`mod@ltdc_l1bfcr`]
module*/
pub type LTDC_L1BFCR = crate::Reg<ltdc_l1bfcr::LTDC_L1BFCRrs>;
///LTDC layer 1 blending factors configuration register
pub mod ltdc_l1bfcr;
/**LTDC_L1CFBAR (rw) register accessor: LTDC layer 1 color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CFBAR)

For information about available fields see [`mod@ltdc_l1cfbar`]
module*/
pub type LTDC_L1CFBAR = crate::Reg<ltdc_l1cfbar::LTDC_L1CFBARrs>;
///LTDC layer 1 color frame buffer address register
pub mod ltdc_l1cfbar;
/**LTDC_L1CFBLR (rw) register accessor: LTDC layer 1 color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CFBLR)

For information about available fields see [`mod@ltdc_l1cfblr`]
module*/
pub type LTDC_L1CFBLR = crate::Reg<ltdc_l1cfblr::LTDC_L1CFBLRrs>;
///LTDC layer 1 color frame buffer length register
pub mod ltdc_l1cfblr;
/**LTDC_L1CFBLNR (rw) register accessor: LTDC layer 1 color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CFBLNR)

For information about available fields see [`mod@ltdc_l1cfblnr`]
module*/
pub type LTDC_L1CFBLNR = crate::Reg<ltdc_l1cfblnr::LTDC_L1CFBLNRrs>;
///LTDC layer 1 color frame buffer line number register
pub mod ltdc_l1cfblnr;
/**LTDC_L1CLUTWR (w) register accessor: LTDC layer 1 CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L1CLUTWR)

For information about available fields see [`mod@ltdc_l1clutwr`]
module*/
pub type LTDC_L1CLUTWR = crate::Reg<ltdc_l1clutwr::LTDC_L1CLUTWRrs>;
///LTDC layer 1 CLUT write register
pub mod ltdc_l1clutwr;
/**LTDC_L2CR (rw) register accessor:

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CR)

For information about available fields see [`mod@ltdc_l2cr`]
module*/
pub type LTDC_L2CR = crate::Reg<ltdc_l2cr::LTDC_L2CRrs>;
///
pub mod ltdc_l2cr;
/**LTDC_L2WHPCR (rw) register accessor: LTDC layer 2 window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2whpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2whpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2WHPCR)

For information about available fields see [`mod@ltdc_l2whpcr`]
module*/
pub type LTDC_L2WHPCR = crate::Reg<ltdc_l2whpcr::LTDC_L2WHPCRrs>;
///LTDC layer 2 window horizontal position configuration register
pub mod ltdc_l2whpcr;
/**LTDC_L2WVPCR (rw) register accessor: LTDC layer 2 window vertical position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2wvpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2wvpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2WVPCR)

For information about available fields see [`mod@ltdc_l2wvpcr`]
module*/
pub type LTDC_L2WVPCR = crate::Reg<ltdc_l2wvpcr::LTDC_L2WVPCRrs>;
///LTDC layer 2 window vertical position configuration register
pub mod ltdc_l2wvpcr;
/**LTDC_L2CKCR (rw) register accessor: LTDC layer 2 color keying configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CKCR)

For information about available fields see [`mod@ltdc_l2ckcr`]
module*/
pub type LTDC_L2CKCR = crate::Reg<ltdc_l2ckcr::LTDC_L2CKCRrs>;
///LTDC layer 2 color keying configuration register
pub mod ltdc_l2ckcr;
/**LTDC_L2PFCR (rw) register accessor: LTDC layer 2 pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2pfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2pfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2PFCR)

For information about available fields see [`mod@ltdc_l2pfcr`]
module*/
pub type LTDC_L2PFCR = crate::Reg<ltdc_l2pfcr::LTDC_L2PFCRrs>;
///LTDC layer 2 pixel format configuration register
pub mod ltdc_l2pfcr;
/**LTDC_L2CACR (rw) register accessor: LTDC layer 2 constant alpha configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CACR)

For information about available fields see [`mod@ltdc_l2cacr`]
module*/
pub type LTDC_L2CACR = crate::Reg<ltdc_l2cacr::LTDC_L2CACRrs>;
///LTDC layer 2 constant alpha configuration register
pub mod ltdc_l2cacr;
/**LTDC_L2DCCR (rw) register accessor: LTDC layer 2 default color configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2DCCR)

For information about available fields see [`mod@ltdc_l2dccr`]
module*/
pub type LTDC_L2DCCR = crate::Reg<ltdc_l2dccr::LTDC_L2DCCRrs>;
///LTDC layer 2 default color configuration register
pub mod ltdc_l2dccr;
/**LTDC_L2BFCR (rw) register accessor: LTDC layer 2 blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2bfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2bfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2BFCR)

For information about available fields see [`mod@ltdc_l2bfcr`]
module*/
pub type LTDC_L2BFCR = crate::Reg<ltdc_l2bfcr::LTDC_L2BFCRrs>;
///LTDC layer 2 blending factors configuration register
pub mod ltdc_l2bfcr;
/**LTDC_L2CFBAR (rw) register accessor: LTDC layer 2 color frame buffer address register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CFBAR)

For information about available fields see [`mod@ltdc_l2cfbar`]
module*/
pub type LTDC_L2CFBAR = crate::Reg<ltdc_l2cfbar::LTDC_L2CFBARrs>;
///LTDC layer 2 color frame buffer address register
pub mod ltdc_l2cfbar;
/**LTDC_L2CFBLR (rw) register accessor: LTDC layer 2 color frame buffer length register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfblr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfblr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CFBLR)

For information about available fields see [`mod@ltdc_l2cfblr`]
module*/
pub type LTDC_L2CFBLR = crate::Reg<ltdc_l2cfblr::LTDC_L2CFBLRrs>;
///LTDC layer 2 color frame buffer length register
pub mod ltdc_l2cfblr;
/**LTDC_L2CFBLNR (rw) register accessor: LTDC layer 2 color frame buffer line number register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cfblnr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cfblnr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CFBLNR)

For information about available fields see [`mod@ltdc_l2cfblnr`]
module*/
pub type LTDC_L2CFBLNR = crate::Reg<ltdc_l2cfblnr::LTDC_L2CFBLNRrs>;
///LTDC layer 2 color frame buffer line number register
pub mod ltdc_l2cfblnr;
/**LTDC_L2CLUTWR (w) register accessor: LTDC layer 2 CLUT write register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2clutwr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CLUTWR)

For information about available fields see [`mod@ltdc_l2clutwr`]
module*/
pub type LTDC_L2CLUTWR = crate::Reg<ltdc_l2clutwr::LTDC_L2CLUTWRrs>;
///LTDC layer 2 CLUT write register
pub mod ltdc_l2clutwr;
