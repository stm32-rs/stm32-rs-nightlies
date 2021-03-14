#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: CR,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb_fz1: APB_FZ1,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb_fz2: APB_FZ2,
}
#[doc = "DBGMCU_IDCODE\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcode](idcode) module"]
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
#[doc = "`read()` method returns [idcode::R](idcode::R) reader structure"]
impl crate::Readable for IDCODE {}
#[doc = "DBGMCU_IDCODE"]
pub mod idcode;
#[doc = "Debug MCU configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Debug MCU configuration register"]
pub mod cr;
#[doc = "Debug MCU APB1 freeze register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_fz1](apb_fz1) module"]
pub type APB_FZ1 = crate::Reg<u32, _APB_FZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_FZ1;
#[doc = "`read()` method returns [apb_fz1::R](apb_fz1::R) reader structure"]
impl crate::Readable for APB_FZ1 {}
#[doc = "`write(|w| ..)` method takes [apb_fz1::W](apb_fz1::W) writer structure"]
impl crate::Writable for APB_FZ1 {}
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb_fz1;
#[doc = "Debug MCU APB1 freeze register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_fz2](apb_fz2) module"]
pub type APB_FZ2 = crate::Reg<u32, _APB_FZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB_FZ2;
#[doc = "`read()` method returns [apb_fz2::R](apb_fz2::R) reader structure"]
impl crate::Readable for APB_FZ2 {}
#[doc = "`write(|w| ..)` method takes [apb_fz2::W](apb_fz2::W) writer structure"]
impl crate::Writable for APB_FZ2 {}
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb_fz2;
