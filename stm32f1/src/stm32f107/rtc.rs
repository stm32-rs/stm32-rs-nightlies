#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - RTC Control Register High"]
    pub crh: CRH,
    #[doc = "0x04 - RTC Control Register Low"]
    pub crl: CRL,
    #[doc = "0x08 - RTC Prescaler Load Register High"]
    pub prlh: PRLH,
    #[doc = "0x0c - RTC Prescaler Load Register Low"]
    pub prll: PRLL,
    #[doc = "0x10 - RTC Prescaler Divider Register High"]
    pub divh: DIVH,
    #[doc = "0x14 - RTC Prescaler Divider Register Low"]
    pub divl: DIVL,
    #[doc = "0x18 - RTC Counter Register High"]
    pub cnth: CNTH,
    #[doc = "0x1c - RTC Counter Register Low"]
    pub cntl: CNTL,
    #[doc = "0x20 - RTC Alarm Register High"]
    pub alrh: ALRH,
    #[doc = "0x24 - RTC Alarm Register Low"]
    pub alrl: ALRL,
}
#[doc = "RTC Control Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crh](crh) module"]
pub type CRH = crate::Reg<u32, _CRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRH;
#[doc = "`read()` method returns [crh::R](crh::R) reader structure"]
impl crate::Readable for CRH {}
#[doc = "`write(|w| ..)` method takes [crh::W](crh::W) writer structure"]
impl crate::Writable for CRH {}
#[doc = "RTC Control Register High"]
pub mod crh;
#[doc = "RTC Control Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crl](crl) module"]
pub type CRL = crate::Reg<u32, _CRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CRL;
#[doc = "`read()` method returns [crl::R](crl::R) reader structure"]
impl crate::Readable for CRL {}
#[doc = "`write(|w| ..)` method takes [crl::W](crl::W) writer structure"]
impl crate::Writable for CRL {}
#[doc = "RTC Control Register Low"]
pub mod crl;
#[doc = "RTC Prescaler Load Register High\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prlh](prlh) module"]
pub type PRLH = crate::Reg<u32, _PRLH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRLH;
#[doc = "`write(|w| ..)` method takes [prlh::W](prlh::W) writer structure"]
impl crate::Writable for PRLH {}
#[doc = "RTC Prescaler Load Register High"]
pub mod prlh;
#[doc = "RTC Prescaler Load Register Low\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prll](prll) module"]
pub type PRLL = crate::Reg<u32, _PRLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PRLL;
#[doc = "`write(|w| ..)` method takes [prll::W](prll::W) writer structure"]
impl crate::Writable for PRLL {}
#[doc = "RTC Prescaler Load Register Low"]
pub mod prll;
#[doc = "RTC Prescaler Divider Register High\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divh](divh) module"]
pub type DIVH = crate::Reg<u32, _DIVH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVH;
#[doc = "`read()` method returns [divh::R](divh::R) reader structure"]
impl crate::Readable for DIVH {}
#[doc = "RTC Prescaler Divider Register High"]
pub mod divh;
#[doc = "RTC Prescaler Divider Register Low\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [divl](divl) module"]
pub type DIVL = crate::Reg<u32, _DIVL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIVL;
#[doc = "`read()` method returns [divl::R](divl::R) reader structure"]
impl crate::Readable for DIVL {}
#[doc = "RTC Prescaler Divider Register Low"]
pub mod divl;
#[doc = "RTC Counter Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](cnth) module"]
pub type CNTH = crate::Reg<u32, _CNTH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTH;
#[doc = "`read()` method returns [cnth::R](cnth::R) reader structure"]
impl crate::Readable for CNTH {}
#[doc = "`write(|w| ..)` method takes [cnth::W](cnth::W) writer structure"]
impl crate::Writable for CNTH {}
#[doc = "RTC Counter Register High"]
pub mod cnth;
#[doc = "RTC Counter Register Low\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntl](cntl) module"]
pub type CNTL = crate::Reg<u32, _CNTL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTL;
#[doc = "`read()` method returns [cntl::R](cntl::R) reader structure"]
impl crate::Readable for CNTL {}
#[doc = "`write(|w| ..)` method takes [cntl::W](cntl::W) writer structure"]
impl crate::Writable for CNTL {}
#[doc = "RTC Counter Register Low"]
pub mod cntl;
#[doc = "RTC Alarm Register High\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrh](alrh) module"]
pub type ALRH = crate::Reg<u32, _ALRH>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRH;
#[doc = "`write(|w| ..)` method takes [alrh::W](alrh::W) writer structure"]
impl crate::Writable for ALRH {}
#[doc = "RTC Alarm Register High"]
pub mod alrh;
#[doc = "RTC Alarm Register Low\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrl](alrl) module"]
pub type ALRL = crate::Reg<u32, _ALRL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ALRL;
#[doc = "`write(|w| ..)` method takes [alrl::W](alrl::W) writer structure"]
impl crate::Writable for ALRL {}
#[doc = "RTC Alarm Register Low"]
pub mod alrl;
