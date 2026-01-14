#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    isr: ISR,
    ier: IER,
    cr: CR,
    cfgr1: CFGR1,
    cfgr2: CFGR2,
    smpr1: SMPR1,
    smpr2: SMPR2,
    pcsel: PCSEL,
    _reserved8: [u8; 0x10],
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    sqr4: SQR4,
    dr: DR,
    _reserved13: [u8; 0x08],
    jsqr: JSQR,
    ofcfgr1: OFCFGR1,
    ofcfgr2: OFCFGR2,
    ofcfgr3: OFCFGR3,
    ofcfgr4: OFCFGR4,
    ofr1: OFR1,
    ofr2: OFR2,
    ofr3: OFR3,
    ofr4: OFR4,
    gcomp: GCOMP,
    _reserved23: [u8; 0x0c],
    jdr1: JDR1,
    jdr2: JDR2,
    jdr3: JDR3,
    jdr4: JDR4,
    _reserved27: [u8; 0x10],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    awd1ltr: AWD1LTR,
    awd1htr: AWD1HTR,
    awd2ltr: AWD2LTR,
    awd2htr: AWD2HTR,
    awd3ltr: AWD3LTR,
    awd3htr: AWD3HTR,
    difsel: DIFSEL,
    calfact: CALFACT,
    _reserved37: [u8; 0x08],
    or: OR,
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
    ///0x0c - ADC configuration register
    #[inline(always)]
    pub const fn cfgr1(&self) -> &CFGR1 {
        &self.cfgr1
    }
    ///0x10 - ADC configuration register 2
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x14 - ADC sample time register 1
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    ///0x18 - ADC sample time register 2
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    ///0x1c - ADC channel preselection register
    #[inline(always)]
    pub const fn pcsel(&self) -> &PCSEL {
        &self.pcsel
    }
    ///0x30 - ADC regular sequence register 1
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    ///0x34 - ADC regular sequence register 2
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    ///0x38 - ADC regular sequence register 3
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    ///0x3c - ADC regular sequence register 4
    #[inline(always)]
    pub const fn sqr4(&self) -> &SQR4 {
        &self.sqr4
    }
    ///0x40 - ADC regular data register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x4c - ADC injected sequence register
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    ///0x50 - ADC offset 1 configuration register
    #[inline(always)]
    pub const fn ofcfgr1(&self) -> &OFCFGR1 {
        &self.ofcfgr1
    }
    ///0x54 - ADC offset 2 configuration register
    #[inline(always)]
    pub const fn ofcfgr2(&self) -> &OFCFGR2 {
        &self.ofcfgr2
    }
    ///0x58 - ADC offset 3 configuration register
    #[inline(always)]
    pub const fn ofcfgr3(&self) -> &OFCFGR3 {
        &self.ofcfgr3
    }
    ///0x5c - ADC offset 4 configuration register
    #[inline(always)]
    pub const fn ofcfgr4(&self) -> &OFCFGR4 {
        &self.ofcfgr4
    }
    ///0x60 - ADC offset 1 register
    #[inline(always)]
    pub const fn ofr1(&self) -> &OFR1 {
        &self.ofr1
    }
    ///0x64 - ADC offset 2 register
    #[inline(always)]
    pub const fn ofr2(&self) -> &OFR2 {
        &self.ofr2
    }
    ///0x68 - ADC offset 3 register
    #[inline(always)]
    pub const fn ofr3(&self) -> &OFR3 {
        &self.ofr3
    }
    ///0x6c - ADC offset 4 register
    #[inline(always)]
    pub const fn ofr4(&self) -> &OFR4 {
        &self.ofr4
    }
    ///0x70 - ADC gain compensation register
    #[inline(always)]
    pub const fn gcomp(&self) -> &GCOMP {
        &self.gcomp
    }
    ///0x80 - ADC injected channel 1 data register
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR1 {
        &self.jdr1
    }
    ///0x84 - ADC injected channel 2 data register
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR2 {
        &self.jdr2
    }
    ///0x88 - ADC injected channel 3 data register
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR3 {
        &self.jdr3
    }
    ///0x8c - ADC injected channel 4 data register
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR4 {
        &self.jdr4
    }
    ///0xa0 - ADC Analog Watchdog 2 Configuration Register
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    ///0xa4 - ADC Analog Watchdog 3 Configuration Register
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    ///0xa8 - ADC analog watchdog 1 lower threshold register
    #[inline(always)]
    pub const fn awd1ltr(&self) -> &AWD1LTR {
        &self.awd1ltr
    }
    ///0xac - ADC analog watchdog 1 higher threshold register
    #[inline(always)]
    pub const fn awd1htr(&self) -> &AWD1HTR {
        &self.awd1htr
    }
    ///0xb0 - ADC analog watchdog 2 lower threshold register
    #[inline(always)]
    pub const fn awd2ltr(&self) -> &AWD2LTR {
        &self.awd2ltr
    }
    ///0xb4 - ADC analog watchdog 2 higher threshold register
    #[inline(always)]
    pub const fn awd2htr(&self) -> &AWD2HTR {
        &self.awd2htr
    }
    ///0xb8 - ADC analog watchdog 3 lower threshold register
    #[inline(always)]
    pub const fn awd3ltr(&self) -> &AWD3LTR {
        &self.awd3ltr
    }
    ///0xbc - ADC analog watchdog 3 higher threshold register
    #[inline(always)]
    pub const fn awd3htr(&self) -> &AWD3HTR {
        &self.awd3htr
    }
    ///0xc0 - ADC Differential mode selection register
    #[inline(always)]
    pub const fn difsel(&self) -> &DIFSEL {
        &self.difsel
    }
    ///0xc4 - ADC calibration factors
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
    ///0xd0 - ADC option register
    #[inline(always)]
    pub const fn or(&self) -> &OR {
        &self.or
    }
}
/**ISR (rw) register accessor: ADC interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///ADC interrupt and status register
pub mod isr;
/**IER (rw) register accessor: ADC interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:IER)

For information about available fields see [`mod@ier`] module*/
pub type IER = crate::Reg<ier::IERrs>;
///ADC interrupt enable register
pub mod ier;
/**CR (rw) register accessor: ADC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///ADC control register
pub mod cr;
/**CFGR1 (rw) register accessor: ADC configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CFGR1)

For information about available fields see [`mod@cfgr1`] module*/
pub type CFGR1 = crate::Reg<cfgr1::CFGR1rs>;
///ADC configuration register
pub mod cfgr1;
/**CFGR2 (rw) register accessor: ADC configuration register 2

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CFGR2)

For information about available fields see [`mod@cfgr2`] module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///ADC configuration register 2
pub mod cfgr2;
/**SMPR1 (rw) register accessor: ADC sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SMPR1)

For information about available fields see [`mod@smpr1`] module*/
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
///ADC sample time register 1
pub mod smpr1;
/**SMPR2 (rw) register accessor: ADC sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SMPR2)

For information about available fields see [`mod@smpr2`] module*/
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
///ADC sample time register 2
pub mod smpr2;
/**PCSEL (rw) register accessor: ADC channel preselection register

You can [`read`](crate::Reg::read) this register and get [`pcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:PCSEL)

For information about available fields see [`mod@pcsel`] module*/
pub type PCSEL = crate::Reg<pcsel::PCSELrs>;
///ADC channel preselection register
pub mod pcsel;
/**SQR1 (rw) register accessor: ADC regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SQR1)

For information about available fields see [`mod@sqr1`] module*/
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
///ADC regular sequence register 1
pub mod sqr1;
/**SQR2 (rw) register accessor: ADC regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SQR2)

For information about available fields see [`mod@sqr2`] module*/
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
///ADC regular sequence register 2
pub mod sqr2;
/**SQR3 (rw) register accessor: ADC regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SQR3)

For information about available fields see [`mod@sqr3`] module*/
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
///ADC regular sequence register 3
pub mod sqr3;
/**SQR4 (rw) register accessor: ADC regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:SQR4)

For information about available fields see [`mod@sqr4`] module*/
pub type SQR4 = crate::Reg<sqr4::SQR4rs>;
///ADC regular sequence register 4
pub mod sqr4;
/**DR (r) register accessor: ADC regular data register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:DR)

For information about available fields see [`mod@dr`] module*/
pub type DR = crate::Reg<dr::DRrs>;
///ADC regular data register
pub mod dr;
/**JSQR (rw) register accessor: ADC injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JSQR)

For information about available fields see [`mod@jsqr`] module*/
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
///ADC injected sequence register
pub mod jsqr;
/**OFCFGR1 (rw) register accessor: ADC offset 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`ofcfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofcfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFCFGR1)

For information about available fields see [`mod@ofcfgr1`] module*/
pub type OFCFGR1 = crate::Reg<ofcfgr1::OFCFGR1rs>;
///ADC offset 1 configuration register
pub mod ofcfgr1;
/**OFCFGR2 (rw) register accessor: ADC offset 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`ofcfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofcfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFCFGR2)

For information about available fields see [`mod@ofcfgr2`] module*/
pub type OFCFGR2 = crate::Reg<ofcfgr2::OFCFGR2rs>;
///ADC offset 2 configuration register
pub mod ofcfgr2;
/**OFCFGR3 (rw) register accessor: ADC offset 3 configuration register

You can [`read`](crate::Reg::read) this register and get [`ofcfgr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofcfgr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFCFGR3)

For information about available fields see [`mod@ofcfgr3`] module*/
pub type OFCFGR3 = crate::Reg<ofcfgr3::OFCFGR3rs>;
///ADC offset 3 configuration register
pub mod ofcfgr3;
/**OFCFGR4 (rw) register accessor: ADC offset 4 configuration register

You can [`read`](crate::Reg::read) this register and get [`ofcfgr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofcfgr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFCFGR4)

For information about available fields see [`mod@ofcfgr4`] module*/
pub type OFCFGR4 = crate::Reg<ofcfgr4::OFCFGR4rs>;
///ADC offset 4 configuration register
pub mod ofcfgr4;
/**OFR1 (rw) register accessor: ADC offset 1 register

You can [`read`](crate::Reg::read) this register and get [`ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFR1)

For information about available fields see [`mod@ofr1`] module*/
pub type OFR1 = crate::Reg<ofr1::OFR1rs>;
///ADC offset 1 register
pub mod ofr1;
/**OFR2 (rw) register accessor: ADC offset 2 register

You can [`read`](crate::Reg::read) this register and get [`ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFR2)

For information about available fields see [`mod@ofr2`] module*/
pub type OFR2 = crate::Reg<ofr2::OFR2rs>;
///ADC offset 2 register
pub mod ofr2;
/**OFR3 (rw) register accessor: ADC offset 3 register

You can [`read`](crate::Reg::read) this register and get [`ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFR3)

For information about available fields see [`mod@ofr3`] module*/
pub type OFR3 = crate::Reg<ofr3::OFR3rs>;
///ADC offset 3 register
pub mod ofr3;
/**OFR4 (rw) register accessor: ADC offset 4 register

You can [`read`](crate::Reg::read) this register and get [`ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OFR4)

For information about available fields see [`mod@ofr4`] module*/
pub type OFR4 = crate::Reg<ofr4::OFR4rs>;
///ADC offset 4 register
pub mod ofr4;
/**GCOMP (rw) register accessor: ADC gain compensation register

You can [`read`](crate::Reg::read) this register and get [`gcomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:GCOMP)

For information about available fields see [`mod@gcomp`] module*/
pub type GCOMP = crate::Reg<gcomp::GCOMPrs>;
///ADC gain compensation register
pub mod gcomp;
/**JDR1 (r) register accessor: ADC injected channel 1 data register

You can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JDR1)

For information about available fields see [`mod@jdr1`] module*/
pub type JDR1 = crate::Reg<jdr1::JDR1rs>;
///ADC injected channel 1 data register
pub mod jdr1;
/**JDR2 (r) register accessor: ADC injected channel 2 data register

You can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JDR2)

For information about available fields see [`mod@jdr2`] module*/
pub type JDR2 = crate::Reg<jdr2::JDR2rs>;
///ADC injected channel 2 data register
pub mod jdr2;
/**JDR3 (r) register accessor: ADC injected channel 3 data register

You can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JDR3)

For information about available fields see [`mod@jdr3`] module*/
pub type JDR3 = crate::Reg<jdr3::JDR3rs>;
///ADC injected channel 3 data register
pub mod jdr3;
/**JDR4 (r) register accessor: ADC injected channel 4 data register

You can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:JDR4)

For information about available fields see [`mod@jdr4`] module*/
pub type JDR4 = crate::Reg<jdr4::JDR4rs>;
///ADC injected channel 4 data register
pub mod jdr4;
/**AWD2CR (rw) register accessor: ADC Analog Watchdog 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD2CR)

For information about available fields see [`mod@awd2cr`] module*/
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
///ADC Analog Watchdog 2 Configuration Register
pub mod awd2cr;
/**AWD3CR (rw) register accessor: ADC Analog Watchdog 3 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD3CR)

For information about available fields see [`mod@awd3cr`] module*/
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
///ADC Analog Watchdog 3 Configuration Register
pub mod awd3cr;
/**AWD1LTR (rw) register accessor: ADC analog watchdog 1 lower threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1ltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1ltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD1LTR)

For information about available fields see [`mod@awd1ltr`] module*/
pub type AWD1LTR = crate::Reg<awd1ltr::AWD1LTRrs>;
///ADC analog watchdog 1 lower threshold register
pub mod awd1ltr;
/**AWD1HTR (rw) register accessor: ADC analog watchdog 1 higher threshold register

You can [`read`](crate::Reg::read) this register and get [`awd1htr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd1htr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD1HTR)

For information about available fields see [`mod@awd1htr`] module*/
pub type AWD1HTR = crate::Reg<awd1htr::AWD1HTRrs>;
///ADC analog watchdog 1 higher threshold register
pub mod awd1htr;
/**AWD2LTR (rw) register accessor: ADC analog watchdog 2 lower threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2ltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2ltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD2LTR)

For information about available fields see [`mod@awd2ltr`] module*/
pub type AWD2LTR = crate::Reg<awd2ltr::AWD2LTRrs>;
///ADC analog watchdog 2 lower threshold register
pub mod awd2ltr;
/**AWD2HTR (rw) register accessor: ADC analog watchdog 2 higher threshold register

You can [`read`](crate::Reg::read) this register and get [`awd2htr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2htr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD2HTR)

For information about available fields see [`mod@awd2htr`] module*/
pub type AWD2HTR = crate::Reg<awd2htr::AWD2HTRrs>;
///ADC analog watchdog 2 higher threshold register
pub mod awd2htr;
/**AWD3LTR (rw) register accessor: ADC analog watchdog 3 lower threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3ltr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3ltr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD3LTR)

For information about available fields see [`mod@awd3ltr`] module*/
pub type AWD3LTR = crate::Reg<awd3ltr::AWD3LTRrs>;
///ADC analog watchdog 3 lower threshold register
pub mod awd3ltr;
/**AWD3HTR (rw) register accessor: ADC analog watchdog 3 higher threshold register

You can [`read`](crate::Reg::read) this register and get [`awd3htr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3htr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:AWD3HTR)

For information about available fields see [`mod@awd3htr`] module*/
pub type AWD3HTR = crate::Reg<awd3htr::AWD3HTRrs>;
///ADC analog watchdog 3 higher threshold register
pub mod awd3htr;
/**DIFSEL (rw) register accessor: ADC Differential mode selection register

You can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:DIFSEL)

For information about available fields see [`mod@difsel`] module*/
pub type DIFSEL = crate::Reg<difsel::DIFSELrs>;
///ADC Differential mode selection register
pub mod difsel;
/**CALFACT (rw) register accessor: ADC calibration factors

You can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:CALFACT)

For information about available fields see [`mod@calfact`] module*/
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
///ADC calibration factors
pub mod calfact;
/**OR (rw) register accessor: ADC option register

You can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#ADC1:OR)

For information about available fields see [`mod@or`] module*/
pub type OR = crate::Reg<or::ORrs>;
///ADC option register
pub mod or;
