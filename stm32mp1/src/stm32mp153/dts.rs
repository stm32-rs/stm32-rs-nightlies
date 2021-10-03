#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DTS_CFGR1 is the configuration register for temperature sensor 1."]
    pub dts_cfgr1: crate::Reg<dts_cfgr1::DTS_CFGR1_SPEC>,
    _reserved1: [u8; 0x04],
    #[doc = "0x08 - DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
    pub dts_t0valr1: crate::Reg<dts_t0valr1::DTS_T0VALR1_SPEC>,
    _reserved2: [u8; 0x04],
    #[doc = "0x10 - The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
    pub dts_rampvalr: crate::Reg<dts_rampvalr::DTS_RAMPVALR_SPEC>,
    #[doc = "0x14 - DTS_ITR1 contains the threshold values for sensor 1."]
    pub dts_itr1: crate::Reg<dts_itr1::DTS_ITR1_SPEC>,
    _reserved4: [u8; 0x04],
    #[doc = "0x1c - The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
    pub dts_dr: crate::Reg<dts_dr::DTS_DR_SPEC>,
    #[doc = "0x20 - Temperature sensor status register"]
    pub dts_sr: crate::Reg<dts_sr::DTS_SR_SPEC>,
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    pub dts_itenr: crate::Reg<dts_itenr::DTS_ITENR_SPEC>,
    #[doc = "0x28 - DTS_ICIFR is the control register for the interrupt flags."]
    pub dts_icifr: crate::Reg<dts_icifr::DTS_ICIFR_SPEC>,
    #[doc = "0x2c - The DTS_OR contains general-purpose option bits."]
    pub dts_or: crate::Reg<dts_or::DTS_OR_SPEC>,
}
#[doc = "DTS_CFGR1 register accessor: an alias for `Reg<DTS_CFGR1_SPEC>`"]
pub type DTS_CFGR1 = crate::Reg<dts_cfgr1::DTS_CFGR1_SPEC>;
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1."]
pub mod dts_cfgr1;
#[doc = "DTS_T0VALR1 register accessor: an alias for `Reg<DTS_T0VALR1_SPEC>`"]
pub type DTS_T0VALR1 = crate::Reg<dts_t0valr1::DTS_T0VALR1_SPEC>;
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
pub mod dts_t0valr1;
#[doc = "DTS_RAMPVALR register accessor: an alias for `Reg<DTS_RAMPVALR_SPEC>`"]
pub type DTS_RAMPVALR = crate::Reg<dts_rampvalr::DTS_RAMPVALR_SPEC>;
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
pub mod dts_rampvalr;
#[doc = "DTS_ITR1 register accessor: an alias for `Reg<DTS_ITR1_SPEC>`"]
pub type DTS_ITR1 = crate::Reg<dts_itr1::DTS_ITR1_SPEC>;
#[doc = "DTS_ITR1 contains the threshold values for sensor 1."]
pub mod dts_itr1;
#[doc = "DTS_DR register accessor: an alias for `Reg<DTS_DR_SPEC>`"]
pub type DTS_DR = crate::Reg<dts_dr::DTS_DR_SPEC>;
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
pub mod dts_dr;
#[doc = "DTS_SR register accessor: an alias for `Reg<DTS_SR_SPEC>`"]
pub type DTS_SR = crate::Reg<dts_sr::DTS_SR_SPEC>;
#[doc = "Temperature sensor status register"]
pub mod dts_sr;
#[doc = "DTS_ITENR register accessor: an alias for `Reg<DTS_ITENR_SPEC>`"]
pub type DTS_ITENR = crate::Reg<dts_itenr::DTS_ITENR_SPEC>;
#[doc = "Temperature sensor interrupt enable register"]
pub mod dts_itenr;
#[doc = "DTS_ICIFR register accessor: an alias for `Reg<DTS_ICIFR_SPEC>`"]
pub type DTS_ICIFR = crate::Reg<dts_icifr::DTS_ICIFR_SPEC>;
#[doc = "DTS_ICIFR is the control register for the interrupt flags."]
pub mod dts_icifr;
#[doc = "DTS_OR register accessor: an alias for `Reg<DTS_OR_SPEC>`"]
pub type DTS_OR = crate::Reg<dts_or::DTS_OR_SPEC>;
#[doc = "The DTS_OR contains general-purpose option bits."]
pub mod dts_or;
