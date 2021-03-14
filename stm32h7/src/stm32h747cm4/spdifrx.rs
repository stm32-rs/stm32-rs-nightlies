#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub cr: CR,
    #[doc = "0x04 - Interrupt mask register"]
    pub imr: IMR,
    #[doc = "0x08 - Status register"]
    pub sr: SR,
    #[doc = "0x0c - Interrupt Flag Clear register"]
    pub ifcr: IFCR,
    _reserved_4_dr_: [u8; 4usize],
    #[doc = "0x14 - Channel Status register"]
    pub csr: CSR,
    #[doc = "0x18 - Debug Information register"]
    pub dir: DIR,
    _reserved7: [u8; 984usize],
    #[doc = "0x3f4 - SPDIFRX version register"]
    pub verr: VERR,
    #[doc = "0x3f8 - SPDIFRX identification register"]
    pub idr: IDR,
    #[doc = "0x3fc - SPDIFRX size identification register"]
    pub sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_10(&self) -> &DR_10 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const DR_10) }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_10_mut(&self) -> &mut DR_10 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut DR_10) }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_01(&self) -> &DR_01 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const DR_01) }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_01_mut(&self) -> &mut DR_01 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut DR_01) }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_00(&self) -> &DR_00 {
        unsafe { &*(((self as *const Self) as *const u8).add(16usize) as *const DR_00) }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub fn dr_00_mut(&self) -> &mut DR_00 {
        unsafe { &mut *(((self as *const Self) as *mut u8).add(16usize) as *mut DR_00) }
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](cr) module"]
pub type CR = crate::Reg<u32, _CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR;
#[doc = "`read()` method returns [cr::R](cr::R) reader structure"]
impl crate::Readable for CR {}
#[doc = "`write(|w| ..)` method takes [cr::W](cr::W) writer structure"]
impl crate::Writable for CR {}
#[doc = "Control register"]
pub mod cr;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](imr) module"]
pub type IMR = crate::Reg<u32, _IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IMR;
#[doc = "`read()` method returns [imr::R](imr::R) reader structure"]
impl crate::Readable for IMR {}
#[doc = "`write(|w| ..)` method takes [imr::W](imr::W) writer structure"]
impl crate::Writable for IMR {}
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](sr) module"]
pub type SR = crate::Reg<u32, _SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SR;
#[doc = "`read()` method returns [sr::R](sr::R) reader structure"]
impl crate::Readable for SR {}
#[doc = "Status register"]
pub mod sr;
#[doc = "Interrupt Flag Clear register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifcr](ifcr) module"]
pub type IFCR = crate::Reg<u32, _IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IFCR;
#[doc = "`write(|w| ..)` method takes [ifcr::W](ifcr::W) writer structure"]
impl crate::Writable for IFCR {}
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_00](dr_00) module"]
pub type DR_00 = crate::Reg<u32, _DR_00>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_00;
#[doc = "`read()` method returns [dr_00::R](dr_00::R) reader structure"]
impl crate::Readable for DR_00 {}
#[doc = "Data input register"]
pub mod dr_00;
#[doc = "Channel Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "Debug Information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "Debug Information register"]
pub mod dir;
#[doc = "SPDIFRX version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [verr](verr) module"]
pub type VERR = crate::Reg<u32, _VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VERR;
#[doc = "`read()` method returns [verr::R](verr::R) reader structure"]
impl crate::Readable for VERR {}
#[doc = "SPDIFRX version register"]
pub mod verr;
#[doc = "SPDIFRX identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idr](idr) module"]
pub type IDR = crate::Reg<u32, _IDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IDR;
#[doc = "`read()` method returns [idr::R](idr::R) reader structure"]
impl crate::Readable for IDR {}
#[doc = "SPDIFRX identification register"]
pub mod idr;
#[doc = "SPDIFRX size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sidr](sidr) module"]
pub type SIDR = crate::Reg<u32, _SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIDR;
#[doc = "`read()` method returns [sidr::R](sidr::R) reader structure"]
impl crate::Readable for SIDR {}
#[doc = "SPDIFRX size identification register"]
pub mod sidr;
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_01](dr_01) module"]
pub type DR_01 = crate::Reg<u32, _DR_01>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_01;
#[doc = "`read()` method returns [dr_01::R](dr_01::R) reader structure"]
impl crate::Readable for DR_01 {}
#[doc = "Data input register"]
pub mod dr_01;
#[doc = "Data input register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_10](dr_10) module"]
pub type DR_10 = crate::Reg<u32, _DR_10>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DR_10;
#[doc = "`read()` method returns [dr_10::R](dr_10::R) reader structure"]
impl crate::Readable for DR_10 {}
#[doc = "Data input register"]
pub mod dr_10;
