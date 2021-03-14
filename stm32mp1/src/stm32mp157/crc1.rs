#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CRC data register"]
    pub crc_dr: CRC_DR,
    #[doc = "0x04 - CRC independent data register"]
    pub crc_idr: CRC_IDR,
    #[doc = "0x08 - CRC control register"]
    pub crc_cr: CRC_CR,
    _reserved3: [u8; 4usize],
    #[doc = "0x10 - CRC initial value"]
    pub crc_init: CRC_INIT,
    #[doc = "0x14 - CRC polynomial"]
    pub crc_pol: CRC_POL,
}
#[doc = "CRC data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_dr](crc_dr) module"]
pub type CRC_DR = crate::Reg<u32, _CRC_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_DR;
#[doc = "`read()` method returns [crc_dr::R](crc_dr::R) reader structure"]
impl crate::Readable for CRC_DR {}
#[doc = "`write(|w| ..)` method takes [crc_dr::W](crc_dr::W) writer structure"]
impl crate::Writable for CRC_DR {}
#[doc = "CRC data register"]
pub mod crc_dr;
#[doc = "CRC independent data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_idr](crc_idr) module"]
pub type CRC_IDR = crate::Reg<u32, _CRC_IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_IDR;
#[doc = "`read()` method returns [crc_idr::R](crc_idr::R) reader structure"]
impl crate::Readable for CRC_IDR {}
#[doc = "`write(|w| ..)` method takes [crc_idr::W](crc_idr::W) writer structure"]
impl crate::Writable for CRC_IDR {}
#[doc = "CRC independent data register"]
pub mod crc_idr;
#[doc = "CRC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_cr](crc_cr) module"]
pub type CRC_CR = crate::Reg<u32, _CRC_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_CR;
#[doc = "`read()` method returns [crc_cr::R](crc_cr::R) reader structure"]
impl crate::Readable for CRC_CR {}
#[doc = "`write(|w| ..)` method takes [crc_cr::W](crc_cr::W) writer structure"]
impl crate::Writable for CRC_CR {}
#[doc = "CRC control register"]
pub mod crc_cr;
#[doc = "CRC initial value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_init](crc_init) module"]
pub type CRC_INIT = crate::Reg<u32, _CRC_INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_INIT;
#[doc = "`read()` method returns [crc_init::R](crc_init::R) reader structure"]
impl crate::Readable for CRC_INIT {}
#[doc = "`write(|w| ..)` method takes [crc_init::W](crc_init::W) writer structure"]
impl crate::Writable for CRC_INIT {}
#[doc = "CRC initial value"]
pub mod crc_init;
#[doc = "CRC polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crc_pol](crc_pol) module"]
pub type CRC_POL = crate::Reg<u32, _CRC_POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRC_POL;
#[doc = "`read()` method returns [crc_pol::R](crc_pol::R) reader structure"]
impl crate::Readable for CRC_POL {}
#[doc = "`write(|w| ..)` method takes [crc_pol::W](crc_pol::W) writer structure"]
impl crate::Writable for CRC_POL {}
#[doc = "CRC polynomial"]
pub mod crc_pol;
