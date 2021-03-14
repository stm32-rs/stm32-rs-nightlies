#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DTS_CFGR1 is the configuration register for temperature sensor 1."]
    pub dts_cfgr1: DTS_CFGR1,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
    pub dts_t0valr1: DTS_T0VALR1,
    _reserved2: [u8; 4usize],
    #[doc = "0x10 - The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
    pub dts_rampvalr: DTS_RAMPVALR,
    #[doc = "0x14 - DTS_ITR1 contains the threshold values for sensor 1."]
    pub dts_itr1: DTS_ITR1,
    _reserved4: [u8; 4usize],
    #[doc = "0x1c - The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
    pub dts_dr: DTS_DR,
    #[doc = "0x20 - Temperature sensor status register"]
    pub dts_sr: DTS_SR,
    #[doc = "0x24 - Temperature sensor interrupt enable register"]
    pub dts_itenr: DTS_ITENR,
    #[doc = "0x28 - DTS_ICIFR is the control register for the interrupt flags."]
    pub dts_icifr: DTS_ICIFR,
    #[doc = "0x2c - The DTS_OR contains general-purpose option bits."]
    pub dts_or: DTS_OR,
}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_cfgr1](dts_cfgr1) module"]
pub type DTS_CFGR1 = crate::Reg<u32, _DTS_CFGR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_CFGR1;
#[doc = "`read()` method returns [dts_cfgr1::R](dts_cfgr1::R) reader structure"]
impl crate::Readable for DTS_CFGR1 {}
#[doc = "`write(|w| ..)` method takes [dts_cfgr1::W](dts_cfgr1::W) writer structure"]
impl crate::Writable for DTS_CFGR1 {}
#[doc = "DTS_CFGR1 is the configuration register for temperature sensor 1."]
pub mod dts_cfgr1;
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_t0valr1](dts_t0valr1) module"]
pub type DTS_T0VALR1 = crate::Reg<u32, _DTS_T0VALR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_T0VALR1;
#[doc = "`read()` method returns [dts_t0valr1::R](dts_t0valr1::R) reader structure"]
impl crate::Readable for DTS_T0VALR1 {}
#[doc = "DTS_T0VALR1 contains the value of the factory calibration temperature (T0) for temperature sensor 1. The system reset value is factory trimmed."]
pub mod dts_t0valr1;
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_rampvalr](dts_rampvalr) module"]
pub type DTS_RAMPVALR = crate::Reg<u32, _DTS_RAMPVALR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_RAMPVALR;
#[doc = "`read()` method returns [dts_rampvalr::R](dts_rampvalr::R) reader structure"]
impl crate::Readable for DTS_RAMPVALR {}
#[doc = "The DTS_RAMPVALR is the ramp coefficient for the temperature sensor. The system reset value is factory trimmed."]
pub mod dts_rampvalr;
#[doc = "DTS_ITR1 contains the threshold values for sensor 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_itr1](dts_itr1) module"]
pub type DTS_ITR1 = crate::Reg<u32, _DTS_ITR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_ITR1;
#[doc = "`read()` method returns [dts_itr1::R](dts_itr1::R) reader structure"]
impl crate::Readable for DTS_ITR1 {}
#[doc = "`write(|w| ..)` method takes [dts_itr1::W](dts_itr1::W) writer structure"]
impl crate::Writable for DTS_ITR1 {}
#[doc = "DTS_ITR1 contains the threshold values for sensor 1."]
pub mod dts_itr1;
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_dr](dts_dr) module"]
pub type DTS_DR = crate::Reg<u32, _DTS_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_DR;
#[doc = "`read()` method returns [dts_dr::R](dts_dr::R) reader structure"]
impl crate::Readable for DTS_DR {}
#[doc = "`write(|w| ..)` method takes [dts_dr::W](dts_dr::W) writer structure"]
impl crate::Writable for DTS_DR {}
#[doc = "The DTS_DR contains the number of REF_CLK cycles used to compute the FM(T) frequency."]
pub mod dts_dr;
#[doc = "Temperature sensor status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_sr](dts_sr) module"]
pub type DTS_SR = crate::Reg<u32, _DTS_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_SR;
#[doc = "`read()` method returns [dts_sr::R](dts_sr::R) reader structure"]
impl crate::Readable for DTS_SR {}
#[doc = "Temperature sensor status register"]
pub mod dts_sr;
#[doc = "Temperature sensor interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_itenr](dts_itenr) module"]
pub type DTS_ITENR = crate::Reg<u32, _DTS_ITENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_ITENR;
#[doc = "`read()` method returns [dts_itenr::R](dts_itenr::R) reader structure"]
impl crate::Readable for DTS_ITENR {}
#[doc = "`write(|w| ..)` method takes [dts_itenr::W](dts_itenr::W) writer structure"]
impl crate::Writable for DTS_ITENR {}
#[doc = "Temperature sensor interrupt enable register"]
pub mod dts_itenr;
#[doc = "DTS_ICIFR is the control register for the interrupt flags.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_icifr](dts_icifr) module"]
pub type DTS_ICIFR = crate::Reg<u32, _DTS_ICIFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_ICIFR;
#[doc = "`read()` method returns [dts_icifr::R](dts_icifr::R) reader structure"]
impl crate::Readable for DTS_ICIFR {}
#[doc = "`write(|w| ..)` method takes [dts_icifr::W](dts_icifr::W) writer structure"]
impl crate::Writable for DTS_ICIFR {}
#[doc = "DTS_ICIFR is the control register for the interrupt flags."]
pub mod dts_icifr;
#[doc = "The DTS_OR contains general-purpose option bits.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dts_or](dts_or) module"]
pub type DTS_OR = crate::Reg<u32, _DTS_OR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTS_OR;
#[doc = "`read()` method returns [dts_or::R](dts_or::R) reader structure"]
impl crate::Readable for DTS_OR {}
#[doc = "`write(|w| ..)` method takes [dts_or::W](dts_or::W) writer structure"]
impl crate::Writable for DTS_OR {}
#[doc = "The DTS_OR contains general-purpose option bits."]
pub mod dts_or;
