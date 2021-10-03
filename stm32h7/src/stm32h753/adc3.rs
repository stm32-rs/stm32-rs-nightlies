#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - ADC control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - ADC configuration register 1"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x10 - ADC configuration register 2"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x14 - ADC sampling time register 1"]
    pub smpr1: crate::Reg<smpr1::SMPR1_SPEC>,
    #[doc = "0x18 - ADC sampling time register 2"]
    pub smpr2: crate::Reg<smpr2::SMPR2_SPEC>,
    #[doc = "0x1c - ADC pre channel selection register"]
    pub pcsel: crate::Reg<pcsel::PCSEL_SPEC>,
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    pub ltr1: crate::Reg<ltr1::LTR1_SPEC>,
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    pub htr1: crate::Reg<htr1::HTR1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - ADC group regular sequencer ranks register 1"]
    pub sqr1: crate::Reg<sqr1::SQR1_SPEC>,
    #[doc = "0x34 - ADC group regular sequencer ranks register 2"]
    pub sqr2: crate::Reg<sqr2::SQR2_SPEC>,
    #[doc = "0x38 - ADC group regular sequencer ranks register 3"]
    pub sqr3: crate::Reg<sqr3::SQR3_SPEC>,
    #[doc = "0x3c - ADC group regular sequencer ranks register 4"]
    pub sqr4: crate::Reg<sqr4::SQR4_SPEC>,
    #[doc = "0x40 - ADC group regular conversion data register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x4c - ADC group injected sequencer register"]
    pub jsqr: crate::Reg<jsqr::JSQR_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - ADC offset number 1 register"]
    pub ofr1: crate::Reg<ofr1::OFR1_SPEC>,
    #[doc = "0x64 - ADC offset number 2 register"]
    pub ofr2: crate::Reg<ofr2::OFR2_SPEC>,
    #[doc = "0x68 - ADC offset number 3 register"]
    pub ofr3: crate::Reg<ofr3::OFR3_SPEC>,
    #[doc = "0x6c - ADC offset number 4 register"]
    pub ofr4: crate::Reg<ofr4::OFR4_SPEC>,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - ADC group injected sequencer rank 1 register"]
    pub jdr1: crate::Reg<jdr1::JDR1_SPEC>,
    #[doc = "0x84 - ADC group injected sequencer rank 2 register"]
    pub jdr2: crate::Reg<jdr2::JDR2_SPEC>,
    #[doc = "0x88 - ADC group injected sequencer rank 3 register"]
    pub jdr3: crate::Reg<jdr3::JDR3_SPEC>,
    #[doc = "0x8c - ADC group injected sequencer rank 4 register"]
    pub jdr4: crate::Reg<jdr4::JDR4_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    pub awd2cr: crate::Reg<awd2cr::AWD2CR_SPEC>,
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    pub awd3cr: crate::Reg<awd3cr::AWD3CR_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    pub ltr2: crate::Reg<ltr2::LTR2_SPEC>,
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    pub htr2: crate::Reg<htr2::HTR2_SPEC>,
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    pub ltr3: crate::Reg<ltr3::LTR3_SPEC>,
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    pub htr3: crate::Reg<htr3::HTR3_SPEC>,
    #[doc = "0xc0 - ADC channel differential or single-ended mode selection register"]
    pub difsel: crate::Reg<difsel::DIFSEL_SPEC>,
    #[doc = "0xc4 - ADC calibration factors register"]
    pub calfact: crate::Reg<calfact::CALFACT_SPEC>,
    #[doc = "0xc8 - ADC Calibration Factor register 2"]
    pub calfact2: crate::Reg<calfact2::CALFACT2_SPEC>,
}
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "SMPR2 register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "LTR1 register accessor: an alias for `Reg<LTR1_SPEC>`"]
pub type LTR1 = crate::Reg<ltr1::LTR1_SPEC>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod ltr1;
#[doc = "HTR1 register accessor: an alias for `Reg<HTR1_SPEC>`"]
pub type HTR1 = crate::Reg<htr1::HTR1_SPEC>;
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod htr1;
#[doc = "SQR1 register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "ADC group regular sequencer ranks register 1"]
pub mod sqr1;
#[doc = "SQR2 register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "ADC group regular sequencer ranks register 2"]
pub mod sqr2;
#[doc = "SQR3 register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "ADC group regular sequencer ranks register 3"]
pub mod sqr3;
#[doc = "SQR4 register accessor: an alias for `Reg<SQR4_SPEC>`"]
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
#[doc = "ADC group regular sequencer ranks register 4"]
pub mod sqr4;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "JSQR register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "OFR1 register accessor: an alias for `Reg<OFR1_SPEC>`"]
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
#[doc = "ADC offset number 1 register"]
pub mod ofr1;
#[doc = "OFR2 register accessor: an alias for `Reg<OFR2_SPEC>`"]
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
#[doc = "ADC offset number 2 register"]
pub mod ofr2;
#[doc = "OFR3 register accessor: an alias for `Reg<OFR3_SPEC>`"]
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
#[doc = "ADC offset number 3 register"]
pub mod ofr3;
#[doc = "OFR4 register accessor: an alias for `Reg<OFR4_SPEC>`"]
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
#[doc = "ADC offset number 4 register"]
pub mod ofr4;
#[doc = "JDR1 register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "ADC group injected sequencer rank 1 register"]
pub mod jdr1;
#[doc = "JDR2 register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "ADC group injected sequencer rank 2 register"]
pub mod jdr2;
#[doc = "JDR3 register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "ADC group injected sequencer rank 3 register"]
pub mod jdr3;
#[doc = "JDR4 register accessor: an alias for `Reg<JDR4_SPEC>`"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "ADC group injected sequencer rank 4 register"]
pub mod jdr4;
#[doc = "AWD2CR register accessor: an alias for `Reg<AWD2CR_SPEC>`"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR register accessor: an alias for `Reg<AWD3CR_SPEC>`"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "DIFSEL register accessor: an alias for `Reg<DIFSEL_SPEC>`"]
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
#[doc = "ADC channel differential or single-ended mode selection register"]
pub mod difsel;
#[doc = "CALFACT register accessor: an alias for `Reg<CALFACT_SPEC>`"]
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
#[doc = "ADC calibration factors register"]
pub mod calfact;
#[doc = "PCSEL register accessor: an alias for `Reg<PCSEL_SPEC>`"]
pub type PCSEL = crate::Reg<pcsel::PCSEL_SPEC>;
#[doc = "ADC pre channel selection register"]
pub mod pcsel;
#[doc = "LTR2 register accessor: an alias for `Reg<LTR2_SPEC>`"]
pub type LTR2 = crate::Reg<ltr2::LTR2_SPEC>;
#[doc = "ADC watchdog lower threshold register 2"]
pub mod ltr2;
#[doc = "HTR2 register accessor: an alias for `Reg<HTR2_SPEC>`"]
pub type HTR2 = crate::Reg<htr2::HTR2_SPEC>;
#[doc = "ADC watchdog higher threshold register 2"]
pub mod htr2;
#[doc = "LTR3 register accessor: an alias for `Reg<LTR3_SPEC>`"]
pub type LTR3 = crate::Reg<ltr3::LTR3_SPEC>;
#[doc = "ADC watchdog lower threshold register 3"]
pub mod ltr3;
#[doc = "HTR3 register accessor: an alias for `Reg<HTR3_SPEC>`"]
pub type HTR3 = crate::Reg<htr3::HTR3_SPEC>;
#[doc = "ADC watchdog higher threshold register 3"]
pub mod htr3;
#[doc = "CALFACT2 register accessor: an alias for `Reg<CALFACT2_SPEC>`"]
pub type CALFACT2 = crate::Reg<calfact2::CALFACT2_SPEC>;
#[doc = "ADC Calibration Factor register 2"]
pub mod calfact2;
