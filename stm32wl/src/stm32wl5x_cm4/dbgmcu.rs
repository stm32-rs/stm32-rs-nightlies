#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DBGMCU Identity Code Register"]
    pub idcoder: IDCODER,
    #[doc = "0x04 - DBGMCU Configuration Register"]
    pub cr: CR,
    _reserved2: [u8; 52usize],
    #[doc = "0x3c - DBGMCU CPU1 APB1 Peripheral Freeze Register 1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x40 - DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device"]
    pub c2apb1fzr1: C2APB1FZR1,
    #[doc = "0x44 - DBGMCU CPU1 APB1 Peripheral Freeze Register 2"]
    pub apb1fzr2: APB1FZR2,
    #[doc = "0x48 - DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device"]
    pub c2apb1fzr2: C2APB1FZR2,
    #[doc = "0x4c - DBGMCU CPU1 APB2 Peripheral Freeze Register"]
    pub apb2fzr: APB2FZR,
    #[doc = "0x50 - DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device"]
    pub c2apb2fzr: C2APB2FZR,
}
#[doc = "DBGMCU Identity Code Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idcoder](idcoder) module"]
pub type IDCODER = crate::Reg<u32, _IDCODER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDCODER;
#[doc = "`read()` method returns [idcoder::R](idcoder::R) reader structure"]
impl crate::Readable for IDCODER {}
#[doc = "DBGMCU Identity Code Register"]
pub mod idcoder;
#[doc = "DBGMCU Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "DBGMCU Configuration Register"]
pub mod cr;
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr1](apb1fzr1) module"]
pub type APB1FZR1 = crate::Reg<u32, _APB1FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR1;
#[doc = "`read()` method returns [apb1fzr1::R](apb1fzr1::R) reader structure"]
impl crate::Readable for APB1FZR1 {}
#[doc = "`write(|w| ..)` method takes [apb1fzr1::W](apb1fzr1::W) writer structure"]
impl crate::Writable for APB1FZR1 {}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 1"]
pub mod apb1fzr1;
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1fzr1](c2apb1fzr1) module"]
pub type C2APB1FZR1 = crate::Reg<u32, _C2APB1FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1FZR1;
#[doc = "`read()` method returns [c2apb1fzr1::R](c2apb1fzr1::R) reader structure"]
impl crate::Readable for C2APB1FZR1 {}
#[doc = "`write(|w| ..)` method takes [c2apb1fzr1::W](c2apb1fzr1::W) writer structure"]
impl crate::Writable for C2APB1FZR1 {}
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 1 \\[dual core device"]
pub mod c2apb1fzr1;
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr2](apb1fzr2) module"]
pub type APB1FZR2 = crate::Reg<u32, _APB1FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR2;
#[doc = "`read()` method returns [apb1fzr2::R](apb1fzr2::R) reader structure"]
impl crate::Readable for APB1FZR2 {}
#[doc = "`write(|w| ..)` method takes [apb1fzr2::W](apb1fzr2::W) writer structure"]
impl crate::Writable for APB1FZR2 {}
#[doc = "DBGMCU CPU1 APB1 Peripheral Freeze Register 2"]
pub mod apb1fzr2;
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1fzr2](c2apb1fzr2) module"]
pub type C2APB1FZR2 = crate::Reg<u32, _C2APB1FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1FZR2;
#[doc = "`read()` method returns [c2apb1fzr2::R](c2apb1fzr2::R) reader structure"]
impl crate::Readable for C2APB1FZR2 {}
#[doc = "`write(|w| ..)` method takes [c2apb1fzr2::W](c2apb1fzr2::W) writer structure"]
impl crate::Writable for C2APB1FZR2 {}
#[doc = "DBGMCU CPU2 APB1 Peripheral Freeze Register 2 \\[dual core device"]
pub mod c2apb1fzr2;
#[doc = "DBGMCU CPU1 APB2 Peripheral Freeze Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fzr](apb2fzr) module"]
pub type APB2FZR = crate::Reg<u32, _APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZR;
#[doc = "`read()` method returns [apb2fzr::R](apb2fzr::R) reader structure"]
impl crate::Readable for APB2FZR {}
#[doc = "`write(|w| ..)` method takes [apb2fzr::W](apb2fzr::W) writer structure"]
impl crate::Writable for APB2FZR {}
#[doc = "DBGMCU CPU1 APB2 Peripheral Freeze Register"]
pub mod apb2fzr;
#[doc = "DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb2fzr](c2apb2fzr) module"]
pub type C2APB2FZR = crate::Reg<u32, _C2APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB2FZR;
#[doc = "`read()` method returns [c2apb2fzr::R](c2apb2fzr::R) reader structure"]
impl crate::Readable for C2APB2FZR {}
#[doc = "`write(|w| ..)` method takes [c2apb2fzr::W](c2apb2fzr::W) writer structure"]
impl crate::Writable for C2APB2FZR {}
#[doc = "DBGMCU CPU2 APB2 Peripheral Freeze Register \\[dual core device"]
pub mod c2apb2fzr;
