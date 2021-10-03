#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    pub adc_isr: crate::Reg<adc_isr::ADC_ISR_SPEC>,
    #[doc = "0x04 - ADC interrupt enable register"]
    pub adc_ier: crate::Reg<adc_ier::ADC_IER_SPEC>,
    #[doc = "0x08 - ADC control register"]
    pub adc_cr: crate::Reg<adc_cr::ADC_CR_SPEC>,
    #[doc = "0x0c - ADC configuration register 1"]
    pub adc_cfgr1: crate::Reg<adc_cfgr1::ADC_CFGR1_SPEC>,
    #[doc = "0x10 - ADC configuration register 2"]
    pub adc_cfgr2: crate::Reg<adc_cfgr2::ADC_CFGR2_SPEC>,
    #[doc = "0x14 - ADC sampling time register"]
    pub adc_smpr: crate::Reg<adc_smpr::ADC_SMPR_SPEC>,
    _reserved6: [u8; 0x08],
    #[doc = "0x20 - ADC watchdog threshold register"]
    pub adc_awd1tr: crate::Reg<adc_awd1tr::ADC_AWD1TR_SPEC>,
    #[doc = "0x24 - ADC watchdog threshold register"]
    pub adc_awd2tr: crate::Reg<adc_awd2tr::ADC_AWD2TR_SPEC>,
    _reserved_8_adc_chselrmod: [u8; 0x04],
    #[doc = "0x2c - ADC watchdog threshold register"]
    pub adc_awd3tr: crate::Reg<adc_awd3tr::ADC_AWD3TR_SPEC>,
    _reserved10: [u8; 0x10],
    #[doc = "0x40 - ADC data register"]
    pub adc_dr: crate::Reg<adc_dr::ADC_DR_SPEC>,
    _reserved11: [u8; 0x5c],
    #[doc = "0xa0 - ADC Analog Watchdog 2 Configuration register"]
    pub adc_awd2cr: crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>,
    #[doc = "0xa4 - ADC Analog Watchdog 3 Configuration register"]
    pub adc_awd3cr: crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>,
    _reserved13: [u8; 0x0c],
    #[doc = "0xb4 - ADC Calibration factor"]
    pub adc_calfact: crate::Reg<adc_calfact::ADC_CALFACT_SPEC>,
    _reserved14: [u8; 0x0250],
    #[doc = "0x308 - ADC common configuration register"]
    pub adc_ccr: crate::Reg<adc_ccr::ADC_CCR_SPEC>,
}
impl RegisterBlock {
    #[doc = "0x28 - ADC channel selection register"]
    #[inline(always)]
    pub fn adc_chselrmod1(&self) -> &crate::Reg<adc_chselrmod1::ADC_CHSELRMOD1_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<adc_chselrmod1::ADC_CHSELRMOD1_SPEC>)
        }
    }
    #[doc = "0x28 - ADC channel selection register"]
    #[inline(always)]
    pub fn adc_chselrmod0(&self) -> &crate::Reg<adc_chselrmod0::ADC_CHSELRMOD0_SPEC> {
        unsafe {
            &*(((self as *const Self) as *const u8).add(40usize)
                as *const crate::Reg<adc_chselrmod0::ADC_CHSELRMOD0_SPEC>)
        }
    }
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
#[doc = "ADC_CFGR1 register accessor: an alias for `Reg<ADC_CFGR1_SPEC>`"]
pub type ADC_CFGR1 = crate::Reg<adc_cfgr1::ADC_CFGR1_SPEC>;
#[doc = "ADC configuration register 1"]
pub mod adc_cfgr1;
#[doc = "ADC_CFGR2 register accessor: an alias for `Reg<ADC_CFGR2_SPEC>`"]
pub type ADC_CFGR2 = crate::Reg<adc_cfgr2::ADC_CFGR2_SPEC>;
#[doc = "ADC configuration register 2"]
pub mod adc_cfgr2;
#[doc = "ADC_SMPR register accessor: an alias for `Reg<ADC_SMPR_SPEC>`"]
pub type ADC_SMPR = crate::Reg<adc_smpr::ADC_SMPR_SPEC>;
#[doc = "ADC sampling time register"]
pub mod adc_smpr;
#[doc = "ADC_AWD1TR register accessor: an alias for `Reg<ADC_AWD1TR_SPEC>`"]
pub type ADC_AWD1TR = crate::Reg<adc_awd1tr::ADC_AWD1TR_SPEC>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd1tr;
#[doc = "ADC_AWD2TR register accessor: an alias for `Reg<ADC_AWD2TR_SPEC>`"]
pub type ADC_AWD2TR = crate::Reg<adc_awd2tr::ADC_AWD2TR_SPEC>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd2tr;
#[doc = "ADC_CHSELRMOD0 register accessor: an alias for `Reg<ADC_CHSELRMOD0_SPEC>`"]
pub type ADC_CHSELRMOD0 = crate::Reg<adc_chselrmod0::ADC_CHSELRMOD0_SPEC>;
#[doc = "ADC channel selection register"]
pub mod adc_chselrmod0;
#[doc = "ADC_CHSELRMOD1 register accessor: an alias for `Reg<ADC_CHSELRMOD1_SPEC>`"]
pub type ADC_CHSELRMOD1 = crate::Reg<adc_chselrmod1::ADC_CHSELRMOD1_SPEC>;
#[doc = "ADC channel selection register"]
pub mod adc_chselrmod1;
#[doc = "ADC_AWD3TR register accessor: an alias for `Reg<ADC_AWD3TR_SPEC>`"]
pub type ADC_AWD3TR = crate::Reg<adc_awd3tr::ADC_AWD3TR_SPEC>;
#[doc = "ADC watchdog threshold register"]
pub mod adc_awd3tr;
#[doc = "ADC_DR register accessor: an alias for `Reg<ADC_DR_SPEC>`"]
pub type ADC_DR = crate::Reg<adc_dr::ADC_DR_SPEC>;
#[doc = "ADC data register"]
pub mod adc_dr;
#[doc = "ADC_AWD2CR register accessor: an alias for `Reg<ADC_AWD2CR_SPEC>`"]
pub type ADC_AWD2CR = crate::Reg<adc_awd2cr::ADC_AWD2CR_SPEC>;
#[doc = "ADC Analog Watchdog 2 Configuration register"]
pub mod adc_awd2cr;
#[doc = "ADC_AWD3CR register accessor: an alias for `Reg<ADC_AWD3CR_SPEC>`"]
pub type ADC_AWD3CR = crate::Reg<adc_awd3cr::ADC_AWD3CR_SPEC>;
#[doc = "ADC Analog Watchdog 3 Configuration register"]
pub mod adc_awd3cr;
#[doc = "ADC_CALFACT register accessor: an alias for `Reg<ADC_CALFACT_SPEC>`"]
pub type ADC_CALFACT = crate::Reg<adc_calfact::ADC_CALFACT_SPEC>;
#[doc = "ADC Calibration factor"]
pub mod adc_calfact;
#[doc = "ADC_CCR register accessor: an alias for `Reg<ADC_CCR_SPEC>`"]
pub type ADC_CCR = crate::Reg<adc_ccr::ADC_CCR_SPEC>;
#[doc = "ADC common configuration register"]
pub mod adc_ccr;
