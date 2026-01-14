#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    cgcr: CGCR,
    tcr: TCR,
    tdr: TDR,
    evcr: EVCR,
    evsr: EVSR,
    _reserved6: [u8; 0x08],
    wdgtcr: WDGTCR,
    _reserved7: [u8; 0x0c],
    isr: ISR,
    icr: ICR,
    ier: IER,
    tsr: TSR,
    lccrr: LCCRR,
    fccrr: FCCRR,
    _reserved13: [u8; 0x08],
    atr: ATR,
    afcr: AFCR,
    alcr: ALCR,
    _reserved16: [u8; 0x04],
    afcc1r: AFCC1R,
    _reserved17: [u8; 0x0c],
    alcc1r: ALCC1R,
    alcc2r: ALCC2R,
    _reserved19: [u8; 0x08],
    rfc1r: RFC1R,
    rfc1rr: RFC1RR,
    rfc2r: RFC2R,
    rfc2rr: RFC2RR,
    _reserved23: [u8; 0x10],
    wdgcr: WDGCR,
    wdgrr: WDGRR,
    wdgpar: WDGPAR,
}
impl RegisterBlock {
    ///0x00 - GFXTIM configuration register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - GFXTIM clock generator configuration register
    #[inline(always)]
    pub const fn cgcr(&self) -> &CGCR {
        &self.cgcr
    }
    ///0x08 - GFXTIM timers configuration register
    #[inline(always)]
    pub const fn tcr(&self) -> &TCR {
        &self.tcr
    }
    ///0x0c - GFXTIM timers disable register
    #[inline(always)]
    pub const fn tdr(&self) -> &TDR {
        &self.tdr
    }
    ///0x10 - GFXTIM events control register
    #[inline(always)]
    pub const fn evcr(&self) -> &EVCR {
        &self.evcr
    }
    ///0x14 - GFXTIM events selection register
    #[inline(always)]
    pub const fn evsr(&self) -> &EVSR {
        &self.evsr
    }
    ///0x20 - GFXTIM watchdog timer configuration register
    #[inline(always)]
    pub const fn wdgtcr(&self) -> &WDGTCR {
        &self.wdgtcr
    }
    ///0x30 - GFXTIM interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x34 - GFXTIM interrupt clear register
    #[inline(always)]
    pub const fn icr(&self) -> &ICR {
        &self.icr
    }
    ///0x38 - GFXTIM interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x3c - GFXTIM timers status register
    #[inline(always)]
    pub const fn tsr(&self) -> &TSR {
        &self.tsr
    }
    ///0x40 - GFXTIM line clock counter reload register
    #[inline(always)]
    pub const fn lccrr(&self) -> &LCCRR {
        &self.lccrr
    }
    ///0x44 - GFXTIM frame clock counter reload register
    #[inline(always)]
    pub const fn fccrr(&self) -> &FCCRR {
        &self.fccrr
    }
    ///0x50 - GFXTIM absolute time register
    #[inline(always)]
    pub const fn atr(&self) -> &ATR {
        &self.atr
    }
    ///0x54 - GFXTIM absolute frame counter register
    #[inline(always)]
    pub const fn afcr(&self) -> &AFCR {
        &self.afcr
    }
    ///0x58 - GFXTIM absolute line counter register
    #[inline(always)]
    pub const fn alcr(&self) -> &ALCR {
        &self.alcr
    }
    ///0x60 - GFXTIM absolute frame counter compare 1 register
    #[inline(always)]
    pub const fn afcc1r(&self) -> &AFCC1R {
        &self.afcc1r
    }
    ///0x70 - GFXTIM absolute line counter compare 1 register
    #[inline(always)]
    pub const fn alcc1r(&self) -> &ALCC1R {
        &self.alcc1r
    }
    ///0x74 - GFXTIM absolute line counter compare 2 register
    #[inline(always)]
    pub const fn alcc2r(&self) -> &ALCC2R {
        &self.alcc2r
    }
    ///0x80 - GFXTIM relative frame counter 1 register
    #[inline(always)]
    pub const fn rfc1r(&self) -> &RFC1R {
        &self.rfc1r
    }
    ///0x84 - GFXTIM relative frame counter 1 reload register
    #[inline(always)]
    pub const fn rfc1rr(&self) -> &RFC1RR {
        &self.rfc1rr
    }
    ///0x88 - GFXTIM relative frame counter 2 register
    #[inline(always)]
    pub const fn rfc2r(&self) -> &RFC2R {
        &self.rfc2r
    }
    ///0x8c - GFXTIM relative frame counter 2 reload register
    #[inline(always)]
    pub const fn rfc2rr(&self) -> &RFC2RR {
        &self.rfc2rr
    }
    ///0xa0 - GFXTIM watchdog counter register
    #[inline(always)]
    pub const fn wdgcr(&self) -> &WDGCR {
        &self.wdgcr
    }
    ///0xa4 - GFXTIM watchdog reload register
    #[inline(always)]
    pub const fn wdgrr(&self) -> &WDGRR {
        &self.wdgrr
    }
    ///0xa8 - GFXTIM watchdog pre-alarm register
    #[inline(always)]
    pub const fn wdgpar(&self) -> &WDGPAR {
        &self.wdgpar
    }
}
/**CR (rw) register accessor: GFXTIM configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///GFXTIM configuration register
pub mod cr;
/**CGCR (rw) register accessor: GFXTIM clock generator configuration register

You can [`read`](crate::Reg::read) this register and get [`cgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:CGCR)

For information about available fields see [`mod@cgcr`] module*/
pub type CGCR = crate::Reg<cgcr::CGCRrs>;
///GFXTIM clock generator configuration register
pub mod cgcr;
/**TCR (rw) register accessor: GFXTIM timers configuration register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:TCR)

For information about available fields see [`mod@tcr`] module*/
pub type TCR = crate::Reg<tcr::TCRrs>;
///GFXTIM timers configuration register
pub mod tcr;
/**TDR (w) register accessor: GFXTIM timers disable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:TDR)

For information about available fields see [`mod@tdr`] module*/
pub type TDR = crate::Reg<tdr::TDRrs>;
///GFXTIM timers disable register
pub mod tdr;
/**EVCR (rw) register accessor: GFXTIM events control register

You can [`read`](crate::Reg::read) this register and get [`evcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:EVCR)

For information about available fields see [`mod@evcr`] module*/
pub type EVCR = crate::Reg<evcr::EVCRrs>;
///GFXTIM events control register
pub mod evcr;
/**EVSR (rw) register accessor: GFXTIM events selection register

You can [`read`](crate::Reg::read) this register and get [`evsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:EVSR)

For information about available fields see [`mod@evsr`] module*/
pub type EVSR = crate::Reg<evsr::EVSRrs>;
///GFXTIM events selection register
pub mod evsr;
/**WDGTCR (rw) register accessor: GFXTIM watchdog timer configuration register

You can [`read`](crate::Reg::read) this register and get [`wdgtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:WDGTCR)

For information about available fields see [`mod@wdgtcr`] module*/
pub type WDGTCR = crate::Reg<wdgtcr::WDGTCRrs>;
///GFXTIM watchdog timer configuration register
pub mod wdgtcr;
/**ISR (r) register accessor: GFXTIM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///GFXTIM interrupt status register
pub mod isr;
/**ICR (w) register accessor: GFXTIM interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ICR)

For information about available fields see [`mod@icr`] module*/
pub type ICR = crate::Reg<icr::ICRrs>;
///GFXTIM interrupt clear register
pub mod icr;
/**IER (rw) register accessor: GFXTIM interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///GFXTIM interrupt enable register
pub mod ier;
/**TSR (r) register accessor: GFXTIM timers status register

You can [`read`](crate::Reg::read) this register and get [`tsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:TSR)

For information about available fields see [`mod@tsr`] module*/
pub type TSR = crate::Reg<tsr::TSRrs>;
///GFXTIM timers status register
pub mod tsr;
/**LCCRR (rw) register accessor: GFXTIM line clock counter reload register

You can [`read`](crate::Reg::read) this register and get [`lccrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:LCCRR)

For information about available fields see [`mod@lccrr`] module*/
pub type LCCRR = crate::Reg<lccrr::LCCRRrs>;
///GFXTIM line clock counter reload register
pub mod lccrr;
/**FCCRR (rw) register accessor: GFXTIM frame clock counter reload register

You can [`read`](crate::Reg::read) this register and get [`fccrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fccrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:FCCRR)

For information about available fields see [`mod@fccrr`] module*/
pub type FCCRR = crate::Reg<fccrr::FCCRRrs>;
///GFXTIM frame clock counter reload register
pub mod fccrr;
/**ATR (r) register accessor: GFXTIM absolute time register

You can [`read`](crate::Reg::read) this register and get [`atr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ATR)

For information about available fields see [`mod@atr`] module*/
pub type ATR = crate::Reg<atr::ATRrs>;
///GFXTIM absolute time register
pub mod atr;
/**AFCR (rw) register accessor: GFXTIM absolute frame counter register

You can [`read`](crate::Reg::read) this register and get [`afcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:AFCR)

For information about available fields see [`mod@afcr`] module*/
pub type AFCR = crate::Reg<afcr::AFCRrs>;
///GFXTIM absolute frame counter register
pub mod afcr;
/**ALCR (rw) register accessor: GFXTIM absolute line counter register

You can [`read`](crate::Reg::read) this register and get [`alcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ALCR)

For information about available fields see [`mod@alcr`] module*/
pub type ALCR = crate::Reg<alcr::ALCRrs>;
///GFXTIM absolute line counter register
pub mod alcr;
/**AFCC1R (rw) register accessor: GFXTIM absolute frame counter compare 1 register

You can [`read`](crate::Reg::read) this register and get [`afcc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`afcc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:AFCC1R)

For information about available fields see [`mod@afcc1r`] module*/
pub type AFCC1R = crate::Reg<afcc1r::AFCC1Rrs>;
///GFXTIM absolute frame counter compare 1 register
pub mod afcc1r;
/**ALCC1R (rw) register accessor: GFXTIM absolute line counter compare 1 register

You can [`read`](crate::Reg::read) this register and get [`alcc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ALCC1R)

For information about available fields see [`mod@alcc1r`] module*/
pub type ALCC1R = crate::Reg<alcc1r::ALCC1Rrs>;
///GFXTIM absolute line counter compare 1 register
pub mod alcc1r;
/**ALCC2R (rw) register accessor: GFXTIM absolute line counter compare 2 register

You can [`read`](crate::Reg::read) this register and get [`alcc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alcc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:ALCC2R)

For information about available fields see [`mod@alcc2r`] module*/
pub type ALCC2R = crate::Reg<alcc2r::ALCC2Rrs>;
///GFXTIM absolute line counter compare 2 register
pub mod alcc2r;
/**RFC1R (r) register accessor: GFXTIM relative frame counter 1 register

You can [`read`](crate::Reg::read) this register and get [`rfc1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:RFC1R)

For information about available fields see [`mod@rfc1r`] module*/
pub type RFC1R = crate::Reg<rfc1r::RFC1Rrs>;
///GFXTIM relative frame counter 1 register
pub mod rfc1r;
/**RFC1RR (rw) register accessor: GFXTIM relative frame counter 1 reload register

You can [`read`](crate::Reg::read) this register and get [`rfc1rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc1rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:RFC1RR)

For information about available fields see [`mod@rfc1rr`] module*/
pub type RFC1RR = crate::Reg<rfc1rr::RFC1RRrs>;
///GFXTIM relative frame counter 1 reload register
pub mod rfc1rr;
/**RFC2R (r) register accessor: GFXTIM relative frame counter 2 register

You can [`read`](crate::Reg::read) this register and get [`rfc2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:RFC2R)

For information about available fields see [`mod@rfc2r`] module*/
pub type RFC2R = crate::Reg<rfc2r::RFC2Rrs>;
///GFXTIM relative frame counter 2 register
pub mod rfc2r;
/**RFC2RR (rw) register accessor: GFXTIM relative frame counter 2 reload register

You can [`read`](crate::Reg::read) this register and get [`rfc2rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfc2rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:RFC2RR)

For information about available fields see [`mod@rfc2rr`] module*/
pub type RFC2RR = crate::Reg<rfc2rr::RFC2RRrs>;
///GFXTIM relative frame counter 2 reload register
pub mod rfc2rr;
/**WDGCR (r) register accessor: GFXTIM watchdog counter register

You can [`read`](crate::Reg::read) this register and get [`wdgcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:WDGCR)

For information about available fields see [`mod@wdgcr`] module*/
pub type WDGCR = crate::Reg<wdgcr::WDGCRrs>;
///GFXTIM watchdog counter register
pub mod wdgcr;
/**WDGRR (rw) register accessor: GFXTIM watchdog reload register

You can [`read`](crate::Reg::read) this register and get [`wdgrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:WDGRR)

For information about available fields see [`mod@wdgrr`] module*/
pub type WDGRR = crate::Reg<wdgrr::WDGRRrs>;
///GFXTIM watchdog reload register
pub mod wdgrr;
/**WDGPAR (rw) register accessor: GFXTIM watchdog pre-alarm register

You can [`read`](crate::Reg::read) this register and get [`wdgpar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdgpar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#GFXTIM:WDGPAR)

For information about available fields see [`mod@wdgpar`] module*/
pub type WDGPAR = crate::Reg<wdgpar::WDGPARrs>;
///GFXTIM watchdog pre-alarm register
pub mod wdgpar;
