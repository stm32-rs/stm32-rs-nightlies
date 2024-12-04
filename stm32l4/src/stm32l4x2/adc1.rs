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
    _reserved7: [u8; 0x04],
    tr1: TR1,
    tr2: TR2,
    tr3: TR3,
    _reserved10: [u8; 0x04],
    sqr1: SQR1,
    sqr2: SQR2,
    sqr3: SQR3,
    sqr4: SQR4,
    dr: DR,
    _reserved15: [u8; 0x08],
    jsqr: JSQR,
    _reserved16: [u8; 0x10],
    ofr1: OFR1,
    ofr2: OFR2,
    ofr3: OFR3,
    ofr4: OFR4,
    _reserved20: [u8; 0x10],
    jdr1: JDR1,
    jdr2: JDR2,
    jdr3: JDR3,
    jdr4: JDR4,
    _reserved24: [u8; 0x10],
    awd2cr: AWD2CR,
    awd3cr: AWD3CR,
    _reserved26: [u8; 0x08],
    difsel: DIFSEL,
    calfact: CALFACT,
}
impl RegisterBlock {
    ///0x00 - interrupt and status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x04 - interrupt enable register
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    ///0x08 - control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x0c - configuration register
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    ///0x10 - configuration register
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    ///0x14 - sample time register 1
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    ///0x18 - sample time register 2
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    ///0x20 - watchdog threshold register 1
    #[inline(always)]
    pub const fn tr1(&self) -> &TR1 {
        &self.tr1
    }
    ///0x24 - watchdog threshold register
    #[inline(always)]
    pub const fn tr2(&self) -> &TR2 {
        &self.tr2
    }
    ///0x28 - watchdog threshold register 3
    #[inline(always)]
    pub const fn tr3(&self) -> &TR3 {
        &self.tr3
    }
    ///0x30 - regular sequence register 1
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    ///0x34 - regular sequence register 2
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    ///0x38 - regular sequence register 3
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    ///0x3c - regular sequence register 4
    #[inline(always)]
    pub const fn sqr4(&self) -> &SQR4 {
        &self.sqr4
    }
    ///0x40 - regular Data Register
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    ///0x4c - injected sequence register
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    ///0x60 - offset register 1
    #[inline(always)]
    pub const fn ofr1(&self) -> &OFR1 {
        &self.ofr1
    }
    ///0x64 - offset register 2
    #[inline(always)]
    pub const fn ofr2(&self) -> &OFR2 {
        &self.ofr2
    }
    ///0x68 - offset register 3
    #[inline(always)]
    pub const fn ofr3(&self) -> &OFR3 {
        &self.ofr3
    }
    ///0x6c - offset register 4
    #[inline(always)]
    pub const fn ofr4(&self) -> &OFR4 {
        &self.ofr4
    }
    ///0x80 - injected data register 1
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR1 {
        &self.jdr1
    }
    ///0x84 - injected data register 2
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR2 {
        &self.jdr2
    }
    ///0x88 - injected data register 3
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR3 {
        &self.jdr3
    }
    ///0x8c - injected data register 4
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR4 {
        &self.jdr4
    }
    ///0xa0 - Analog Watchdog 2 Configuration Register
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    ///0xa4 - Analog Watchdog 3 Configuration Register
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    ///0xb0 - Differential Mode Selection Register 2
    #[inline(always)]
    pub const fn difsel(&self) -> &DIFSEL {
        &self.difsel
    }
    ///0xb4 - Calibration Factors
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
}
/**ISR (rw) register accessor: interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:ISR)

For information about available fields see [`mod@isr`]
module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///interrupt and status register
pub mod isr;
/**IER (rw) register accessor: interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:IER)

For information about available fields see [`mod@ier`]
module*/
pub type IER = crate::Reg<ier::IERrs>;
///interrupt enable register
pub mod ier;
/**CR (rw) register accessor: control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:CR)

For information about available fields see [`mod@cr`]
module*/
pub type CR = crate::Reg<cr::CRrs>;
///control register
pub mod cr;
/**CFGR (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:CFGR)

For information about available fields see [`mod@cfgr`]
module*/
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
///configuration register
pub mod cfgr;
/**CFGR2 (rw) register accessor: configuration register

You can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:CFGR2)

For information about available fields see [`mod@cfgr2`]
module*/
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
///configuration register
pub mod cfgr2;
/**SMPR1 (rw) register accessor: sample time register 1

You can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SMPR1)

For information about available fields see [`mod@smpr1`]
module*/
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
///sample time register 1
pub mod smpr1;
/**SMPR2 (rw) register accessor: sample time register 2

You can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SMPR2)

For information about available fields see [`mod@smpr2`]
module*/
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
///sample time register 2
pub mod smpr2;
/**TR1 (rw) register accessor: watchdog threshold register 1

You can [`read`](crate::Reg::read) this register and get [`tr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:TR1)

For information about available fields see [`mod@tr1`]
module*/
pub type TR1 = crate::Reg<tr1::TR1rs>;
///watchdog threshold register 1
pub mod tr1;
/**TR2 (rw) register accessor: watchdog threshold register

You can [`read`](crate::Reg::read) this register and get [`tr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:TR2)

For information about available fields see [`mod@tr2`]
module*/
pub type TR2 = crate::Reg<tr2::TR2rs>;
///watchdog threshold register
pub mod tr2;
/**TR3 (rw) register accessor: watchdog threshold register 3

You can [`read`](crate::Reg::read) this register and get [`tr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:TR3)

For information about available fields see [`mod@tr3`]
module*/
pub type TR3 = crate::Reg<tr3::TR3rs>;
///watchdog threshold register 3
pub mod tr3;
/**SQR1 (rw) register accessor: regular sequence register 1

You can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SQR1)

For information about available fields see [`mod@sqr1`]
module*/
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
///regular sequence register 1
pub mod sqr1;
/**SQR2 (rw) register accessor: regular sequence register 2

You can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SQR2)

For information about available fields see [`mod@sqr2`]
module*/
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
///regular sequence register 2
pub mod sqr2;
/**SQR3 (rw) register accessor: regular sequence register 3

You can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SQR3)

For information about available fields see [`mod@sqr3`]
module*/
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
///regular sequence register 3
pub mod sqr3;
/**SQR4 (rw) register accessor: regular sequence register 4

You can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:SQR4)

For information about available fields see [`mod@sqr4`]
module*/
pub type SQR4 = crate::Reg<sqr4::SQR4rs>;
///regular sequence register 4
pub mod sqr4;
/**DR (r) register accessor: regular Data Register

You can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:DR)

For information about available fields see [`mod@dr`]
module*/
pub type DR = crate::Reg<dr::DRrs>;
///regular Data Register
pub mod dr;
/**JSQR (rw) register accessor: injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:JSQR)

For information about available fields see [`mod@jsqr`]
module*/
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
///injected sequence register
pub mod jsqr;
/**OFR1 (rw) register accessor: offset register 1

You can [`read`](crate::Reg::read) this register and get [`ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:OFR1)

For information about available fields see [`mod@ofr1`]
module*/
pub type OFR1 = crate::Reg<ofr1::OFR1rs>;
///offset register 1
pub mod ofr1;
/**OFR2 (rw) register accessor: offset register 2

You can [`read`](crate::Reg::read) this register and get [`ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:OFR2)

For information about available fields see [`mod@ofr2`]
module*/
pub type OFR2 = crate::Reg<ofr2::OFR2rs>;
///offset register 2
pub mod ofr2;
/**OFR3 (rw) register accessor: offset register 3

You can [`read`](crate::Reg::read) this register and get [`ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:OFR3)

For information about available fields see [`mod@ofr3`]
module*/
pub type OFR3 = crate::Reg<ofr3::OFR3rs>;
///offset register 3
pub mod ofr3;
/**OFR4 (rw) register accessor: offset register 4

You can [`read`](crate::Reg::read) this register and get [`ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:OFR4)

For information about available fields see [`mod@ofr4`]
module*/
pub type OFR4 = crate::Reg<ofr4::OFR4rs>;
///offset register 4
pub mod ofr4;
/**JDR1 (r) register accessor: injected data register 1

You can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:JDR1)

For information about available fields see [`mod@jdr1`]
module*/
pub type JDR1 = crate::Reg<jdr1::JDR1rs>;
///injected data register 1
pub mod jdr1;
/**JDR2 (r) register accessor: injected data register 2

You can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:JDR2)

For information about available fields see [`mod@jdr2`]
module*/
pub type JDR2 = crate::Reg<jdr2::JDR2rs>;
///injected data register 2
pub mod jdr2;
/**JDR3 (r) register accessor: injected data register 3

You can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:JDR3)

For information about available fields see [`mod@jdr3`]
module*/
pub type JDR3 = crate::Reg<jdr3::JDR3rs>;
///injected data register 3
pub mod jdr3;
/**JDR4 (r) register accessor: injected data register 4

You can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:JDR4)

For information about available fields see [`mod@jdr4`]
module*/
pub type JDR4 = crate::Reg<jdr4::JDR4rs>;
///injected data register 4
pub mod jdr4;
/**AWD2CR (rw) register accessor: Analog Watchdog 2 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:AWD2CR)

For information about available fields see [`mod@awd2cr`]
module*/
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
///Analog Watchdog 2 Configuration Register
pub mod awd2cr;
/**AWD3CR (rw) register accessor: Analog Watchdog 3 Configuration Register

You can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:AWD3CR)

For information about available fields see [`mod@awd3cr`]
module*/
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
///Analog Watchdog 3 Configuration Register
pub mod awd3cr;
/**DIFSEL (rw) register accessor: Differential Mode Selection Register 2

You can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:DIFSEL)

For information about available fields see [`mod@difsel`]
module*/
pub type DIFSEL = crate::Reg<difsel::DIFSELrs>;
///Differential Mode Selection Register 2
pub mod difsel;
/**CALFACT (rw) register accessor: Calibration Factors

You can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x2.html#ADC1:CALFACT)

For information about available fields see [`mod@calfact`]
module*/
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
///Calibration Factors
pub mod calfact;
