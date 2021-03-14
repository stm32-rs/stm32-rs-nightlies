#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved_0_dr: [u8; 4usize],
    #[doc = "0x04 - Independent Data register"]
    pub idr: IDR,
    #[doc = "0x08 - Control register"]
    pub cr: CR,
    #[doc = "0x0c - Initial CRC value"]
    pub init: INIT,
    #[doc = "0x10 - CRC polynomial"]
    pub pol: POL,
}
impl RegisterBlock {
    #[doc = "0x00 - Data register - half-word sized"]
    #[inline(always)]
    pub fn dr16(&self) -> &DR16 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR16) }
    }
    #[doc = "0x00 - Data register - half-word sized"]
    #[inline(always)]
    pub fn dr16_mut(&self) -> &mut DR16 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR16) }
    }
    #[doc = "0x00 - Data register - byte sized"]
    #[inline(always)]
    pub fn dr8(&self) -> &DR8 {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR8) }
    }
    #[doc = "0x00 - Data register - byte sized"]
    #[inline(always)]
    pub fn dr8_mut(&self) -> &mut DR8 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR8) }
    }
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub fn dr(&self) -> &DR {
        unsafe { &*(((self as *const Self) as *const u8).add(0usize) as *const DR) }
    }
    #[doc = "0x00 - Data register"]
    #[inline(always)]
    pub fn dr_mut(&self) -> &mut DR {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(0usize) as *mut DR) }
    }
}
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr](dr) module"]
pub type DR = crate::Reg<u32, _DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR;
#[doc = "`read()` method returns [dr::R](dr::R) reader structure"]
impl crate::Readable for DR {}
#[doc = "`write(|w| ..)` method takes [dr::W](dr::W) writer structure"]
impl crate::Writable for DR {}
#[doc = "Data register"]
pub mod dr;
#[doc = "Independent Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "`write(|w| ..)` method takes [idr::W](idr::W) writer structure"]
impl crate::Writable for IDR {}
#[doc = "Independent Data register"]
pub mod idr;
#[doc = "Control register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control register"]
pub mod cr;
#[doc = "Initial CRC value\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [init](init) module"]
pub type INIT = crate::Reg<u32, _INIT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _INIT;
#[doc = "`read()` method returns [init::R](init::R) reader structure"]
impl crate::Readable for INIT {}
#[doc = "`write(|w| ..)` method takes [init::W](init::W) writer structure"]
impl crate::Writable for INIT {}
#[doc = "Initial CRC value"]
pub mod init;
#[doc = "CRC polynomial\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pol](pol) module"]
pub type POL = crate::Reg<u32, _POL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _POL;
#[doc = "`read()` method returns [pol::R](pol::R) reader structure"]
impl crate::Readable for POL {}
#[doc = "`write(|w| ..)` method takes [pol::W](pol::W) writer structure"]
impl crate::Writable for POL {}
#[doc = "CRC polynomial"]
pub mod pol;
#[doc = "Data register - byte sized\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr8](dr8) module"]
pub type DR8 = crate::Reg<u8, _DR8>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR8;
#[doc = "`read()` method returns [dr8::R](dr8::R) reader structure"]
impl crate::Readable for DR8 {}
#[doc = "`write(|w| ..)` method takes [dr8::W](dr8::W) writer structure"]
impl crate::Writable for DR8 {}
#[doc = "Data register - byte sized"]
pub mod dr8;
#[doc = "Data register - half-word sized\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr16](dr16) module"]
pub type DR16 = crate::Reg<u16, _DR16>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR16;
#[doc = "`read()` method returns [dr16::R](dr16::R) reader structure"]
impl crate::Readable for DR16 {}
#[doc = "`write(|w| ..)` method takes [dr16::W](dr16::W) writer structure"]
impl crate::Writable for DR16 {}
#[doc = "Data register - half-word sized"]
pub mod dr16;
