#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control register"]
    pub spdifrx_cr: SPDIFRX_CR,
    #[doc = "0x04 - Interrupt mask register"]
    pub spdifrx_imr: SPDIFRX_IMR,
    #[doc = "0x08 - Status register"]
    pub spdifrx_sr: SPDIFRX_SR,
    #[doc = "0x0c - Interrupt flag clear register"]
    pub spdifrx_ifcr: SPDIFRX_IFCR,
    #[doc = "0x10 - This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
    pub spdifrx_fmt0_dr: SPDIFRX_FMT0_DR,
    #[doc = "0x14 - Channel status register"]
    pub spdifrx_csr: SPDIFRX_CSR,
    #[doc = "0x18 - Debug information register"]
    pub spdifrx_dir: SPDIFRX_DIR,
    _reserved7: [u8; 984usize],
    #[doc = "0x3f4 - SPDIFRX version register"]
    pub spdifrx_verr: SPDIFRX_VERR,
    #[doc = "0x3f8 - SPDIFRX identification register"]
    pub spdifrx_ipidr: SPDIFRX_IPIDR,
    #[doc = "0x3fc - SPDIFRX size identification register"]
    pub spdifrx_sidr: SPDIFRX_SIDR,
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_cr](spdifrx_cr) module"]
pub type SPDIFRX_CR = crate::Reg<u32, _SPDIFRX_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_CR;
#[doc = "`read()` method returns [spdifrx_cr::R](spdifrx_cr::R) reader structure"]
impl crate::Readable for SPDIFRX_CR {}
#[doc = "`write(|w| ..)` method takes [spdifrx_cr::W](spdifrx_cr::W) writer structure"]
impl crate::Writable for SPDIFRX_CR {}
#[doc = "Control register"]
pub mod spdifrx_cr;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_imr](spdifrx_imr) module"]
pub type SPDIFRX_IMR = crate::Reg<u32, _SPDIFRX_IMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_IMR;
#[doc = "`read()` method returns [spdifrx_imr::R](spdifrx_imr::R) reader structure"]
impl crate::Readable for SPDIFRX_IMR {}
#[doc = "`write(|w| ..)` method takes [spdifrx_imr::W](spdifrx_imr::W) writer structure"]
impl crate::Writable for SPDIFRX_IMR {}
#[doc = "Interrupt mask register"]
pub mod spdifrx_imr;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_sr](spdifrx_sr) module"]
pub type SPDIFRX_SR = crate::Reg<u32, _SPDIFRX_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_SR;
#[doc = "`read()` method returns [spdifrx_sr::R](spdifrx_sr::R) reader structure"]
impl crate::Readable for SPDIFRX_SR {}
#[doc = "Status register"]
pub mod spdifrx_sr;
#[doc = "Interrupt flag clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_ifcr](spdifrx_ifcr) module"]
pub type SPDIFRX_IFCR = crate::Reg<u32, _SPDIFRX_IFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_IFCR;
#[doc = "`read()` method returns [spdifrx_ifcr::R](spdifrx_ifcr::R) reader structure"]
impl crate::Readable for SPDIFRX_IFCR {}
#[doc = "`write(|w| ..)` method takes [spdifrx_ifcr::W](spdifrx_ifcr::W) writer structure"]
impl crate::Writable for SPDIFRX_IFCR {}
#[doc = "Interrupt flag clear register"]
pub mod spdifrx_ifcr;
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_fmt0_dr](spdifrx_fmt0_dr) module"]
pub type SPDIFRX_FMT0_DR = crate::Reg<u32, _SPDIFRX_FMT0_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_FMT0_DR;
#[doc = "`read()` method returns [spdifrx_fmt0_dr::R](spdifrx_fmt0_dr::R) reader structure"]
impl crate::Readable for SPDIFRX_FMT0_DR {}
#[doc = "This register can take 3 different formats according to DRFMT. Here is the format when DRFMT = 00:"]
pub mod spdifrx_fmt0_dr;
#[doc = "Channel status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_csr](spdifrx_csr) module"]
pub type SPDIFRX_CSR = crate::Reg<u32, _SPDIFRX_CSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_CSR;
#[doc = "`read()` method returns [spdifrx_csr::R](spdifrx_csr::R) reader structure"]
impl crate::Readable for SPDIFRX_CSR {}
#[doc = "Channel status register"]
pub mod spdifrx_csr;
#[doc = "Debug information register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_dir](spdifrx_dir) module"]
pub type SPDIFRX_DIR = crate::Reg<u32, _SPDIFRX_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_DIR;
#[doc = "`read()` method returns [spdifrx_dir::R](spdifrx_dir::R) reader structure"]
impl crate::Readable for SPDIFRX_DIR {}
#[doc = "Debug information register"]
pub mod spdifrx_dir;
#[doc = "SPDIFRX version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_verr](spdifrx_verr) module"]
pub type SPDIFRX_VERR = crate::Reg<u32, _SPDIFRX_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_VERR;
#[doc = "`read()` method returns [spdifrx_verr::R](spdifrx_verr::R) reader structure"]
impl crate::Readable for SPDIFRX_VERR {}
#[doc = "SPDIFRX version register"]
pub mod spdifrx_verr;
#[doc = "SPDIFRX identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_ipidr](spdifrx_ipidr) module"]
pub type SPDIFRX_IPIDR = crate::Reg<u32, _SPDIFRX_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_IPIDR;
#[doc = "`read()` method returns [spdifrx_ipidr::R](spdifrx_ipidr::R) reader structure"]
impl crate::Readable for SPDIFRX_IPIDR {}
#[doc = "SPDIFRX identification register"]
pub mod spdifrx_ipidr;
#[doc = "SPDIFRX size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spdifrx_sidr](spdifrx_sidr) module"]
pub type SPDIFRX_SIDR = crate::Reg<u32, _SPDIFRX_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SPDIFRX_SIDR;
#[doc = "`read()` method returns [spdifrx_sidr::R](spdifrx_sidr::R) reader structure"]
impl crate::Readable for SPDIFRX_SIDR {}
#[doc = "SPDIFRX size identification register"]
pub mod spdifrx_sidr;
