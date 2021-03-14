#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - MCU Device ID Code Register"]
    pub idcode: IDCODE,
    #[doc = "0x04 - Debug MCU Configuration Register"]
    pub cr: CR,
    _reserved2: [u8; 52usize],
    #[doc = "0x3c - APB1 Low Freeze Register CPU1"]
    pub apb1fzr1: APB1FZR1,
    #[doc = "0x40 - APB1 Low Freeze Register CPU2"]
    pub c2ap_b1fzr1: C2AP_B1FZR1,
    #[doc = "0x44 - APB1 High Freeze Register CPU1"]
    pub apb1fzr2: APB1FZR2,
    _reserved_5_c2apb: [u8; 4usize],
    #[doc = "0x4c - APB2 Freeze Register CPU1"]
    pub apb2fzr: APB2FZR,
}
impl RegisterBlock {
    #[doc = "0x48 - APB2 Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb2fzr(&self) -> &C2APB2FZR {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const C2APB2FZR) }
    }
    #[doc = "0x48 - APB2 Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb2fzr_mut(&self) -> &mut C2APB2FZR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut C2APB2FZR) }
    }
    #[doc = "0x48 - APB1 High Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb1fzr2(&self) -> &C2APB1FZR2 {
        unsafe { &*(((self as *const Self) as *const u8).add(72usize) as *const C2APB1FZR2) }
    }
    #[doc = "0x48 - APB1 High Freeze Register CPU2"]
    #[inline(always)]
    pub fn c2apb1fzr2_mut(&self) -> &mut C2APB1FZR2 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(72usize) as *mut C2APB1FZR2) }
    }
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
#[doc = "APB1 Low Freeze Register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr1](apb1fzr1) module"]
pub type APB1FZR1 = crate::Reg<u32, _APB1FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR1;
#[doc = "`read()` method returns [apb1fzr1::R](apb1fzr1::R) reader structure"]
impl crate::Readable for APB1FZR1 {}
#[doc = "`write(|w| ..)` method takes [apb1fzr1::W](apb1fzr1::W) writer structure"]
impl crate::Writable for APB1FZR1 {}
#[doc = "APB1 Low Freeze Register CPU1"]
pub mod apb1fzr1;
#[doc = "APB1 Low Freeze Register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2ap_b1fzr1](c2ap_b1fzr1) module"]
pub type C2AP_B1FZR1 = crate::Reg<u32, _C2AP_B1FZR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2AP_B1FZR1;
#[doc = "`read()` method returns [c2ap_b1fzr1::R](c2ap_b1fzr1::R) reader structure"]
impl crate::Readable for C2AP_B1FZR1 {}
#[doc = "`write(|w| ..)` method takes [c2ap_b1fzr1::W](c2ap_b1fzr1::W) writer structure"]
impl crate::Writable for C2AP_B1FZR1 {}
#[doc = "APB1 Low Freeze Register CPU2"]
pub mod c2ap_b1fzr1;
#[doc = "APB1 High Freeze Register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb1fzr2](apb1fzr2) module"]
pub type APB1FZR2 = crate::Reg<u32, _APB1FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB1FZR2;
#[doc = "`read()` method returns [apb1fzr2::R](apb1fzr2::R) reader structure"]
impl crate::Readable for APB1FZR2 {}
#[doc = "`write(|w| ..)` method takes [apb1fzr2::W](apb1fzr2::W) writer structure"]
impl crate::Writable for APB1FZR2 {}
#[doc = "APB1 High Freeze Register CPU1"]
pub mod apb1fzr2;
#[doc = "APB1 High Freeze Register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb1fzr2](c2apb1fzr2) module"]
pub type C2APB1FZR2 = crate::Reg<u32, _C2APB1FZR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB1FZR2;
#[doc = "`read()` method returns [c2apb1fzr2::R](c2apb1fzr2::R) reader structure"]
impl crate::Readable for C2APB1FZR2 {}
#[doc = "`write(|w| ..)` method takes [c2apb1fzr2::W](c2apb1fzr2::W) writer structure"]
impl crate::Writable for C2APB1FZR2 {}
#[doc = "APB1 High Freeze Register CPU2"]
pub mod c2apb1fzr2;
#[doc = "APB2 Freeze Register CPU1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb2fzr](apb2fzr) module"]
pub type APB2FZR = crate::Reg<u32, _APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _APB2FZR;
#[doc = "`read()` method returns [apb2fzr::R](apb2fzr::R) reader structure"]
impl crate::Readable for APB2FZR {}
#[doc = "`write(|w| ..)` method takes [apb2fzr::W](apb2fzr::W) writer structure"]
impl crate::Writable for APB2FZR {}
#[doc = "APB2 Freeze Register CPU1"]
pub mod apb2fzr;
#[doc = "APB2 Freeze Register CPU2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c2apb2fzr](c2apb2fzr) module"]
pub type C2APB2FZR = crate::Reg<u32, _C2APB2FZR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _C2APB2FZR;
#[doc = "`read()` method returns [c2apb2fzr::R](c2apb2fzr::R) reader structure"]
impl crate::Readable for C2APB2FZR {}
#[doc = "`write(|w| ..)` method takes [c2apb2fzr::W](c2apb2fzr::W) writer structure"]
impl crate::Writable for C2APB2FZR {}
#[doc = "APB2 Freeze Register CPU2"]
pub mod c2apb2fzr;
