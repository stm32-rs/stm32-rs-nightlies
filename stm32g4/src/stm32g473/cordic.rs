#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - CORDIC Control Status register"]
    pub csr: CSR,
    #[doc = "0x04 - CORDIC argument register"]
    pub wdata: WDATA,
    #[doc = "0x08 - CORDIC result register"]
    pub rdata: RDATA,
}
#[doc = "CORDIC Control Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](csr) module"]
pub type CSR = crate::Reg<u32, _CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSR;
#[doc = "`read()` method returns [csr::R](csr::R) reader structure"]
impl crate::Readable for CSR {}
#[doc = "`write(|w| ..)` method takes [csr::W](csr::W) writer structure"]
impl crate::Writable for CSR {}
#[doc = "CORDIC Control Status register"]
pub mod csr;
#[doc = "CORDIC argument register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdata](wdata) module"]
pub type WDATA = crate::Reg<u32, _WDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _WDATA;
#[doc = "`read()` method returns [wdata::R](wdata::R) reader structure"]
impl crate::Readable for WDATA {}
#[doc = "`write(|w| ..)` method takes [wdata::W](wdata::W) writer structure"]
impl crate::Writable for WDATA {}
#[doc = "CORDIC argument register"]
pub mod wdata;
#[doc = "CORDIC result register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rdata](rdata) module"]
pub type RDATA = crate::Reg<u32, _RDATA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RDATA;
#[doc = "`read()` method returns [rdata::R](rdata::R) reader structure"]
impl crate::Readable for RDATA {}
#[doc = "CORDIC result register"]
pub mod rdata;
