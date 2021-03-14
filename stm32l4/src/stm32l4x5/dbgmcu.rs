#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    #[doc = "0x08 - APB Low Freeze Register 1"]
    pub apb1_fzr1: APB1_FZR1,
    #[doc = "0x0c - APB Low Freeze Register 2"]
    pub apb1_fzr2: APB1_FZR2,
    #[doc = "0x10 - APB High Freeze Register"]
    pub apb2_fzr: APB2_FZR,
}
#[doc = "MCU Device ID Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcode](idcode) module"]
pub type IDCODE = crate::Reg<u32, _IDCODE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODE;
#[doc = "`read()` method returns [idcode::R](idcode::R) reader structure"]
impl crate::Readable for IDCODE {}
#[doc = "MCU Device ID Code Register"]
pub mod idcode;
#[doc = "Debug MCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Debug MCU Configuration Register"]
pub mod cr;
#[doc = "APB Low Freeze Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_fzr1](apb1_fzr1) module"]
pub type APB1_FZR1 = crate::Reg<u32, _APB1_FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1_FZR1;
#[doc = "`read()` method returns [apb1_fzr1::R](apb1_fzr1::R) reader structure"]
impl crate::Readable for APB1_FZR1 {}
#[doc = "`write(|w| ..)` method takes [apb1_fzr1::W](apb1_fzr1::W) writer structure"]
impl crate::Writable for APB1_FZR1 {}
#[doc = "APB Low Freeze Register 1"]
pub mod apb1_fzr1;
#[doc = "APB Low Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1_fzr2](apb1_fzr2) module"]
pub type APB1_FZR2 = crate::Reg<u32, _APB1_FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1_FZR2;
#[doc = "`read()` method returns [apb1_fzr2::R](apb1_fzr2::R) reader structure"]
impl crate::Readable for APB1_FZR2 {}
#[doc = "`write(|w| ..)` method takes [apb1_fzr2::W](apb1_fzr2::W) writer structure"]
impl crate::Writable for APB1_FZR2 {}
#[doc = "APB Low Freeze Register 2"]
pub mod apb1_fzr2;
#[doc = "APB High Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2_fzr](apb2_fzr) module"]
pub type APB2_FZR = crate::Reg<u32, _APB2_FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2_FZR;
#[doc = "`read()` method returns [apb2_fzr::R](apb2_fzr::R) reader structure"]
impl crate::Readable for APB2_FZR {}
#[doc = "`write(|w| ..)` method takes [apb2_fzr::W](apb2_fzr::W) writer structure"]
impl crate::Writable for APB2_FZR {}
#[doc = "APB High Freeze Register"]
pub mod apb2_fzr;
