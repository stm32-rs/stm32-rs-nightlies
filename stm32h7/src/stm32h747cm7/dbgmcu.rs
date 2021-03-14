#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Identity code"]
    pub idc: IDC,
    #[doc = "0x04 - Configuration register"]
    pub cr: CR,
    _reserved2: [u8; 44usize],
    #[doc = "0x34 - APB3 peripheral freeze register"]
    pub apb3fz1: APB3FZ1,
    #[doc = "0x38 - APB3 peripheral freeze register CPU2"]
    pub apb3fz2: APB3FZ2,
    #[doc = "0x3c - APB1L peripheral freeze register"]
    pub apb1lfz1: APB1LFZ1,
    #[doc = "0x40 - APB1L peripheral freeze register CPU2"]
    pub apb1lfz2: APB1LFZ2,
    _reserved6: [u8; 4usize],
    #[doc = "0x48 - APB2 peripheral freeze register CPU2"]
    pub apb2fz2: APB2FZ2,
    #[doc = "0x4c - APB2 peripheral freeze register"]
    pub apb2fz1: APB2FZ1,
    _reserved8: [u8; 4usize],
    #[doc = "0x54 - APB4 peripheral freeze register"]
    pub apb4fz1: APB4FZ1,
    #[doc = "0x58 - APB4 peripheral freeze register CPU2"]
    pub apb4fz2: APB4FZ2,
}
#[doc = "Identity code\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idc](idc) module"]
pub type IDC = crate::Reg<u32, _IDC>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDC;
#[doc = "`read()` method returns [idc::R](idc::R) reader structure"]
impl crate::Readable for IDC {}
#[doc = "Identity code"]
pub mod idc;
#[doc = "Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Configuration register"]
pub mod cr;
#[doc = "APB3 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3fz1](apb3fz1) module"]
pub type APB3FZ1 = crate::Reg<u32, _APB3FZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3FZ1;
#[doc = "`read()` method returns [apb3fz1::R](apb3fz1::R) reader structure"]
impl crate::Readable for APB3FZ1 {}
#[doc = "`write(|w| ..)` method takes [apb3fz1::W](apb3fz1::W) writer structure"]
impl crate::Writable for APB3FZ1 {}
#[doc = "APB3 peripheral freeze register"]
pub mod apb3fz1;
#[doc = "APB1L peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz1](apb1lfz1) module"]
pub type APB1LFZ1 = crate::Reg<u32, _APB1LFZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LFZ1;
#[doc = "`read()` method returns [apb1lfz1::R](apb1lfz1::R) reader structure"]
impl crate::Readable for APB1LFZ1 {}
#[doc = "`write(|w| ..)` method takes [apb1lfz1::W](apb1lfz1::W) writer structure"]
impl crate::Writable for APB1LFZ1 {}
#[doc = "APB1L peripheral freeze register"]
pub mod apb1lfz1;
#[doc = "APB2 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz1](apb2fz1) module"]
pub type APB2FZ1 = crate::Reg<u32, _APB2FZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZ1;
#[doc = "`read()` method returns [apb2fz1::R](apb2fz1::R) reader structure"]
impl crate::Readable for APB2FZ1 {}
#[doc = "`write(|w| ..)` method takes [apb2fz1::W](apb2fz1::W) writer structure"]
impl crate::Writable for APB2FZ1 {}
#[doc = "APB2 peripheral freeze register"]
pub mod apb2fz1;
#[doc = "APB4 peripheral freeze register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4fz1](apb4fz1) module"]
pub type APB4FZ1 = crate::Reg<u32, _APB4FZ1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB4FZ1;
#[doc = "`read()` method returns [apb4fz1::R](apb4fz1::R) reader structure"]
impl crate::Readable for APB4FZ1 {}
#[doc = "`write(|w| ..)` method takes [apb4fz1::W](apb4fz1::W) writer structure"]
impl crate::Writable for APB4FZ1 {}
#[doc = "APB4 peripheral freeze register"]
pub mod apb4fz1;
#[doc = "APB3 peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb3fz2](apb3fz2) module"]
pub type APB3FZ2 = crate::Reg<u32, _APB3FZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB3FZ2;
#[doc = "`read()` method returns [apb3fz2::R](apb3fz2::R) reader structure"]
impl crate::Readable for APB3FZ2 {}
#[doc = "`write(|w| ..)` method takes [apb3fz2::W](apb3fz2::W) writer structure"]
impl crate::Writable for APB3FZ2 {}
#[doc = "APB3 peripheral freeze register CPU2"]
pub mod apb3fz2;
#[doc = "APB1L peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1lfz2](apb1lfz2) module"]
pub type APB1LFZ2 = crate::Reg<u32, _APB1LFZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1LFZ2;
#[doc = "`read()` method returns [apb1lfz2::R](apb1lfz2::R) reader structure"]
impl crate::Readable for APB1LFZ2 {}
#[doc = "`write(|w| ..)` method takes [apb1lfz2::W](apb1lfz2::W) writer structure"]
impl crate::Writable for APB1LFZ2 {}
#[doc = "APB1L peripheral freeze register CPU2"]
pub mod apb1lfz2;
#[doc = "APB2 peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fz2](apb2fz2) module"]
pub type APB2FZ2 = crate::Reg<u32, _APB2FZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZ2;
#[doc = "`read()` method returns [apb2fz2::R](apb2fz2::R) reader structure"]
impl crate::Readable for APB2FZ2 {}
#[doc = "`write(|w| ..)` method takes [apb2fz2::W](apb2fz2::W) writer structure"]
impl crate::Writable for APB2FZ2 {}
#[doc = "APB2 peripheral freeze register CPU2"]
pub mod apb2fz2;
#[doc = "APB4 peripheral freeze register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb4fz2](apb4fz2) module"]
pub type APB4FZ2 = crate::Reg<u32, _APB4FZ2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB4FZ2;
#[doc = "`read()` method returns [apb4fz2::R](apb4fz2::R) reader structure"]
impl crate::Readable for APB4FZ2 {}
#[doc = "`write(|w| ..)` method takes [apb4fz2::W](apb4fz2::W) writer structure"]
impl crate::Writable for APB4FZ2 {}
#[doc = "APB4 peripheral freeze register CPU2"]
pub mod apb4fz2;
