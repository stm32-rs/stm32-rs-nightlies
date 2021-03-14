#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Port configuration register low (GPIOn_CRL)"]
    pub crl: CRL,
    #[doc = "0x04 - Port configuration register high (GPIOn_CRL)"]
    pub crh: CRH,
    #[doc = "0x08 - Port input data register (GPIOn_IDR)"]
    pub idr: IDR,
    #[doc = "0x0c - Port output data register (GPIOn_ODR)"]
    pub odr: ODR,
    #[doc = "0x10 - Port bit set/reset register (GPIOn_BSRR)"]
    pub bsrr: BSRR,
    #[doc = "0x14 - Port bit reset register (GPIOn_BRR)"]
    pub brr: BRR,
    #[doc = "0x18 - Port configuration lock register"]
    pub lckr: LCKR,
}
#[doc = "Port configuration register low (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crl](crl) module"]
pub type CRL = crate::Reg<u32, _CRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRL;
#[doc = "`read()` method returns [crl::R](crl::R) reader structure"]
impl crate::Readable for CRL {}
#[doc = "`write(|w| ..)` method takes [crl::W](crl::W) writer structure"]
impl crate::Writable for CRL {}
#[doc = "Port configuration register low (GPIOn_CRL)"]
pub mod crl;
#[doc = "Port configuration register high (GPIOn_CRL)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](crh) module"]
pub type CRH = crate::Reg<u32, _CRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRH;
#[doc = "`read()` method returns [crh::R](crh::R) reader structure"]
impl crate::Readable for CRH {}
#[doc = "`write(|w| ..)` method takes [crh::W](crh::W) writer structure"]
impl crate::Writable for CRH {}
#[doc = "Port configuration register high (GPIOn_CRL)"]
pub mod crh;
#[doc = "Port input data register (GPIOn_IDR)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "Port input data register (GPIOn_IDR)"]
pub mod idr;
#[doc = "Port output data register (GPIOn_ODR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odr](odr) module"]
pub type ODR = crate::Reg<u32, _ODR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODR;
#[doc = "`read()` method returns [odr::R](odr::R) reader structure"]
impl crate::Readable for ODR {}
#[doc = "`write(|w| ..)` method takes [odr::W](odr::W) writer structure"]
impl crate::Writable for ODR {}
#[doc = "Port output data register (GPIOn_ODR)"]
pub mod odr;
#[doc = "Port bit set/reset register (GPIOn_BSRR)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsrr](bsrr) module"]
pub type BSRR = crate::Reg<u32, _BSRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSRR;
#[doc = "`write(|w| ..)` method takes [bsrr::W](bsrr::W) writer structure"]
impl crate::Writable for BSRR {}
#[doc = "Port bit set/reset register (GPIOn_BSRR)"]
pub mod bsrr;
#[doc = "Port bit reset register (GPIOn_BRR)\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [brr](brr) module"]
pub type BRR = crate::Reg<u32, _BRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BRR;
#[doc = "`write(|w| ..)` method takes [brr::W](brr::W) writer structure"]
impl crate::Writable for BRR {}
#[doc = "Port bit reset register (GPIOn_BRR)"]
pub mod brr;
#[doc = "Port configuration lock register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lckr](lckr) module"]
pub type LCKR = crate::Reg<u32, _LCKR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _LCKR;
#[doc = "`read()` method returns [lckr::R](lckr::R) reader structure"]
impl crate::Readable for LCKR {}
#[doc = "`write(|w| ..)` method takes [lckr::W](lckr::W) writer structure"]
impl crate::Writable for LCKR {}
#[doc = "Port configuration lock register"]
pub mod lckr;
