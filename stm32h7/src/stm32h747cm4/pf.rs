#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Cache Level ID register"]
    pub clidr: CLIDR,
    #[doc = "0x04 - Cache Type register"]
    pub ctr: CTR,
    #[doc = "0x08 - Cache Size ID register"]
    pub ccsidr: CCSIDR,
}
#[doc = "Cache Level ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clidr](clidr) module"]
pub type CLIDR = crate::Reg<u32, _CLIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLIDR;
#[doc = "`read()` method returns [clidr::R](clidr::R) reader structure"]
impl crate::Readable for CLIDR {}
#[doc = "Cache Level ID register"]
pub mod clidr;
#[doc = "Cache Type register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctr](ctr) module"]
pub type CTR = crate::Reg<u32, _CTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CTR;
#[doc = "`read()` method returns [ctr::R](ctr::R) reader structure"]
impl crate::Readable for CTR {}
#[doc = "Cache Type register"]
pub mod ctr;
#[doc = "Cache Size ID register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccsidr](ccsidr) module"]
pub type CCSIDR = crate::Reg<u32, _CCSIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CCSIDR;
#[doc = "`read()` method returns [ccsidr::R](ccsidr::R) reader structure"]
impl crate::Readable for CCSIDR {}
#[doc = "Cache Size ID register"]
pub mod ccsidr;
