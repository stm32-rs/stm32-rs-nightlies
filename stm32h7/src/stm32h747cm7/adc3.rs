#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr: CFGR,
    cfgr2: CFGR2,
    smpr1: SMPR1,
    smpr2: SMPR2,
    pcsel: PCSEL,
    ltr1: LTR1,
    htr1: HTR1,
    _reserved10: [u8; 0x08],
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    sqr4: SQR4,
    dr: DR,
    _reserved15: [u8; 0x08],
    jsqr: JSQR,
    _reserved16: [u8; 0x10],
    ofr: [OFR; 4],
    _reserved17: [u8; 0x10],
    jdr: [JDR; 4],
    _reserved18: [u8; 0x10],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    _reserved20: [u8; 0x08],
    ltr2: LTR2,
    htr2: HTR2,
    ltr3: LTR3,
    htr3: HTR3,
    difsel: DIFSEL,
    calfact: CALFACT,
    calfact2: CALFACT2,
}
impl RegisterBlock {
    ///0x00 - ADC interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - ADC interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - ADC control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - ADC configuration register 1
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - ADC configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x14 - ADC sampling time register 1
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    ///0x18 - ADC sampling time register 2
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    ///0x1c - ADC pre channel selection register
    #[inline(always)]
    pub const fn pcsel(&self) -> &PCSEL {
        &self.pcsel
    }
    ///0x20 - ADC analog watchdog 1 threshold register
    #[inline(always)]
    pub const fn ltr1(&self) -> &LTR1 {
        &self.ltr1
    }
    ///0x24 - ADC analog watchdog 2 threshold register
    #[inline(always)]
    pub const fn htr1(&self) -> &HTR1 {
        &self.htr1
    }
    ///0x30 - ADC group regular sequencer ranks register 1
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    ///0x34 - ADC group regular sequencer ranks register 2
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    ///0x38 - ADC group regular sequencer ranks register 3
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    ///0x3c - ADC group regular sequencer ranks register 4
    #[inline(always)]
    pub const fn sqr4(&self) -> &SQR4 {
        &self.sqr4
    }
    ///0x40 - ADC group regular conversion data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x4c - ADC group injected sequencer register
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    ///0x60..0x70 - ADC offset number %s register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `OFR1` register.</div>
    #[inline(always)]
    pub const fn ofr(&self, n: usize) -> &OFR {
        &self.ofr[n]
    }
    ///Iterator for array of:
    ///0x60..0x70 - ADC offset number %s register
    #[inline(always)]
    pub fn ofr_iter(&self) -> impl Iterator<Item = &OFR> {
        self.ofr.iter()
    }
    ///0x60 - ADC offset number 1 register
    #[inline(always)]
    pub const fn ofr1(&self) -> &OFR {
        self.ofr(0)
    }
    ///0x64 - ADC offset number 2 register
    #[inline(always)]
    pub const fn ofr2(&self) -> &OFR {
        self.ofr(1)
    }
    ///0x68 - ADC offset number 3 register
    #[inline(always)]
    pub const fn ofr3(&self) -> &OFR {
        self.ofr(2)
    }
    ///0x6c - ADC offset number 4 register
    #[inline(always)]
    pub const fn ofr4(&self) -> &OFR {
        self.ofr(3)
    }
    ///0x80..0x90 - ADC group injected sequencer rank %s register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `JDR1` register.</div>
    #[inline(always)]
    pub const fn jdr(&self, n: usize) -> &JDR {
        &self.jdr[n]
    }
    ///Iterator for array of:
    ///0x80..0x90 - ADC group injected sequencer rank %s register
    #[inline(always)]
    pub fn jdr_iter(&self) -> impl Iterator<Item = &JDR> {
        self.jdr.iter()
    }
    ///0x80 - ADC group injected sequencer rank 1 register
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR {
        self.jdr(0)
    }
    ///0x84 - ADC group injected sequencer rank 2 register
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR {
        self.jdr(1)
    }
    ///0x88 - ADC group injected sequencer rank 3 register
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR {
        self.jdr(2)
    }
    ///0x8c - ADC group injected sequencer rank 4 register
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR {
        self.jdr(3)
    }
    ///0xa0 - ADC analog watchdog 2 configuration register
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    ///0xa4 - ADC analog watchdog 3 configuration register
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    ///0xb0 - ADC watchdog lower threshold register 2
    #[inline(always)]
    pub const fn ltr2(&self) -> &LTR2 {
        &self.ltr2
    }
    ///0xb4 - ADC watchdog higher threshold register 2
    #[inline(always)]
    pub const fn htr2(&self) -> &HTR2 {
        &self.htr2
    }
    ///0xb8 - ADC watchdog lower threshold register 3
    #[inline(always)]
    pub const fn ltr3(&self) -> &LTR3 {
        &self.ltr3
    }
    ///0xbc - ADC watchdog higher threshold register 3
    #[inline(always)]
    pub const fn htr3(&self) -> &HTR3 {
        &self.htr3
    }
    ///0xc0 - ADC channel differential or single-ended mode selection register
    #[inline(always)]
    pub const fn difsel(&self) -> &DIFSEL {
        &self.difsel
    }
    ///0xc4 - ADC calibration factors register
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    ///0xc8 - ADC Calibration Factor register 2
    #[inline(always)]
    pub const fn calfact2(&self) -> &CALFACT2 {
        &self.calfact2
    }
}
/**ISR (rw) register accessor: ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///ADC interrupt and status register
pub mod isr;
/**IER (rw) register accessor: ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///ADC interrupt enable register
pub mod ier;
/**CR (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///ADC control register
pub mod cr;
/**CFGR (rw) register accessor: ADC configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:CFGR)

For information about available fields see [`mod@cfgr`] module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///ADC configuration register 1
pub mod cfgr;
/**CFGR2 (rw) register accessor: ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///ADC configuration register 2
pub mod cfgr2;
/**SMPR1 (rw) register accessor: ADC sampling time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SMPR1)

For information about available fields see [`mod@smpr1`] module*/
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
///ADC sampling time register 1
pub mod smpr1;
/**SMPR2 (rw) register accessor: ADC sampling time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SMPR2)

For information about available fields see [`mod@smpr2`] module*/
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
///ADC sampling time register 2
pub mod smpr2;
/**LTR1 (rw) register accessor: ADC analog watchdog 1 threshold register

You can [`read`](crate::Reg::read) this register and get [`ltr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:LTR1)

For information about available fields see [`mod@ltr1`] module*/
pub type LTR1 = crate::Reg<ltr1::LTR1rs>;
///ADC analog watchdog 1 threshold register
pub mod ltr1;
/**HTR1 (rw) register accessor: ADC analog watchdog 2 threshold register

You can [`read`](crate::Reg::read) this register and get [`htr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:HTR1)

For information about available fields see [`mod@htr1`] module*/
pub type HTR1 = crate::Reg<htr1::HTR1rs>;
///ADC analog watchdog 2 threshold register
pub mod htr1;
/**SQR1 (rw) register accessor: ADC group regular sequencer ranks register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SQR1)

For information about available fields see [`mod@sqr1`] module*/
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
///ADC group regular sequencer ranks register 1
pub mod sqr1;
/**SQR2 (rw) register accessor: ADC group regular sequencer ranks register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SQR2)

For information about available fields see [`mod@sqr2`] module*/
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
///ADC group regular sequencer ranks register 2
pub mod sqr2;
/**SQR3 (rw) register accessor: ADC group regular sequencer ranks register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SQR3)

For information about available fields see [`mod@sqr3`] module*/
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
///ADC group regular sequencer ranks register 3
pub mod sqr3;
/**SQR4 (rw) register accessor: ADC group regular sequencer ranks register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:SQR4)

For information about available fields see [`mod@sqr4`] module*/
pub type SQR4 = crate::Reg<sqr4::SQR4rs>;
///ADC group regular sequencer ranks register 4
pub mod sqr4;
/**DR (r) register accessor: ADC group regular conversion data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///ADC group regular conversion data register
pub mod dr;
/**JSQR (rw) register accessor: ADC group injected sequencer register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:JSQR)

For information about available fields see [`mod@jsqr`] module*/
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
///ADC group injected sequencer register
pub mod jsqr;
/**OFR (rw) register accessor: ADC offset number %s register

You can [`read`](crate::Reg::read) this register and get [`ofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:OFR[1])

For information about available fields see [`mod@ofr`] module*/
pub type OFR = crate::Reg<ofr::OFRrs>;
///ADC offset number %s register
pub mod ofr;
/**JDR (r) register accessor: ADC group injected sequencer rank %s register

You can [`read`](crate::Reg::read) this register and get [`jdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:JDR[1])

For information about available fields see [`mod@jdr`] module*/
pub type JDR = crate::Reg<jdr::JDRrs>;
///ADC group injected sequencer rank %s register
pub mod jdr;
/**AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:AWD2CR)

For information about available fields see [`mod@awd2cr`] module*/
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
///ADC analog watchdog 2 configuration register
pub mod awd2cr;
/**AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:AWD3CR)

For information about available fields see [`mod@awd3cr`] module*/
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
///ADC analog watchdog 3 configuration register
pub mod awd3cr;
/**DIFSEL (rw) register accessor: ADC channel differential or single-ended mode selection register

You can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:DIFSEL)

For information about available fields see [`mod@difsel`] module*/
pub type DIFSEL = crate::Reg<difsel::DIFSELrs>;
///ADC channel differential or single-ended mode selection register
pub mod difsel;
/**CALFACT (rw) register accessor: ADC calibration factors register

You can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:CALFACT)

For information about available fields see [`mod@calfact`] module*/
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
///ADC calibration factors register
pub mod calfact;
/**PCSEL (rw) register accessor: ADC pre channel selection register

You can [`read`](crate::Reg::read) this register and get [`pcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:PCSEL)

For information about available fields see [`mod@pcsel`] module*/
pub type PCSEL = crate::Reg<pcsel::PCSELrs>;
///ADC pre channel selection register
pub mod pcsel;
/**LTR2 (rw) register accessor: ADC watchdog lower threshold register 2

You can [`read`](crate::Reg::read) this register and get [`ltr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:LTR2)

For information about available fields see [`mod@ltr2`] module*/
pub type LTR2 = crate::Reg<ltr2::LTR2rs>;
///ADC watchdog lower threshold register 2
pub mod ltr2;
/**HTR2 (rw) register accessor: ADC watchdog higher threshold register 2

You can [`read`](crate::Reg::read) this register and get [`htr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:HTR2)

For information about available fields see [`mod@htr2`] module*/
pub type HTR2 = crate::Reg<htr2::HTR2rs>;
///ADC watchdog higher threshold register 2
pub mod htr2;
/**LTR3 (rw) register accessor: ADC watchdog lower threshold register 3

You can [`read`](crate::Reg::read) this register and get [`ltr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:LTR3)

For information about available fields see [`mod@ltr3`] module*/
pub type LTR3 = crate::Reg<ltr3::LTR3rs>;
///ADC watchdog lower threshold register 3
pub mod ltr3;
/**HTR3 (rw) register accessor: ADC watchdog higher threshold register 3

You can [`read`](crate::Reg::read) this register and get [`htr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:HTR3)

For information about available fields see [`mod@htr3`] module*/
pub type HTR3 = crate::Reg<htr3::HTR3rs>;
///ADC watchdog higher threshold register 3
pub mod htr3;
/**CALFACT2 (rw) register accessor: ADC Calibration Factor register 2

You can [`read`](crate::Reg::read) this register and get [`calfact2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#ADC3:CALFACT2)

For information about available fields see [`mod@calfact2`] module*/
pub type CALFACT2 = crate::Reg<calfact2::CALFACT2rs>;
///ADC Calibration Factor register 2
pub mod calfact2;
