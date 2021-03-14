#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICC control register"]
    pub gicc_ctlr: GICC_CTLR,
    #[doc = "0x04 - GICC input priority mask register"]
    pub gicc_pmr: GICC_PMR,
    #[doc = "0x08 - GICC binary point register"]
    pub gicc_bpr: GICC_BPR,
    #[doc = "0x0c - GICC interrupt acknowledge register"]
    pub gicc_iar: GICC_IAR,
    #[doc = "0x10 - GICC end of interrupt register"]
    pub gicc_eoir: GICC_EOIR,
    #[doc = "0x14 - GICC running priority register"]
    pub gicc_rpr: GICC_RPR,
    #[doc = "0x18 - GICC highest priority pending interrupt register"]
    pub gicc_hppir: GICC_HPPIR,
    #[doc = "0x1c - GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR."]
    pub gicc_abpr: GICC_ABPR,
    #[doc = "0x20 - GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR."]
    pub gicc_aiar: GICC_AIAR,
    #[doc = "0x24 - GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used."]
    pub gicc_aeoir: GICC_AEOIR,
    #[doc = "0x28 - ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR."]
    pub gicc_ahppir: GICC_AHPPIR,
    _reserved11: [u8; 164usize],
    #[doc = "0xd0 - GICC active priority register"]
    pub gicc_apr0: GICC_APR0,
    _reserved12: [u8; 12usize],
    #[doc = "0xe0 - GICC non-secure active priority register"]
    pub gicc_nsapr0: GICC_NSAPR0,
    _reserved13: [u8; 24usize],
    #[doc = "0xfc - GICC interface identification register"]
    pub gicc_iidr: GICC_IIDR,
    _reserved14: [u8; 3840usize],
    #[doc = "0x1000 - GICC deactivate interrupt register"]
    pub gicc_dir: GICC_DIR,
}
#[doc = "GICC control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_ctlr](gicc_ctlr) module"]
pub type GICC_CTLR = crate::Reg<u32, _GICC_CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_CTLR;
#[doc = "`read()` method returns [gicc_ctlr::R](gicc_ctlr::R) reader structure"]
impl crate::Readable for GICC_CTLR {}
#[doc = "`write(|w| ..)` method takes [gicc_ctlr::W](gicc_ctlr::W) writer structure"]
impl crate::Writable for GICC_CTLR {}
#[doc = "GICC control register"]
pub mod gicc_ctlr;
#[doc = "GICC input priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_pmr](gicc_pmr) module"]
pub type GICC_PMR = crate::Reg<u32, _GICC_PMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_PMR;
#[doc = "`read()` method returns [gicc_pmr::R](gicc_pmr::R) reader structure"]
impl crate::Readable for GICC_PMR {}
#[doc = "`write(|w| ..)` method takes [gicc_pmr::W](gicc_pmr::W) writer structure"]
impl crate::Writable for GICC_PMR {}
#[doc = "GICC input priority mask register"]
pub mod gicc_pmr;
#[doc = "GICC binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_bpr](gicc_bpr) module"]
pub type GICC_BPR = crate::Reg<u32, _GICC_BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_BPR;
#[doc = "`read()` method returns [gicc_bpr::R](gicc_bpr::R) reader structure"]
impl crate::Readable for GICC_BPR {}
#[doc = "`write(|w| ..)` method takes [gicc_bpr::W](gicc_bpr::W) writer structure"]
impl crate::Writable for GICC_BPR {}
#[doc = "GICC binary point register"]
pub mod gicc_bpr;
#[doc = "GICC interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_iar](gicc_iar) module"]
pub type GICC_IAR = crate::Reg<u32, _GICC_IAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_IAR;
#[doc = "`read()` method returns [gicc_iar::R](gicc_iar::R) reader structure"]
impl crate::Readable for GICC_IAR {}
#[doc = "GICC interrupt acknowledge register"]
pub mod gicc_iar;
#[doc = "GICC end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_eoir](gicc_eoir) module"]
pub type GICC_EOIR = crate::Reg<u32, _GICC_EOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_EOIR;
#[doc = "`write(|w| ..)` method takes [gicc_eoir::W](gicc_eoir::W) writer structure"]
impl crate::Writable for GICC_EOIR {}
#[doc = "GICC end of interrupt register"]
pub mod gicc_eoir;
#[doc = "GICC running priority register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_rpr](gicc_rpr) module"]
pub type GICC_RPR = crate::Reg<u32, _GICC_RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_RPR;
#[doc = "`read()` method returns [gicc_rpr::R](gicc_rpr::R) reader structure"]
impl crate::Readable for GICC_RPR {}
#[doc = "GICC running priority register"]
pub mod gicc_rpr;
#[doc = "GICC highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_hppir](gicc_hppir) module"]
pub type GICC_HPPIR = crate::Reg<u32, _GICC_HPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_HPPIR;
#[doc = "`read()` method returns [gicc_hppir::R](gicc_hppir::R) reader structure"]
impl crate::Readable for GICC_HPPIR {}
#[doc = "GICC highest priority pending interrupt register"]
pub mod gicc_hppir;
#[doc = "GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_abpr](gicc_abpr) module"]
pub type GICC_ABPR = crate::Reg<u32, _GICC_ABPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_ABPR;
#[doc = "`read()` method returns [gicc_abpr::R](gicc_abpr::R) reader structure"]
impl crate::Readable for GICC_ABPR {}
#[doc = "`write(|w| ..)` method takes [gicc_abpr::W](gicc_abpr::W) writer structure"]
impl crate::Writable for GICC_ABPR {}
#[doc = "GICC_ABPR is an alias of the non-secure GICC_BPR. When GICC_CTLR.CBPR is set to 0, a secure access to this register is equivalent to a non-secure access to GICC_BPR."]
pub mod gicc_abpr;
#[doc = "GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_aiar](gicc_aiar) module"]
pub type GICC_AIAR = crate::Reg<u32, _GICC_AIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_AIAR;
#[doc = "`read()` method returns [gicc_aiar::R](gicc_aiar::R) reader structure"]
impl crate::Readable for GICC_AIAR {}
#[doc = "GICC_AIAR is an alias of the non-secure view of GICC_IAR. A secure access to this register is identical to a non-secure access to GICC_IAR."]
pub mod gicc_aiar;
#[doc = "GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_aeoir](gicc_aeoir) module"]
pub type GICC_AEOIR = crate::Reg<u32, _GICC_AEOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_AEOIR;
#[doc = "`write(|w| ..)` method takes [gicc_aeoir::W](gicc_aeoir::W) writer structure"]
impl crate::Writable for GICC_AEOIR {}
#[doc = "GICC_AEOIR is an alias of the Non-secure GICC_EOIR. A secure access to this register is similar to a non-secure access to GICC_EOIR, except that the GICC_CTLR.EOImodeS bit is used."]
pub mod gicc_aeoir;
#[doc = "ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_ahppir](gicc_ahppir) module"]
pub type GICC_AHPPIR = crate::Reg<u32, _GICC_AHPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_AHPPIR;
#[doc = "`read()` method returns [gicc_ahppir::R](gicc_ahppir::R) reader structure"]
impl crate::Readable for GICC_AHPPIR {}
#[doc = "ICC_AHPPIR is an alias of the non-secure GICC_HPPIR. A secure access to this register is equivalent to a non-secure access to GICC_HPPIR."]
pub mod gicc_ahppir;
#[doc = "GICC active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_apr0](gicc_apr0) module"]
pub type GICC_APR0 = crate::Reg<u32, _GICC_APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_APR0;
#[doc = "`read()` method returns [gicc_apr0::R](gicc_apr0::R) reader structure"]
impl crate::Readable for GICC_APR0 {}
#[doc = "`write(|w| ..)` method takes [gicc_apr0::W](gicc_apr0::W) writer structure"]
impl crate::Writable for GICC_APR0 {}
#[doc = "GICC active priority register"]
pub mod gicc_apr0;
#[doc = "GICC non-secure active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_nsapr0](gicc_nsapr0) module"]
pub type GICC_NSAPR0 = crate::Reg<u32, _GICC_NSAPR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_NSAPR0;
#[doc = "`read()` method returns [gicc_nsapr0::R](gicc_nsapr0::R) reader structure"]
impl crate::Readable for GICC_NSAPR0 {}
#[doc = "`write(|w| ..)` method takes [gicc_nsapr0::W](gicc_nsapr0::W) writer structure"]
impl crate::Writable for GICC_NSAPR0 {}
#[doc = "GICC non-secure active priority register"]
pub mod gicc_nsapr0;
#[doc = "GICC interface identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_iidr](gicc_iidr) module"]
pub type GICC_IIDR = crate::Reg<u32, _GICC_IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_IIDR;
#[doc = "`read()` method returns [gicc_iidr::R](gicc_iidr::R) reader structure"]
impl crate::Readable for GICC_IIDR {}
#[doc = "GICC interface identification register"]
pub mod gicc_iidr;
#[doc = "GICC deactivate interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_dir](gicc_dir) module"]
pub type GICC_DIR = crate::Reg<u32, _GICC_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICC_DIR;
#[doc = "`write(|w| ..)` method takes [gicc_dir::W](gicc_dir::W) writer structure"]
impl crate::Writable for GICC_DIR {}
#[doc = "GICC deactivate interrupt register"]
pub mod gicc_dir;
