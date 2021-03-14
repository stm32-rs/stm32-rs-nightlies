#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPCC Processor 1 control register"]
    pub c1cr: C1CR,
    #[doc = "0x04 - IPCC Processor 1 mask register"]
    pub c1mr: C1MR,
    #[doc = "0x08 - Reading this register will always return 0x0000 0000."]
    pub c1scr: C1SCR,
    #[doc = "0x0c - IPCC processor 1 to processor 2 status register"]
    pub ic1toc2sr: IC1TOC2SR,
    #[doc = "0x10 - IPCC Processor 2 control register"]
    pub c2cr: C2CR,
    #[doc = "0x14 - IPCC Processor 2 mask register"]
    pub c2mr: C2MR,
    #[doc = "0x18 - Reading this register will always return 0x0000 0000."]
    pub c2scr: C2SCR,
    #[doc = "0x1c - IPCC processor 2 to processor 1 status register"]
    pub c2toc1sr: C2TOC1SR,
    _reserved8: [u8; 976usize],
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    pub hwcfgr: HWCFGR,
    #[doc = "0x3f4 - IPCC IP Version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - IPCC IP Identification register"]
    pub ipidr: IPIDR,
    #[doc = "0x3fc - IPCC Size ID register"]
    pub sidr: SIDR,
}
#[doc = "IPCC Processor 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1cr](c1cr) module"]
pub type C1CR = crate::Reg<u32, _C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1CR;
#[doc = "`read()` method returns [c1cr::R](c1cr::R) reader structure"]
impl crate::Readable for C1CR {}
#[doc = "`write(|w| ..)` method takes [c1cr::W](c1cr::W) writer structure"]
impl crate::Writable for C1CR {}
#[doc = "IPCC Processor 1 control register"]
pub mod c1cr;
#[doc = "IPCC Processor 1 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1mr](c1mr) module"]
pub type C1MR = crate::Reg<u32, _C1MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1MR;
#[doc = "`read()` method returns [c1mr::R](c1mr::R) reader structure"]
impl crate::Readable for C1MR {}
#[doc = "`write(|w| ..)` method takes [c1mr::W](c1mr::W) writer structure"]
impl crate::Writable for C1MR {}
#[doc = "IPCC Processor 1 mask register"]
pub mod c1mr;
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1scr](c1scr) module"]
pub type C1SCR = crate::Reg<u32, _C1SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C1SCR;
#[doc = "`read()` method returns [c1scr::R](c1scr::R) reader structure"]
impl crate::Readable for C1SCR {}
#[doc = "`write(|w| ..)` method takes [c1scr::W](c1scr::W) writer structure"]
impl crate::Writable for C1SCR {}
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod c1scr;
#[doc = "IPCC processor 1 to processor 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ic1toc2sr](ic1toc2sr) module"]
pub type IC1TOC2SR = crate::Reg<u32, _IC1TOC2SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IC1TOC2SR;
#[doc = "`read()` method returns [ic1toc2sr::R](ic1toc2sr::R) reader structure"]
impl crate::Readable for IC1TOC2SR {}
#[doc = "IPCC processor 1 to processor 2 status register"]
pub mod ic1toc2sr;
#[doc = "IPCC Processor 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2cr](c2cr) module"]
pub type C2CR = crate::Reg<u32, _C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2CR;
#[doc = "`read()` method returns [c2cr::R](c2cr::R) reader structure"]
impl crate::Readable for C2CR {}
#[doc = "`write(|w| ..)` method takes [c2cr::W](c2cr::W) writer structure"]
impl crate::Writable for C2CR {}
#[doc = "IPCC Processor 2 control register"]
pub mod c2cr;
#[doc = "IPCC Processor 2 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2mr](c2mr) module"]
pub type C2MR = crate::Reg<u32, _C2MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2MR;
#[doc = "`read()` method returns [c2mr::R](c2mr::R) reader structure"]
impl crate::Readable for C2MR {}
#[doc = "`write(|w| ..)` method takes [c2mr::W](c2mr::W) writer structure"]
impl crate::Writable for C2MR {}
#[doc = "IPCC Processor 2 mask register"]
pub mod c2mr;
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2scr](c2scr) module"]
pub type C2SCR = crate::Reg<u32, _C2SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2SCR;
#[doc = "`read()` method returns [c2scr::R](c2scr::R) reader structure"]
impl crate::Readable for C2SCR {}
#[doc = "`write(|w| ..)` method takes [c2scr::W](c2scr::W) writer structure"]
impl crate::Writable for C2SCR {}
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod c2scr;
#[doc = "IPCC processor 2 to processor 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2toc1sr](c2toc1sr) module"]
pub type C2TOC1SR = crate::Reg<u32, _C2TOC1SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2TOC1SR;
#[doc = "`read()` method returns [c2toc1sr::R](c2toc1sr::R) reader structure"]
impl crate::Readable for C2TOC1SR {}
#[doc = "IPCC processor 2 to processor 1 status register"]
pub mod c2toc1sr;
#[doc = "IPCC Hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwcfgr](hwcfgr) module"]
pub type HWCFGR = crate::Reg<u32, _HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _HWCFGR;
#[doc = "`read()` method returns [hwcfgr::R](hwcfgr::R) reader structure"]
impl crate::Readable for HWCFGR {}
#[doc = "IPCC Hardware configuration register"]
pub mod hwcfgr;
#[doc = "IPCC IP Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "IPCC IP Version register"]
pub mod verr;
#[doc = "IPCC IP Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipidr](ipidr) module"]
pub type IPIDR = crate::Reg<u32, _IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPIDR;
#[doc = "`read()` method returns [ipidr::R](ipidr::R) reader structure"]
impl crate::Readable for IPIDR {}
#[doc = "IPCC IP Identification register"]
pub mod ipidr;
#[doc = "IPCC Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "IPCC Size ID register"]
pub mod sidr;
