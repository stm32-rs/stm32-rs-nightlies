#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub adc_isr: crate::Reg<adc_isr::ADC_ISR_SPEC>,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub adc_ier: crate::Reg<adc_ier::ADC_IER_SPEC>,
    #[doc = "0x08 - ADC control register"]
    pub adc_cr: crate::Reg<adc_cr::ADC_CR_SPEC>,
    #[doc = "0x0c - ADC configuration register"]
    pub adc_cfgr: crate::Reg<adc_cfgr::ADC_CFGR_SPEC>,
    #[doc = "0x10 - ADC configuration register 2"]
    pub adc_cfgr2: crate::Reg<adc_cfgr2::ADC_CFGR2_SPEC>,
    #[doc = "0x14 - ADC sample time register 1"]
    pub adc_smpr1: crate::Reg<adc_smpr1::ADC_SMPR1_SPEC>,
    #[doc = "0x18 - ADC sample time register 2"]
    pub adc_smpr2: crate::Reg<adc_smpr2::ADC_SMPR2_SPEC>,
    #[doc = "0x1c - ADC channel preselection register"]
    pub adc_pcsel: crate::Reg<adc_pcsel::ADC_PCSEL_SPEC>,
    #[doc = "0x20 - ADC watchdog threshold register 1"]
    pub adc_ltr1: crate::Reg<adc_ltr1::ADC_LTR1_SPEC>,
    #[doc = "0x24 - ADC watchdog threshold register 1"]
    pub adc_htr1: crate::Reg<adc_htr1::ADC_HTR1_SPEC>,
    _reserved10: [u8; 0x08],
    #[doc = "0x30 - ADC regular sequence register 1"]
    pub adc_sqr1: crate::Reg<adc_sqr1::ADC_SQR1_SPEC>,
    #[doc = "0x34 - ADC regular sequence register 2"]
    pub adc_sqr2: crate::Reg<adc_sqr2::ADC_SQR2_SPEC>,
    #[doc = "0x38 - ADC regular sequence register 3"]
    pub adc_sqr3: crate::Reg<adc_sqr3::ADC_SQR3_SPEC>,
    #[doc = "0x3c - ADC regular sequence register 4"]
    pub adc_sqr4: crate::Reg<adc_sqr4::ADC_SQR4_SPEC>,
    #[doc = "0x40 - ADC regular Data Register"]
    pub adc_dr: crate::Reg<adc_dr::ADC_DR_SPEC>,
    _reserved15: [u8; 0x08],
    #[doc = "0x4c - ADC injected sequence register"]
    pub adc_jsqr: crate::Reg<adc_jsqr::ADC_JSQR_SPEC>,
    _reserved16: [u8; 0x10],
    #[doc = "0x60 - ADC offset register"]
    pub adc_ofr1: crate::Reg<adc_ofr1::ADC_OFR1_SPEC>,
    #[doc = "0x64 - ADC offset register"]
    pub adc_ofr2: crate::Reg<adc_ofr2::ADC_OFR2_SPEC>,
    #[doc = "0x68 - ADC offset register"]
    pub adc_ofr3: crate::Reg<adc_ofr3::ADC_OFR3_SPEC>,
    #[doc = "0x6c - ADC offset register"]
    pub adc_ofr4: crate::Reg<adc_ofr4::ADC_OFR4_SPEC>,
    _reserved20: [u8; 0x10],
    #[doc = "0x80 - ADC injected data register"]
    pub adc_jdr1: crate::Reg<adc_jdr1::ADC_JDR1_SPEC>,
    #[doc = "0x84 - ADC injected data register"]
    pub adc_jdr2: crate::Reg<adc_jdr2::ADC_JDR2_SPEC>,
    #[doc = "0x88 - ADC injected data register"]
    pub adc_jdr3: crate::Reg<adc_jdr3::ADC_JDR3_SPEC>,
    #[doc = "0x8c - ADC injected data register"]
    pub adc_jdr4: crate::Reg<adc_jdr4::ADC_JDR4_SPEC>,
    _reserved24: [u8; 0x10],
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    pub adc_awd2cr: crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>,
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    pub adc_awd3cr: crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>,
    _reserved26: [u8; 0x08],
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    pub adc_ltr2: crate::Reg<adc_ltr2::ADC_LTR2_SPEC>,
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    pub adc_htr2: crate::Reg<adc_htr2::ADC_HTR2_SPEC>,
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    pub adc_ltr3: crate::Reg<adc_ltr3::ADC_LTR3_SPEC>,
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    pub adc_htr3: crate::Reg<adc_htr3::ADC_HTR3_SPEC>,
    #[doc = "0xc0 - ADC differential mode selection register"]
    pub adc_difsel: crate::Reg<adc_difsel::ADC_DIFSEL_SPEC>,
    #[doc = "0xc4 - ADC calibration factors register"]
    pub adc_calfact: crate::Reg<adc_calfact::ADC_CALFACT_SPEC>,
    #[doc = "0xc8 - ADC calibration factor register 2"]
    pub adc_calfact2: crate::Reg<adc_calfact2::ADC_CALFACT2_SPEC>,
    _reserved33: [u8; 0x04],
    #[doc = "0xd0 - ADC2 option register"]
    pub adc2_or: crate::Reg<adc2_or::ADC2_OR_SPEC>,
}
#[doc = "ADC_ISR register accessor: an alias for `Reg<ADC_ISR_SPEC>`"]
pub type ADC_ISR = crate::Reg<adc_isr::ADC_ISR_SPEC>;
#[doc = "ADC interrupt and status register"]
pub mod adc_isr;
#[doc = "ADC_IER register accessor: an alias for `Reg<ADC_IER_SPEC>`"]
pub type ADC_IER = crate::Reg<adc_ier::ADC_IER_SPEC>;
#[doc = "ADC interrupt enable register"]
pub mod adc_ier;
#[doc = "ADC_CR register accessor: an alias for `Reg<ADC_CR_SPEC>`"]
pub type ADC_CR = crate::Reg<adc_cr::ADC_CR_SPEC>;
#[doc = "ADC control register"]
pub mod adc_cr;
#[doc = "ADC_CFGR register accessor: an alias for `Reg<ADC_CFGR_SPEC>`"]
pub type ADC_CFGR = crate::Reg<adc_cfgr::ADC_CFGR_SPEC>;
#[doc = "ADC configuration register"]
pub mod adc_cfgr;
#[doc = "ADC_CFGR2 register accessor: an alias for `Reg<ADC_CFGR2_SPEC>`"]
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC_SMPR1 register accessor: an alias for `Reg<ADC_SMPR1_SPEC>`"]
pub type ADC_SMPR1 = crate::Reg<adc_smpr1::ADC_SMPR1_SPEC>;
#[doc = "ADC sample time register 1"]
pub mod adc_smpr1;
#[doc = "ADC_SMPR2 register accessor: an alias for `Reg<ADC_SMPR2_SPEC>`"]
pub type ADC_SMPR2 = crate::Reg<adc_smpr2::ADC_SMPR2_SPEC>;
#[doc = "ADC sample time register 2"]
pub mod adc_smpr2;
#[doc = "ADC_PCSEL register accessor: an alias for `Reg<ADC_PCSEL_SPEC>`"]
pub type ADC_PCSEL = crate::Reg<adc_pcsel::ADC_PCSEL_SPEC>;
#[doc = "ADC channel preselection register"]
pub mod adc_pcsel;
#[doc = "ADC_LTR1 register accessor: an alias for `Reg<ADC_LTR1_SPEC>`"]
pub type ADC_LTR1 = crate::Reg<adc_ltr1::ADC_LTR1_SPEC>;
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_ltr1;
#[doc = "ADC_HTR1 register accessor: an alias for `Reg<ADC_HTR1_SPEC>`"]
pub type ADC_HTR1 = crate::Reg<adc_htr1::ADC_HTR1_SPEC>;
#[doc = "ADC watchdog threshold register 1"]
pub mod adc_htr1;
#[doc = "ADC_SQR1 register accessor: an alias for `Reg<ADC_SQR1_SPEC>`"]
pub type ADC_SQR1 = crate::Reg<adc_sqr1::ADC_SQR1_SPEC>;
#[doc = "ADC regular sequence register 1"]
pub mod adc_sqr1;
#[doc = "ADC_SQR2 register accessor: an alias for `Reg<ADC_SQR2_SPEC>`"]
pub type ADC_SQR2 = crate::Reg<adc_sqr2::ADC_SQR2_SPEC>;
#[doc = "ADC regular sequence register 2"]
pub mod adc_sqr2;
#[doc = "ADC_SQR3 register accessor: an alias for `Reg<ADC_SQR3_SPEC>`"]
pub type ADC_SQR3 = crate::Reg<adc_sqr3::ADC_SQR3_SPEC>;
#[doc = "ADC regular sequence register 3"]
pub mod adc_sqr3;
#[doc = "ADC_SQR4 register accessor: an alias for `Reg<ADC_SQR4_SPEC>`"]
pub type ADC_SQR4 = crate::Reg<adc_sqr4::ADC_SQR4_SPEC>;
#[doc = "ADC regular sequence register 4"]
pub mod adc_sqr4;
#[doc = "ADC_DR register accessor: an alias for `Reg<ADC_DR_SPEC>`"]
pub type ADC_DR = crate::Reg<adc_dr::ADC_DR_SPEC>;
#[doc = "ADC regular Data Register"]
pub mod adc_dr;
#[doc = "ADC_JSQR register accessor: an alias for `Reg<ADC_JSQR_SPEC>`"]
pub type ADC_JSQR = crate::Reg<adc_jsqr::ADC_JSQR_SPEC>;
#[doc = "ADC injected sequence register"]
pub mod adc_jsqr;
#[doc = "ADC_OFR1 register accessor: an alias for `Reg<ADC_OFR1_SPEC>`"]
pub type ADC_OFR1 = crate::Reg<adc_ofr1::ADC_OFR1_SPEC>;
#[doc = "ADC offset register"]
pub mod adc_ofr1;
#[doc = "ADC_OFR2 register accessor: an alias for `Reg<ADC_OFR2_SPEC>`"]
pub type ADC_OFR2 = crate::Reg<adc_ofr2::ADC_OFR2_SPEC>;
#[doc = "ADC offset register"]
pub mod adc_ofr2;
#[doc = "ADC_OFR3 register accessor: an alias for `Reg<ADC_OFR3_SPEC>`"]
pub type ADC_OFR3 = crate::Reg<adc_ofr3::ADC_OFR3_SPEC>;
#[doc = "ADC offset register"]
pub mod adc_ofr3;
#[doc = "ADC_OFR4 register accessor: an alias for `Reg<ADC_OFR4_SPEC>`"]
pub type ADC_OFR4 = crate::Reg<adc_ofr4::ADC_OFR4_SPEC>;
#[doc = "ADC offset register"]
pub mod adc_ofr4;
#[doc = "ADC_JDR1 register accessor: an alias for `Reg<ADC_JDR1_SPEC>`"]
pub type ADC_JDR1 = crate::Reg<adc_jdr1::ADC_JDR1_SPEC>;
#[doc = "ADC injected data register"]
pub mod adc_jdr1;
#[doc = "ADC_JDR2 register accessor: an alias for `Reg<ADC_JDR2_SPEC>`"]
pub type ADC_JDR2 = crate::Reg<adc_jdr2::ADC_JDR2_SPEC>;
#[doc = "ADC injected data register"]
pub mod adc_jdr2;
#[doc = "ADC_JDR3 register accessor: an alias for `Reg<ADC_JDR3_SPEC>`"]
pub type ADC_JDR3 = crate::Reg<adc_jdr3::ADC_JDR3_SPEC>;
#[doc = "ADC injected data register"]
pub mod adc_jdr3;
#[doc = "ADC_JDR4 register accessor: an alias for `Reg<ADC_JDR4_SPEC>`"]
pub type ADC_JDR4 = crate::Reg<adc_jdr4::ADC_JDR4_SPEC>;
#[doc = "ADC injected data register"]
pub mod adc_jdr4;
#[doc = "ADC_AWD2CR register accessor: an alias for `Reg<ADC_AWD2CR_SPEC>`"]
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC_AWD3CR register accessor: an alias for `Reg<ADC_AWD3CR_SPEC>`"]
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC_LTR2 register accessor: an alias for `Reg<ADC_LTR2_SPEC>`"]
pub type ADC_LTR2 = crate::Reg<adc_ltr2::ADC_LTR2_SPEC>;
#[doc = "ADC watchdog lower threshold register 2"]
pub mod adc_ltr2;
#[doc = "ADC_HTR2 register accessor: an alias for `Reg<ADC_HTR2_SPEC>`"]
pub type ADC_HTR2 = crate::Reg<adc_htr2::ADC_HTR2_SPEC>;
#[doc = "ADC watchdog higher threshold register 2"]
pub mod adc_htr2;
#[doc = "ADC_LTR3 register accessor: an alias for `Reg<ADC_LTR3_SPEC>`"]
pub type ADC_LTR3 = crate::Reg<adc_ltr3::ADC_LTR3_SPEC>;
#[doc = "ADC watchdog lower threshold register 3"]
pub mod adc_ltr3;
#[doc = "ADC_HTR3 register accessor: an alias for `Reg<ADC_HTR3_SPEC>`"]
pub type ADC_HTR3 = crate::Reg<adc_htr3::ADC_HTR3_SPEC>;
#[doc = "ADC watchdog higher threshold register 3"]
pub mod adc_htr3;
#[doc = "ADC_DIFSEL register accessor: an alias for `Reg<ADC_DIFSEL_SPEC>`"]
pub type ADC_DIFSEL = crate::Reg<adc_difsel::ADC_DIFSEL_SPEC>;
#[doc = "ADC differential mode selection register"]
pub mod adc_difsel;
#[doc = "ADC_CALFACT register accessor: an alias for `Reg<ADC_CALFACT_SPEC>`"]
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACT_SPEC>;
#[doc = "ADC calibration factors register"]
pub mod adc_calfact;
#[doc = "ADC_CALFACT2 register accessor: an alias for `Reg<ADC_CALFACT2_SPEC>`"]
pub type ADC_CALFACT2 = crate::Reg<adc_calfact2::ADC_CALFACT2_SPEC>;
#[doc = "ADC calibration factor register 2"]
pub mod adc_calfact2;
#[doc = "ADC2_OR register accessor: an alias for `Reg<ADC2_OR_SPEC>`"]
pub type ADC2_OR = crate::Reg<adc2_or::ADC2_OR_SPEC>;
#[doc = "ADC2 option register"]
pub mod adc2_or;
