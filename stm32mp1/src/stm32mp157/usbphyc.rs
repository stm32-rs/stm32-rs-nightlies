#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - This register is used to control the PLL of the HS PHY."]
    pub usbphyc_pll: USBPHYC_PLL,
    _reserved1: [u8; 4usize],
    #[doc = "0x08 - This register is used to control the switch between controllers for the HS PHY."]
    pub usbphyc_misc: USBPHYC_MISC,
    _reserved2: [u8; 256usize],
    #[doc = "0x10c - This register is used to control the tune interface of the HS PHY, port #x."]
    pub usbphyc_tune1: USBPHYC_TUNE1,
    _reserved3: [u8; 252usize],
    #[doc = "0x20c - This register is used to control the tune interface of the HS PHY, port #x."]
    pub usbphyc_tune2: USBPHYC_TUNE2,
    _reserved4: [u8; 3564usize],
    #[doc = "0xffc - This register defines the version of this IP."]
    pub usbphyc_verr: USBPHYC_VERR,
}
#[doc = "This register is used to control the PLL of the HS PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_pll](usbphyc_pll) module"]
pub type USBPHYC_PLL = crate::Reg<u32, _USBPHYC_PLL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYC_PLL;
#[doc = "`read()` method returns [usbphyc_pll::R](usbphyc_pll::R) reader structure"]
impl crate::Readable for USBPHYC_PLL {}
#[doc = "`write(|w| ..)` method takes [usbphyc_pll::W](usbphyc_pll::W) writer structure"]
impl crate::Writable for USBPHYC_PLL {}
#[doc = "This register is used to control the PLL of the HS PHY."]
pub mod usbphyc_pll;
#[doc = "This register is used to control the switch between controllers for the HS PHY.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_misc](usbphyc_misc) module"]
pub type USBPHYC_MISC = crate::Reg<u32, _USBPHYC_MISC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYC_MISC;
#[doc = "`read()` method returns [usbphyc_misc::R](usbphyc_misc::R) reader structure"]
impl crate::Readable for USBPHYC_MISC {}
#[doc = "`write(|w| ..)` method takes [usbphyc_misc::W](usbphyc_misc::W) writer structure"]
impl crate::Writable for USBPHYC_MISC {}
#[doc = "This register is used to control the switch between controllers for the HS PHY."]
pub mod usbphyc_misc;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_tune1](usbphyc_tune1) module"]
pub type USBPHYC_TUNE1 = crate::Reg<u32, _USBPHYC_TUNE1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYC_TUNE1;
#[doc = "`read()` method returns [usbphyc_tune1::R](usbphyc_tune1::R) reader structure"]
impl crate::Readable for USBPHYC_TUNE1 {}
#[doc = "`write(|w| ..)` method takes [usbphyc_tune1::W](usbphyc_tune1::W) writer structure"]
impl crate::Writable for USBPHYC_TUNE1 {}
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune1;
#[doc = "This register is used to control the tune interface of the HS PHY, port #x.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_tune2](usbphyc_tune2) module"]
pub type USBPHYC_TUNE2 = crate::Reg<u32, _USBPHYC_TUNE2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYC_TUNE2;
#[doc = "`read()` method returns [usbphyc_tune2::R](usbphyc_tune2::R) reader structure"]
impl crate::Readable for USBPHYC_TUNE2 {}
#[doc = "`write(|w| ..)` method takes [usbphyc_tune2::W](usbphyc_tune2::W) writer structure"]
impl crate::Writable for USBPHYC_TUNE2 {}
#[doc = "This register is used to control the tune interface of the HS PHY, port #x."]
pub mod usbphyc_tune2;
#[doc = "This register defines the version of this IP.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbphyc_verr](usbphyc_verr) module"]
pub type USBPHYC_VERR = crate::Reg<u32, _USBPHYC_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _USBPHYC_VERR;
#[doc = "`read()` method returns [usbphyc_verr::R](usbphyc_verr::R) reader structure"]
impl crate::Readable for USBPHYC_VERR {}
#[doc = "This register defines the version of this IP."]
pub mod usbphyc_verr;
