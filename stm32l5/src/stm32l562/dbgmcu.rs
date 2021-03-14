#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU_IDCODE"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU configuration register"]
    pub cr: CR,
    #[doc = "0x08 - Debug MCU APB1 freeze register1"]
    pub apb1lfzr: APB1LFZR,
    #[doc = "0x0c - Debug MCU APB1 freeze register 2"]
    pub apb1hfzr: APB1HFZR,
    #[doc = "0x10 - Debug MCU APB2 freeze register"]
    pub apb2fzr: APB2FZR,
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
#[doc = "Debug MCU APB1 freeze register1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfzr](apb1lfzr) module"]
pub type APB1LFZR = crate::Reg<u32, _APB1LFZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LFZR;
#[doc = "`read()` method returns [apb1lfzr::R](apb1lfzr::R) reader structure"]
impl crate::Readable for APB1LFZR {}
#[doc = "`write(|w| ..)` method takes [apb1lfzr::W](apb1lfzr::W) writer structure"]
impl crate::Writable for APB1LFZR {}
#[doc = "Debug MCU APB1 freeze register1"]
pub mod apb1lfzr;
#[doc = "Debug MCU APB1 freeze register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1hfzr](apb1hfzr) module"]
pub type APB1HFZR = crate::Reg<u32, _APB1HFZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1HFZR;
#[doc = "`read()` method returns [apb1hfzr::R](apb1hfzr::R) reader structure"]
impl crate::Readable for APB1HFZR {}
#[doc = "`write(|w| ..)` method takes [apb1hfzr::W](apb1hfzr::W) writer structure"]
impl crate::Writable for APB1HFZR {}
#[doc = "Debug MCU APB1 freeze register 2"]
pub mod apb1hfzr;
#[doc = "Debug MCU APB2 freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fzr](apb2fzr) module"]
pub type APB2FZR = crate::Reg<u32, _APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZR;
#[doc = "`read()` method returns [apb2fzr::R](apb2fzr::R) reader structure"]
impl crate::Readable for APB2FZR {}
#[doc = "`write(|w| ..)` method takes [apb2fzr::W](apb2fzr::W) writer structure"]
impl crate::Writable for APB2FZR {}
#[doc = "Debug MCU APB2 freeze register"]
pub mod apb2fzr;
