#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Backup data register (BKP_DR)"]
    pub dr: [DR; 10],
    #[doc = "0x28 - RTC clock calibration register (BKP_RTCCR)"]
    pub rtccr: RTCCR,
    #[doc = "0x2c - Backup control register (BKP_CR)"]
    pub cr: CR,
    #[doc = "0x30 - BKP_CSR control/status register"]
    pub csr: CSR,
    _reserved4: [u8; 8usize],
    #[doc = "0x3c - Backup data register (BKP_DR)"]
    pub bkp_dr: [BKP_DR; 32],
}
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "Backup data register (BKP_DR)"]
pub mod dr;
#[doc = "Backup data register (BKP_DR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp_dr](bkp_dr) module"]
pub type BKP_DR = crate::Reg<u32, _BKP_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BKP_DR;
#[doc = "`read()` method returns [bkp_dr::R](bkp_dr::R) reader structure"]
impl crate::Readable for BKP_DR {}
#[doc = "`write(|w| ..)` method takes [bkp_dr::W](bkp_dr::W) writer structure"]
impl crate::Writable for BKP_DR {}
#[doc = "Backup data register (BKP_DR)"]
pub mod bkp_dr;
#[doc = "RTC clock calibration register (BKP_RTCCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtccr](rtccr) module"]
pub type RTCCR = crate::Reg<u32, _RTCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RTCCR;
#[doc = "`read()` method returns [rtccr::R](rtccr::R) reader structure"]
impl crate::Readable for RTCCR {}
#[doc = "`write(|w| ..)` method takes [rtccr::W](rtccr::W) writer structure"]
impl crate::Writable for RTCCR {}
#[doc = "RTC clock calibration register (BKP_RTCCR)"]
pub mod rtccr;
#[doc = "Backup control register (BKP_CR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Backup control register (BKP_CR)"]
pub mod cr;
#[doc = "BKP_CSR control/status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "BKP_CSR control/status register"]
pub mod csr;
