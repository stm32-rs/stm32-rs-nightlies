#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - IPCC Processor 1 control register"]
    pub ipcc_c1cr: IPCC_C1CR,
    #[doc = "0x04 - IPCC Processor 1 mask register"]
    pub ipcc_c1mr: IPCC_C1MR,
    #[doc = "0x08 - Reading this register will always return 0x0000 0000."]
    pub ipcc_c1scr: IPCC_C1SCR,
    #[doc = "0x0c - IPCC processor 1 to processor 2 status register"]
    pub ipcc_c1toc2sr: IPCC_C1TOC2SR,
    #[doc = "0x10 - IPCC Processor 2 control register"]
    pub ipcc_c2cr: IPCC_C2CR,
    #[doc = "0x14 - IPCC Processor 2 mask register"]
    pub ipcc_c2mr: IPCC_C2MR,
    #[doc = "0x18 - Reading this register will always return 0x0000 0000."]
    pub ipcc_c2scr: IPCC_C2SCR,
    #[doc = "0x1c - IPCC processor 2 to processor 1 status register"]
    pub ipcc_c2toc1sr: IPCC_C2TOC1SR,
    _reserved8: [u8; 976usize],
    #[doc = "0x3f0 - IPCC Hardware configuration register"]
    pub ipcc_hwcfgr: IPCC_HWCFGR,
    #[doc = "0x3f4 - IPCC IP Version register"]
    pub ipcc_ver: IPCC_VER,
    #[doc = "0x3f8 - IPCC IP Identification register"]
    pub ipcc_id: IPCC_ID,
    #[doc = "0x3fc - IPCC Size ID register"]
    pub ipcc_sid: IPCC_SID,
}
#[doc = "IPCC Processor 1 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1cr](ipcc_c1cr) module"]
pub type IPCC_C1CR = crate::Reg<u32, _IPCC_C1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C1CR;
#[doc = "`read()` method returns [ipcc_c1cr::R](ipcc_c1cr::R) reader structure"]
impl crate::Readable for IPCC_C1CR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c1cr::W](ipcc_c1cr::W) writer structure"]
impl crate::Writable for IPCC_C1CR {}
#[doc = "IPCC Processor 1 control register"]
pub mod ipcc_c1cr;
#[doc = "IPCC Processor 1 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1mr](ipcc_c1mr) module"]
pub type IPCC_C1MR = crate::Reg<u32, _IPCC_C1MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C1MR;
#[doc = "`read()` method returns [ipcc_c1mr::R](ipcc_c1mr::R) reader structure"]
impl crate::Readable for IPCC_C1MR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c1mr::W](ipcc_c1mr::W) writer structure"]
impl crate::Writable for IPCC_C1MR {}
#[doc = "IPCC Processor 1 mask register"]
pub mod ipcc_c1mr;
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1scr](ipcc_c1scr) module"]
pub type IPCC_C1SCR = crate::Reg<u32, _IPCC_C1SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C1SCR;
#[doc = "`read()` method returns [ipcc_c1scr::R](ipcc_c1scr::R) reader structure"]
impl crate::Readable for IPCC_C1SCR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c1scr::W](ipcc_c1scr::W) writer structure"]
impl crate::Writable for IPCC_C1SCR {}
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod ipcc_c1scr;
#[doc = "IPCC processor 1 to processor 2 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c1toc2sr](ipcc_c1toc2sr) module"]
pub type IPCC_C1TOC2SR = crate::Reg<u32, _IPCC_C1TOC2SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C1TOC2SR;
#[doc = "`read()` method returns [ipcc_c1toc2sr::R](ipcc_c1toc2sr::R) reader structure"]
impl crate::Readable for IPCC_C1TOC2SR {}
#[doc = "IPCC processor 1 to processor 2 status register"]
pub mod ipcc_c1toc2sr;
#[doc = "IPCC Processor 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2cr](ipcc_c2cr) module"]
pub type IPCC_C2CR = crate::Reg<u32, _IPCC_C2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C2CR;
#[doc = "`read()` method returns [ipcc_c2cr::R](ipcc_c2cr::R) reader structure"]
impl crate::Readable for IPCC_C2CR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c2cr::W](ipcc_c2cr::W) writer structure"]
impl crate::Writable for IPCC_C2CR {}
#[doc = "IPCC Processor 2 control register"]
pub mod ipcc_c2cr;
#[doc = "IPCC Processor 2 mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2mr](ipcc_c2mr) module"]
pub type IPCC_C2MR = crate::Reg<u32, _IPCC_C2MR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C2MR;
#[doc = "`read()` method returns [ipcc_c2mr::R](ipcc_c2mr::R) reader structure"]
impl crate::Readable for IPCC_C2MR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c2mr::W](ipcc_c2mr::W) writer structure"]
impl crate::Writable for IPCC_C2MR {}
#[doc = "IPCC Processor 2 mask register"]
pub mod ipcc_c2mr;
#[doc = "Reading this register will always return 0x0000 0000.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2scr](ipcc_c2scr) module"]
pub type IPCC_C2SCR = crate::Reg<u32, _IPCC_C2SCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C2SCR;
#[doc = "`read()` method returns [ipcc_c2scr::R](ipcc_c2scr::R) reader structure"]
impl crate::Readable for IPCC_C2SCR {}
#[doc = "`write(|w| ..)` method takes [ipcc_c2scr::W](ipcc_c2scr::W) writer structure"]
impl crate::Writable for IPCC_C2SCR {}
#[doc = "Reading this register will always return 0x0000 0000."]
pub mod ipcc_c2scr;
#[doc = "IPCC processor 2 to processor 1 status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_c2toc1sr](ipcc_c2toc1sr) module"]
pub type IPCC_C2TOC1SR = crate::Reg<u32, _IPCC_C2TOC1SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_C2TOC1SR;
#[doc = "`read()` method returns [ipcc_c2toc1sr::R](ipcc_c2toc1sr::R) reader structure"]
impl crate::Readable for IPCC_C2TOC1SR {}
#[doc = "IPCC processor 2 to processor 1 status register"]
pub mod ipcc_c2toc1sr;
#[doc = "IPCC Hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_hwcfgr](ipcc_hwcfgr) module"]
pub type IPCC_HWCFGR = crate::Reg<u32, _IPCC_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_HWCFGR;
#[doc = "`read()` method returns [ipcc_hwcfgr::R](ipcc_hwcfgr::R) reader structure"]
impl crate::Readable for IPCC_HWCFGR {}
#[doc = "IPCC Hardware configuration register"]
pub mod ipcc_hwcfgr;
#[doc = "IPCC IP Version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_ver](ipcc_ver) module"]
pub type IPCC_VER = crate::Reg<u32, _IPCC_VER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_VER;
#[doc = "`read()` method returns [ipcc_ver::R](ipcc_ver::R) reader structure"]
impl crate::Readable for IPCC_VER {}
#[doc = "IPCC IP Version register"]
pub mod ipcc_ver;
#[doc = "IPCC IP Identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_id](ipcc_id) module"]
pub type IPCC_ID = crate::Reg<u32, _IPCC_ID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_ID;
#[doc = "`read()` method returns [ipcc_id::R](ipcc_id::R) reader structure"]
impl crate::Readable for IPCC_ID {}
#[doc = "IPCC IP Identification register"]
pub mod ipcc_id;
#[doc = "IPCC Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcc_sid](ipcc_sid) module"]
pub type IPCC_SID = crate::Reg<u32, _IPCC_SID>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IPCC_SID;
#[doc = "`read()` method returns [ipcc_sid::R](ipcc_sid::R) reader structure"]
impl crate::Readable for IPCC_SID {}
#[doc = "IPCC Size ID register"]
pub mod ipcc_sid;
