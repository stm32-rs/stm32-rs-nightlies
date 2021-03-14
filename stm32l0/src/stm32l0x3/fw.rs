#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Code segment start address"]
    pub cssa: CSSA,
    #[doc = "0x04 - Code segment length"]
    pub csl: CSL,
    #[doc = "0x08 - Non-volatile data segment start address"]
    pub nvdssa: NVDSSA,
    #[doc = "0x0c - Non-volatile data segment length"]
    pub nvdsl: NVDSL,
    #[doc = "0x10 - Volatile data segment start address"]
    pub vdssa: VDSSA,
    #[doc = "0x14 - Volatile data segment length"]
    pub vdsl: VDSL,
    _reserved6: [u8; 8usize],
    #[doc = "0x20 - Configuration register"]
    pub cr: CR,
}
#[doc = "Code segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cssa](cssa) module"]
pub type CSSA = crate::Reg<u32, _CSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSSA;
#[doc = "`read()` method returns [cssa::R](cssa::R) reader structure"]
impl crate::Readable for CSSA {}
#[doc = "`write(|w| ..)` method takes [cssa::W](cssa::W) writer structure"]
impl crate::Writable for CSSA {}
#[doc = "Code segment start address"]
pub mod cssa;
#[doc = "Code segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csl](csl) module"]
pub type CSL = crate::Reg<u32, _CSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CSL;
#[doc = "`read()` method returns [csl::R](csl::R) reader structure"]
impl crate::Readable for CSL {}
#[doc = "`write(|w| ..)` method takes [csl::W](csl::W) writer structure"]
impl crate::Writable for CSL {}
#[doc = "Code segment length"]
pub mod csl;
#[doc = "Non-volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvdssa](nvdssa) module"]
pub type NVDSSA = crate::Reg<u32, _NVDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVDSSA;
#[doc = "`read()` method returns [nvdssa::R](nvdssa::R) reader structure"]
impl crate::Readable for NVDSSA {}
#[doc = "`write(|w| ..)` method takes [nvdssa::W](nvdssa::W) writer structure"]
impl crate::Writable for NVDSSA {}
#[doc = "Non-volatile data segment start address"]
pub mod nvdssa;
#[doc = "Non-volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvdsl](nvdsl) module"]
pub type NVDSL = crate::Reg<u32, _NVDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NVDSL;
#[doc = "`read()` method returns [nvdsl::R](nvdsl::R) reader structure"]
impl crate::Readable for NVDSL {}
#[doc = "`write(|w| ..)` method takes [nvdsl::W](nvdsl::W) writer structure"]
impl crate::Writable for NVDSL {}
#[doc = "Non-volatile data segment length"]
pub mod nvdsl;
#[doc = "Volatile data segment start address\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdssa](vdssa) module"]
pub type VDSSA = crate::Reg<u32, _VDSSA>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDSSA;
#[doc = "`read()` method returns [vdssa::R](vdssa::R) reader structure"]
impl crate::Readable for VDSSA {}
#[doc = "`write(|w| ..)` method takes [vdssa::W](vdssa::W) writer structure"]
impl crate::Writable for VDSSA {}
#[doc = "Volatile data segment start address"]
pub mod vdssa;
#[doc = "Volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vdsl](vdsl) module"]
pub type VDSL = crate::Reg<u32, _VDSL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _VDSL;
#[doc = "`read()` method returns [vdsl::R](vdsl::R) reader structure"]
impl crate::Readable for VDSL {}
#[doc = "`write(|w| ..)` method takes [vdsl::W](vdsl::W) writer structure"]
impl crate::Writable for VDSL {}
#[doc = "Volatile data segment length"]
pub mod vdsl;
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
