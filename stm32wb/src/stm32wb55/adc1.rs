#[repr(C)]
#[doc = "Register block"]
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
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &IER {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &CFGR2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &SMPR1 {
        &self.smpr1
    }
    #[doc = "0x18 - ADC sampling time register 2"]
    #[inline(always)]
    pub const fn smpr2(&self) -> &SMPR2 {
        &self.smpr2
    }
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn tr1(&self) -> &TR1 {
        &self.tr1
    }
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    #[inline(always)]
    pub const fn tr2(&self) -> &TR2 {
        &self.tr2
    }
    #[doc = "0x28 - ADC analog watchdog 3 threshold register"]
    #[inline(always)]
    pub const fn tr3(&self) -> &TR3 {
        &self.tr3
    }
    #[doc = "0x30 - ADC group regular sequencer ranks register 1"]
    #[inline(always)]
    pub const fn sqr1(&self) -> &SQR1 {
        &self.sqr1
    }
    #[doc = "0x34 - ADC group regular sequencer ranks register 2"]
    #[inline(always)]
    pub const fn sqr2(&self) -> &SQR2 {
        &self.sqr2
    }
    #[doc = "0x38 - ADC group regular sequencer ranks register 3"]
    #[inline(always)]
    pub const fn sqr3(&self) -> &SQR3 {
        &self.sqr3
    }
    #[doc = "0x3c - ADC group regular sequencer ranks register 4"]
    #[inline(always)]
    pub const fn sqr4(&self) -> &SQR4 {
        &self.sqr4
    }
    #[doc = "0x40 - ADC group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x4c - ADC group injected sequencer register"]
    #[inline(always)]
    pub const fn jsqr(&self) -> &JSQR {
        &self.jsqr
    }
    #[doc = "0x60 - ADC offset number 1 register"]
    #[inline(always)]
    pub const fn ofr1(&self) -> &OFR1 {
        &self.ofr1
    }
    #[doc = "0x64 - ADC offset number 2 register"]
    #[inline(always)]
    pub const fn ofr2(&self) -> &OFR2 {
        &self.ofr2
    }
    #[doc = "0x68 - ADC offset number 3 register"]
    #[inline(always)]
    pub const fn ofr3(&self) -> &OFR3 {
        &self.ofr3
    }
    #[doc = "0x6c - ADC offset number 4 register"]
    #[inline(always)]
    pub const fn ofr4(&self) -> &OFR4 {
        &self.ofr4
    }
    #[doc = "0x80 - ADC group injected sequencer rank 1 register"]
    #[inline(always)]
    pub const fn jdr1(&self) -> &JDR1 {
        &self.jdr1
    }
    #[doc = "0x84 - ADC group injected sequencer rank 2 register"]
    #[inline(always)]
    pub const fn jdr2(&self) -> &JDR2 {
        &self.jdr2
    }
    #[doc = "0x88 - ADC group injected sequencer rank 3 register"]
    #[inline(always)]
    pub const fn jdr3(&self) -> &JDR3 {
        &self.jdr3
    }
    #[doc = "0x8c - ADC group injected sequencer rank 4 register"]
    #[inline(always)]
    pub const fn jdr4(&self) -> &JDR4 {
        &self.jdr4
    }
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &AWD2CR {
        &self.awd2cr
    }
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &AWD3CR {
        &self.awd3cr
    }
    #[doc = "0xb0 - ADC channel differential or single-ended mode selection register"]
    #[inline(always)]
    pub const fn difsel(&self) -> &DIFSEL {
        &self.difsel
    }
    #[doc = "0xb4 - ADC calibration factors register"]
    #[inline(always)]
    pub const fn calfact(&self) -> &CALFACT {
        &self.calfact
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
pub type ISR = crate::Reg<isr::ISRrs>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
pub type IER = crate::Reg<ier::IERrs>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`]
module"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2rs>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: ADC sampling time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`]
module"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1rs>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: ADC sampling time register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`]
module"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2rs>;
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "TR1 (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr1`]
module"]
pub type TR1 = crate::Reg<tr1::TR1rs>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod tr1;
#[doc = "TR2 (rw) register accessor: ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr2`]
module"]
pub type TR2 = crate::Reg<tr2::TR2rs>;
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod tr2;
#[doc = "TR3 (rw) register accessor: ADC analog watchdog 3 threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr3`]
module"]
pub type TR3 = crate::Reg<tr3::TR3rs>;
#[doc = "ADC analog watchdog 3 threshold register"]
pub mod tr3;
#[doc = "SQR1 (rw) register accessor: ADC group regular sequencer ranks register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`]
module"]
pub type SQR1 = crate::Reg<sqr1::SQR1rs>;
#[doc = "ADC group regular sequencer ranks register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: ADC group regular sequencer ranks register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`]
module"]
pub type SQR2 = crate::Reg<sqr2::SQR2rs>;
#[doc = "ADC group regular sequencer ranks register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: ADC group regular sequencer ranks register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`]
module"]
pub type SQR3 = crate::Reg<sqr3::SQR3rs>;
#[doc = "ADC group regular sequencer ranks register 3"]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: ADC group regular sequencer ranks register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sqr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr4`]
module"]
pub type SQR4 = crate::Reg<sqr4::SQR4rs>;
#[doc = "ADC group regular sequencer ranks register 4"]
pub mod sqr4;
#[doc = "DR (rw) register accessor: ADC group regular conversion data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: ADC group injected sequencer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`]
module"]
pub type JSQR = crate::Reg<jsqr::JSQRrs>;
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: ADC offset number 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr1`]
module"]
pub type OFR1 = crate::Reg<ofr1::OFR1rs>;
#[doc = "ADC offset number 1 register"]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: ADC offset number 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr2`]
module"]
pub type OFR2 = crate::Reg<ofr2::OFR2rs>;
#[doc = "ADC offset number 2 register"]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: ADC offset number 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr3`]
module"]
pub type OFR3 = crate::Reg<ofr3::OFR3rs>;
#[doc = "ADC offset number 3 register"]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: ADC offset number 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ofr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr4`]
module"]
pub type OFR4 = crate::Reg<ofr4::OFR4rs>;
#[doc = "ADC offset number 4 register"]
pub mod ofr4;
#[doc = "JDR1 (r) register accessor: ADC group injected sequencer rank 1 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`]
module"]
pub type JDR1 = crate::Reg<jdr1::JDR1rs>;
#[doc = "ADC group injected sequencer rank 1 register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: ADC group injected sequencer rank 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`]
module"]
pub type JDR2 = crate::Reg<jdr2::JDR2rs>;
#[doc = "ADC group injected sequencer rank 2 register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: ADC group injected sequencer rank 3 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr3::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`]
module"]
pub type JDR3 = crate::Reg<jdr3::JDR3rs>;
#[doc = "ADC group injected sequencer rank 3 register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: ADC group injected sequencer rank 4 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jdr4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`]
module"]
pub type JDR4 = crate::Reg<jdr4::JDR4rs>;
#[doc = "ADC group injected sequencer rank 4 register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd2cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`]
module"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CRrs>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`awd3cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`]
module"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CRrs>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: ADC channel differential or single-ended mode selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@difsel`]
module"]
pub type DIFSEL = crate::Reg<difsel::DIFSELrs>;
#[doc = "ADC channel differential or single-ended mode selection register"]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: ADC calibration factors register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calfact::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`]
module"]
pub type CALFACT = crate::Reg<calfact::CALFACTrs>;
#[doc = "ADC calibration factors register"]
pub mod calfact;
