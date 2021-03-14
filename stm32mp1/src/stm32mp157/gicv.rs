#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - GICV virtual machine control register"]
    pub gicv_ctlr: GICV_CTLR,
    #[doc = "0x04 - GICV VM priority mask register"]
    pub gicv_pmr: GICV_PMR,
    #[doc = "0x08 - GICV VM binary point register"]
    pub gicv_bpr: GICV_BPR,
    #[doc = "0x0c - GICV VM interrupt acknowledge register"]
    pub gicv_iar: GICV_IAR,
    #[doc = "0x10 - GICV VM end of interrupt register"]
    pub gicv_eoir: GICV_EOIR,
    #[doc = "0x14 - GICV VM running priority register"]
    pub gicv_rpr: GICV_RPR,
    #[doc = "0x18 - GICV VM highest priority pending interrupt register"]
    pub gicv_hppir: GICV_HPPIR,
    #[doc = "0x1c - GICV VM aliased binary point register"]
    pub gicv_abpr: GICV_ABPR,
    #[doc = "0x20 - GICV VM aliased interrupt register"]
    pub gicv_aiar: GICV_AIAR,
    #[doc = "0x24 - GICV VM aliased end of interrupt register"]
    pub gicv_aeoir: GICV_AEOIR,
    #[doc = "0x28 - GICV VM aliased highest priority pending interrupt register"]
    pub gicv_ahppir: GICV_AHPPIR,
    _reserved11: [u8; 164usize],
    #[doc = "0xd0 - The GICV_APR0 is an alias of GICH_APR."]
    pub gicv_apr0: GICV_APR0,
    _reserved12: [u8; 40usize],
    #[doc = "0xfc - The GICV_IIDR is an alias of GICC_IIDR."]
    pub gicv_iidr: GICV_IIDR,
    _reserved13: [u8; 3840usize],
    #[doc = "0x1000 - GICV VM deactivate interrupt register"]
    pub gicv_dir: GICV_DIR,
}
#[doc = "GICV virtual machine control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_ctlr](gicv_ctlr) module"]
pub type GICV_CTLR = crate::Reg<u32, _GICV_CTLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_CTLR;
#[doc = "`read()` method returns [gicv_ctlr::R](gicv_ctlr::R) reader structure"]
impl crate::Readable for GICV_CTLR {}
#[doc = "`write(|w| ..)` method takes [gicv_ctlr::W](gicv_ctlr::W) writer structure"]
impl crate::Writable for GICV_CTLR {}
#[doc = "GICV virtual machine control register"]
pub mod gicv_ctlr;
#[doc = "GICV VM priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_pmr](gicv_pmr) module"]
pub type GICV_PMR = crate::Reg<u32, _GICV_PMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_PMR;
#[doc = "`read()` method returns [gicv_pmr::R](gicv_pmr::R) reader structure"]
impl crate::Readable for GICV_PMR {}
#[doc = "`write(|w| ..)` method takes [gicv_pmr::W](gicv_pmr::W) writer structure"]
impl crate::Writable for GICV_PMR {}
#[doc = "GICV VM priority mask register"]
pub mod gicv_pmr;
#[doc = "GICV VM binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_bpr](gicv_bpr) module"]
pub type GICV_BPR = crate::Reg<u32, _GICV_BPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_BPR;
#[doc = "`read()` method returns [gicv_bpr::R](gicv_bpr::R) reader structure"]
impl crate::Readable for GICV_BPR {}
#[doc = "`write(|w| ..)` method takes [gicv_bpr::W](gicv_bpr::W) writer structure"]
impl crate::Writable for GICV_BPR {}
#[doc = "GICV VM binary point register"]
pub mod gicv_bpr;
#[doc = "GICV VM interrupt acknowledge register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_iar](gicv_iar) module"]
pub type GICV_IAR = crate::Reg<u32, _GICV_IAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_IAR;
#[doc = "`read()` method returns [gicv_iar::R](gicv_iar::R) reader structure"]
impl crate::Readable for GICV_IAR {}
#[doc = "GICV VM interrupt acknowledge register"]
pub mod gicv_iar;
#[doc = "GICV VM end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_eoir](gicv_eoir) module"]
pub type GICV_EOIR = crate::Reg<u32, _GICV_EOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_EOIR;
#[doc = "`write(|w| ..)` method takes [gicv_eoir::W](gicv_eoir::W) writer structure"]
impl crate::Writable for GICV_EOIR {}
#[doc = "GICV VM end of interrupt register"]
pub mod gicv_eoir;
#[doc = "GICV VM running priority register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_rpr](gicv_rpr) module"]
pub type GICV_RPR = crate::Reg<u32, _GICV_RPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_RPR;
#[doc = "`read()` method returns [gicv_rpr::R](gicv_rpr::R) reader structure"]
impl crate::Readable for GICV_RPR {}
#[doc = "GICV VM running priority register"]
pub mod gicv_rpr;
#[doc = "GICV VM highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_hppir](gicv_hppir) module"]
pub type GICV_HPPIR = crate::Reg<u32, _GICV_HPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_HPPIR;
#[doc = "`read()` method returns [gicv_hppir::R](gicv_hppir::R) reader structure"]
impl crate::Readable for GICV_HPPIR {}
#[doc = "GICV VM highest priority pending interrupt register"]
pub mod gicv_hppir;
#[doc = "GICV VM aliased binary point register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_abpr](gicv_abpr) module"]
pub type GICV_ABPR = crate::Reg<u32, _GICV_ABPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_ABPR;
#[doc = "`read()` method returns [gicv_abpr::R](gicv_abpr::R) reader structure"]
impl crate::Readable for GICV_ABPR {}
#[doc = "`write(|w| ..)` method takes [gicv_abpr::W](gicv_abpr::W) writer structure"]
impl crate::Writable for GICV_ABPR {}
#[doc = "GICV VM aliased binary point register"]
pub mod gicv_abpr;
#[doc = "GICV VM aliased interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_aiar](gicv_aiar) module"]
pub type GICV_AIAR = crate::Reg<u32, _GICV_AIAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_AIAR;
#[doc = "`read()` method returns [gicv_aiar::R](gicv_aiar::R) reader structure"]
impl crate::Readable for GICV_AIAR {}
#[doc = "GICV VM aliased interrupt register"]
pub mod gicv_aiar;
#[doc = "GICV VM aliased end of interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_aeoir](gicv_aeoir) module"]
pub type GICV_AEOIR = crate::Reg<u32, _GICV_AEOIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_AEOIR;
#[doc = "`write(|w| ..)` method takes [gicv_aeoir::W](gicv_aeoir::W) writer structure"]
impl crate::Writable for GICV_AEOIR {}
#[doc = "GICV VM aliased end of interrupt register"]
pub mod gicv_aeoir;
#[doc = "GICV VM aliased highest priority pending interrupt register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_ahppir](gicv_ahppir) module"]
pub type GICV_AHPPIR = crate::Reg<u32, _GICV_AHPPIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_AHPPIR;
#[doc = "`read()` method returns [gicv_ahppir::R](gicv_ahppir::R) reader structure"]
impl crate::Readable for GICV_AHPPIR {}
#[doc = "GICV VM aliased highest priority pending interrupt register"]
pub mod gicv_ahppir;
#[doc = "The GICV_APR0 is an alias of GICH_APR.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_apr0](gicv_apr0) module"]
pub type GICV_APR0 = crate::Reg<u32, _GICV_APR0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_APR0;
#[doc = "`read()` method returns [gicv_apr0::R](gicv_apr0::R) reader structure"]
impl crate::Readable for GICV_APR0 {}
#[doc = "`write(|w| ..)` method takes [gicv_apr0::W](gicv_apr0::W) writer structure"]
impl crate::Writable for GICV_APR0 {}
#[doc = "The GICV_APR0 is an alias of GICH_APR."]
pub mod gicv_apr0;
#[doc = "The GICV_IIDR is an alias of GICC_IIDR.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_iidr](gicv_iidr) module"]
pub type GICV_IIDR = crate::Reg<u32, _GICV_IIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_IIDR;
#[doc = "`read()` method returns [gicv_iidr::R](gicv_iidr::R) reader structure"]
impl crate::Readable for GICV_IIDR {}
#[doc = "The GICV_IIDR is an alias of GICC_IIDR."]
pub mod gicv_iidr;
#[doc = "GICV VM deactivate interrupt register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_dir](gicv_dir) module"]
pub type GICV_DIR = crate::Reg<u32, _GICV_DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _GICV_DIR;
#[doc = "`write(|w| ..)` method takes [gicv_dir::W](gicv_dir::W) writer structure"]
impl crate::Writable for GICV_DIR {}
#[doc = "GICV VM deactivate interrupt register"]
pub mod gicv_dir;
