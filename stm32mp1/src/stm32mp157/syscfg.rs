#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
    pub syscfg_bootr: SYSCFG_BOOTR,
    #[doc = "0x04 - SYSCFG peripheral mode configuration set register"]
    pub syscfg_pmcsetr: SYSCFG_PMCSETR,
    _reserved2: [u8; 16usize],
    #[doc = "0x18 - SYSCFG IO control register"]
    pub syscfg_ioctrlsetr: SYSCFG_IOCTRLSETR,
    #[doc = "0x1c - SYSCFG interconnect control register"]
    pub syscfg_icnr: SYSCFG_ICNR,
    #[doc = "0x20 - SYSCFG compensation cell control register"]
    pub syscfg_cmpcr: SYSCFG_CMPCR,
    #[doc = "0x24 - SYSCFG compensation cell enable set register"]
    pub syscfg_cmpensetr: SYSCFG_CMPENSETR,
    #[doc = "0x28 - SYSCFG compensation cell enable set register"]
    pub syscfg_cmpenclrr: SYSCFG_CMPENCLRR,
    #[doc = "0x2c - SYSCFG control timer break register"]
    pub syscfg_cbr: SYSCFG_CBR,
    _reserved8: [u8; 20usize],
    #[doc = "0x44 - SYSCFG peripheral mode configuration clear register"]
    pub syscfg_pmcclrr: SYSCFG_PMCCLRR,
    _reserved9: [u8; 16usize],
    #[doc = "0x58 - SYSCFG IO control register"]
    pub syscfg_ioctrlclrr: SYSCFG_IOCTRLCLRR,
    _reserved10: [u8; 920usize],
    #[doc = "0x3f4 - SYSCFG version register"]
    pub syscfg_verr: SYSCFG_VERR,
    #[doc = "0x3f8 - SYSCFG identification register"]
    pub syscfg_ipidr: SYSCFG_IPIDR,
    #[doc = "0x3fc - SYSCFG size identification register"]
    pub syscfg_sidr: SYSCFG_SIDR,
}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_bootr](syscfg_bootr) module"]
pub type SYSCFG_BOOTR = crate::Reg<u32, _SYSCFG_BOOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_BOOTR;
#[doc = "`read()` method returns [syscfg_bootr::R](syscfg_bootr::R) reader structure"]
impl crate::Readable for SYSCFG_BOOTR {}
#[doc = "`write(|w| ..)` method takes [syscfg_bootr::W](syscfg_bootr::W) writer structure"]
impl crate::Writable for SYSCFG_BOOTR {}
#[doc = "This register is used to know the state of BOOT pins and to control pull-up to reduce the static power consumption on the pin set to high level. )"]
pub mod syscfg_bootr;
#[doc = "SYSCFG peripheral mode configuration set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_pmcsetr](syscfg_pmcsetr) module"]
pub type SYSCFG_PMCSETR = crate::Reg<u32, _SYSCFG_PMCSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_PMCSETR;
#[doc = "`read()` method returns [syscfg_pmcsetr::R](syscfg_pmcsetr::R) reader structure"]
impl crate::Readable for SYSCFG_PMCSETR {}
#[doc = "`write(|w| ..)` method takes [syscfg_pmcsetr::W](syscfg_pmcsetr::W) writer structure"]
impl crate::Writable for SYSCFG_PMCSETR {}
#[doc = "SYSCFG peripheral mode configuration set register"]
pub mod syscfg_pmcsetr;
#[doc = "SYSCFG IO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ioctrlsetr](syscfg_ioctrlsetr) module"]
pub type SYSCFG_IOCTRLSETR = crate::Reg<u32, _SYSCFG_IOCTRLSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_IOCTRLSETR;
#[doc = "`read()` method returns [syscfg_ioctrlsetr::R](syscfg_ioctrlsetr::R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLSETR {}
#[doc = "`write(|w| ..)` method takes [syscfg_ioctrlsetr::W](syscfg_ioctrlsetr::W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLSETR {}
#[doc = "SYSCFG IO control register"]
pub mod syscfg_ioctrlsetr;
#[doc = "SYSCFG interconnect control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_icnr](syscfg_icnr) module"]
pub type SYSCFG_ICNR = crate::Reg<u32, _SYSCFG_ICNR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_ICNR;
#[doc = "`read()` method returns [syscfg_icnr::R](syscfg_icnr::R) reader structure"]
impl crate::Readable for SYSCFG_ICNR {}
#[doc = "`write(|w| ..)` method takes [syscfg_icnr::W](syscfg_icnr::W) writer structure"]
impl crate::Writable for SYSCFG_ICNR {}
#[doc = "SYSCFG interconnect control register"]
pub mod syscfg_icnr;
#[doc = "SYSCFG compensation cell control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpcr](syscfg_cmpcr) module"]
pub type SYSCFG_CMPCR = crate::Reg<u32, _SYSCFG_CMPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CMPCR;
#[doc = "`read()` method returns [syscfg_cmpcr::R](syscfg_cmpcr::R) reader structure"]
impl crate::Readable for SYSCFG_CMPCR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpcr::W](syscfg_cmpcr::W) writer structure"]
impl crate::Writable for SYSCFG_CMPCR {}
#[doc = "SYSCFG compensation cell control register"]
pub mod syscfg_cmpcr;
#[doc = "SYSCFG compensation cell enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpensetr](syscfg_cmpensetr) module"]
pub type SYSCFG_CMPENSETR = crate::Reg<u32, _SYSCFG_CMPENSETR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CMPENSETR;
#[doc = "`read()` method returns [syscfg_cmpensetr::R](syscfg_cmpensetr::R) reader structure"]
impl crate::Readable for SYSCFG_CMPENSETR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpensetr::W](syscfg_cmpensetr::W) writer structure"]
impl crate::Writable for SYSCFG_CMPENSETR {}
#[doc = "SYSCFG compensation cell enable set register"]
pub mod syscfg_cmpensetr;
#[doc = "SYSCFG compensation cell enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpenclrr](syscfg_cmpenclrr) module"]
pub type SYSCFG_CMPENCLRR = crate::Reg<u32, _SYSCFG_CMPENCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CMPENCLRR;
#[doc = "`read()` method returns [syscfg_cmpenclrr::R](syscfg_cmpenclrr::R) reader structure"]
impl crate::Readable for SYSCFG_CMPENCLRR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpenclrr::W](syscfg_cmpenclrr::W) writer structure"]
impl crate::Writable for SYSCFG_CMPENCLRR {}
#[doc = "SYSCFG compensation cell enable set register"]
pub mod syscfg_cmpenclrr;
#[doc = "SYSCFG control timer break register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cbr](syscfg_cbr) module"]
pub type SYSCFG_CBR = crate::Reg<u32, _SYSCFG_CBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_CBR;
#[doc = "`read()` method returns [syscfg_cbr::R](syscfg_cbr::R) reader structure"]
impl crate::Readable for SYSCFG_CBR {}
#[doc = "`write(|w| ..)` method takes [syscfg_cbr::W](syscfg_cbr::W) writer structure"]
impl crate::Writable for SYSCFG_CBR {}
#[doc = "SYSCFG control timer break register"]
pub mod syscfg_cbr;
#[doc = "SYSCFG peripheral mode configuration clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_pmcclrr](syscfg_pmcclrr) module"]
pub type SYSCFG_PMCCLRR = crate::Reg<u32, _SYSCFG_PMCCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_PMCCLRR;
#[doc = "`read()` method returns [syscfg_pmcclrr::R](syscfg_pmcclrr::R) reader structure"]
impl crate::Readable for SYSCFG_PMCCLRR {}
#[doc = "`write(|w| ..)` method takes [syscfg_pmcclrr::W](syscfg_pmcclrr::W) writer structure"]
impl crate::Writable for SYSCFG_PMCCLRR {}
#[doc = "SYSCFG peripheral mode configuration clear register"]
pub mod syscfg_pmcclrr;
#[doc = "SYSCFG IO control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ioctrlclrr](syscfg_ioctrlclrr) module"]
pub type SYSCFG_IOCTRLCLRR = crate::Reg<u32, _SYSCFG_IOCTRLCLRR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_IOCTRLCLRR;
#[doc = "`read()` method returns [syscfg_ioctrlclrr::R](syscfg_ioctrlclrr::R) reader structure"]
impl crate::Readable for SYSCFG_IOCTRLCLRR {}
#[doc = "`write(|w| ..)` method takes [syscfg_ioctrlclrr::W](syscfg_ioctrlclrr::W) writer structure"]
impl crate::Writable for SYSCFG_IOCTRLCLRR {}
#[doc = "SYSCFG IO control register"]
pub mod syscfg_ioctrlclrr;
#[doc = "SYSCFG version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_verr](syscfg_verr) module"]
pub type SYSCFG_VERR = crate::Reg<u32, _SYSCFG_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_VERR;
#[doc = "`read()` method returns [syscfg_verr::R](syscfg_verr::R) reader structure"]
impl crate::Readable for SYSCFG_VERR {}
#[doc = "SYSCFG version register"]
pub mod syscfg_verr;
#[doc = "SYSCFG identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_ipidr](syscfg_ipidr) module"]
pub type SYSCFG_IPIDR = crate::Reg<u32, _SYSCFG_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_IPIDR;
#[doc = "`read()` method returns [syscfg_ipidr::R](syscfg_ipidr::R) reader structure"]
impl crate::Readable for SYSCFG_IPIDR {}
#[doc = "SYSCFG identification register"]
pub mod syscfg_ipidr;
#[doc = "SYSCFG size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_sidr](syscfg_sidr) module"]
pub type SYSCFG_SIDR = crate::Reg<u32, _SYSCFG_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SYSCFG_SIDR;
#[doc = "`read()` method returns [syscfg_sidr::R](syscfg_sidr::R) reader structure"]
impl crate::Readable for SYSCFG_SIDR {}
#[doc = "SYSCFG size identification register"]
pub mod syscfg_sidr;
