#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - interrupt and status register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x04 - interrupt enable register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x08 - control register"]
    pub cr: crate::Reg<cr::CR_SPEC>,
    #[doc = "0x0c - configuration register"]
    pub cfgr: crate::Reg<cfgr::CFGR_SPEC>,
    #[doc = "0x10 - configuration register"]
    pub cfgr2: crate::Reg<cfgr2::CFGR2_SPEC>,
    #[doc = "0x14 - sample time register 1"]
    pub smpr1: crate::Reg<smpr1::SMPR1_SPEC>,
    #[doc = "0x18 - sample time register 2"]
    pub smpr2: crate::Reg<smpr2::SMPR2_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x20 - watchdog threshold register 1"]
    pub tr1: crate::Reg<tr1::TR1_SPEC>,
    #[doc = "0x24 - watchdog threshold register"]
    pub tr2: crate::Reg<tr2::TR2_SPEC>,
    #[doc = "0x28 - watchdog threshold register 3"]
    pub tr3: crate::Reg<tr3::TR3_SPEC>,
    _reserved10: [u8; 0x04],
    #[doc = "0x30 - regular sequence register 1"]
    pub sqr1: crate::Reg<sqr1::SQR1_SPEC>,
    #[doc = "0x34 - regular sequence register 2"]
    pub sqr2: crate::Reg<sqr2::SQR2_SPEC>,
    #[doc = "0x38 - regular sequence register 3"]
    pub sqr3: crate::Reg<sqr3::SQR3_SPEC>,
    #[doc = "0x3c - regular sequence register 4"]
    pub sqr4: crate::Reg<sqr4::SQR4_SPEC>,
    #[doc = "0x40 - regular Data Register"]
    pub dr: crate::Reg<dr::DR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x4c - injected sequence register"]
    pub jsqr: crate::Reg<jsqr::JSQR_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - offset register 1"]
    pub ofr1: crate::Reg<ofr1::OFR1_SPEC>,
    #[doc = "0x64 - offset register 2"]
    pub ofr2: crate::Reg<ofr2::OFR2_SPEC>,
    #[doc = "0x68 - offset register 3"]
    pub ofr3: crate::Reg<ofr3::OFR3_SPEC>,
    #[doc = "0x6c - offset register 4"]
    pub ofr4: crate::Reg<ofr4::OFR4_SPEC>,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - injected data register 1"]
    pub jdr1: crate::Reg<jdr1::JDR1_SPEC>,
    #[doc = "0x84 - injected data register 2"]
    pub jdr2: crate::Reg<jdr2::JDR2_SPEC>,
    #[doc = "0x88 - injected data register 3"]
    pub jdr3: crate::Reg<jdr3::JDR3_SPEC>,
    #[doc = "0x8c - injected data register 4"]
    pub jdr4: crate::Reg<jdr4::JDR4_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0xa0 - Analog Watchdog 2 Configuration Register"]
    pub awd2cr: crate::Reg<awd2cr::AWD2CR_SPEC>,
    #[doc = "0xa4 - Analog Watchdog 3 Configuration Register"]
    pub awd3cr: crate::Reg<awd3cr::AWD3CR_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0xb0 - Differential Mode Selection Register 2"]
    pub difsel: crate::Reg<difsel::DIFSEL_SPEC>,
    #[doc = "0xb4 - Calibration Factors"]
    pub calfact: crate::Reg<calfact::CALFACT_SPEC>,
}
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "interrupt and status register"]
pub mod isr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "interrupt enable register"]
pub mod ier;
#[doc = "CR register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "control register"]
pub mod cr;
#[doc = "CFGR register accessor: an alias for `Reg<CFGR_SPEC>`"]
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CFGR2 register accessor: an alias for `Reg<CFGR2_SPEC>`"]
pub type CFGR2 = crate::Reg<cfgr2::CFGR2_SPEC>;
#[doc = "configuration register"]
pub mod cfgr2;
#[doc = "SMPR1 register accessor: an alias for `Reg<SMPR1_SPEC>`"]
pub type SMPR1 = crate::Reg<smpr1::SMPR1_SPEC>;
#[doc = "sample time register 1"]
pub mod smpr1;
#[doc = "SMPR2 register accessor: an alias for `Reg<SMPR2_SPEC>`"]
pub type SMPR2 = crate::Reg<smpr2::SMPR2_SPEC>;
#[doc = "sample time register 2"]
pub mod smpr2;
#[doc = "TR1 register accessor: an alias for `Reg<TR1_SPEC>`"]
pub type TR1 = crate::Reg<tr1::TR1_SPEC>;
#[doc = "watchdog threshold register 1"]
pub mod tr1;
#[doc = "TR2 register accessor: an alias for `Reg<TR2_SPEC>`"]
pub type TR2 = crate::Reg<tr2::TR2_SPEC>;
#[doc = "watchdog threshold register"]
pub mod tr2;
#[doc = "TR3 register accessor: an alias for `Reg<TR3_SPEC>`"]
pub type TR3 = crate::Reg<tr3::TR3_SPEC>;
#[doc = "watchdog threshold register 3"]
pub mod tr3;
#[doc = "SQR1 register accessor: an alias for `Reg<SQR1_SPEC>`"]
pub type SQR1 = crate::Reg<sqr1::SQR1_SPEC>;
#[doc = "regular sequence register 1"]
pub mod sqr1;
#[doc = "SQR2 register accessor: an alias for `Reg<SQR2_SPEC>`"]
pub type SQR2 = crate::Reg<sqr2::SQR2_SPEC>;
#[doc = "regular sequence register 2"]
pub mod sqr2;
#[doc = "SQR3 register accessor: an alias for `Reg<SQR3_SPEC>`"]
pub type SQR3 = crate::Reg<sqr3::SQR3_SPEC>;
#[doc = "regular sequence register 3"]
pub mod sqr3;
#[doc = "SQR4 register accessor: an alias for `Reg<SQR4_SPEC>`"]
pub type SQR4 = crate::Reg<sqr4::SQR4_SPEC>;
#[doc = "regular sequence register 4"]
pub mod sqr4;
#[doc = "DR register accessor: an alias for `Reg<DR_SPEC>`"]
pub type DR = crate::Reg<dr::DR_SPEC>;
#[doc = "regular Data Register"]
pub mod dr;
#[doc = "JSQR register accessor: an alias for `Reg<JSQR_SPEC>`"]
pub type JSQR = crate::Reg<jsqr::JSQR_SPEC>;
#[doc = "injected sequence register"]
pub mod jsqr;
#[doc = "OFR1 register accessor: an alias for `Reg<OFR1_SPEC>`"]
pub type OFR1 = crate::Reg<ofr1::OFR1_SPEC>;
#[doc = "offset register 1"]
pub mod ofr1;
#[doc = "OFR2 register accessor: an alias for `Reg<OFR2_SPEC>`"]
pub type OFR2 = crate::Reg<ofr2::OFR2_SPEC>;
#[doc = "offset register 2"]
pub mod ofr2;
#[doc = "OFR3 register accessor: an alias for `Reg<OFR3_SPEC>`"]
pub type OFR3 = crate::Reg<ofr3::OFR3_SPEC>;
#[doc = "offset register 3"]
pub mod ofr3;
#[doc = "OFR4 register accessor: an alias for `Reg<OFR4_SPEC>`"]
pub type OFR4 = crate::Reg<ofr4::OFR4_SPEC>;
#[doc = "offset register 4"]
pub mod ofr4;
#[doc = "JDR1 register accessor: an alias for `Reg<JDR1_SPEC>`"]
pub type JDR1 = crate::Reg<jdr1::JDR1_SPEC>;
#[doc = "injected data register 1"]
pub mod jdr1;
#[doc = "JDR2 register accessor: an alias for `Reg<JDR2_SPEC>`"]
pub type JDR2 = crate::Reg<jdr2::JDR2_SPEC>;
#[doc = "injected data register 2"]
pub mod jdr2;
#[doc = "JDR3 register accessor: an alias for `Reg<JDR3_SPEC>`"]
pub type JDR3 = crate::Reg<jdr3::JDR3_SPEC>;
#[doc = "injected data register 3"]
pub mod jdr3;
#[doc = "JDR4 register accessor: an alias for `Reg<JDR4_SPEC>`"]
pub type JDR4 = crate::Reg<jdr4::JDR4_SPEC>;
#[doc = "injected data register 4"]
pub mod jdr4;
#[doc = "AWD2CR register accessor: an alias for `Reg<AWD2CR_SPEC>`"]
pub type AWD2CR = crate::Reg<awd2cr::AWD2CR_SPEC>;
#[doc = "Analog Watchdog 2 Configuration Register"]
pub mod awd2cr;
#[doc = "AWD3CR register accessor: an alias for `Reg<AWD3CR_SPEC>`"]
pub type AWD3CR = crate::Reg<awd3cr::AWD3CR_SPEC>;
#[doc = "Analog Watchdog 3 Configuration Register"]
pub mod awd3cr;
#[doc = "DIFSEL register accessor: an alias for `Reg<DIFSEL_SPEC>`"]
pub type DIFSEL = crate::Reg<difsel::DIFSEL_SPEC>;
#[doc = "Differential Mode Selection Register 2"]
pub mod difsel;
#[doc = "CALFACT register accessor: an alias for `Reg<CALFACT_SPEC>`"]
pub type CALFACT = crate::Reg<calfact::CALFACT_SPEC>;
#[doc = "Calibration Factors"]
pub mod calfact;
