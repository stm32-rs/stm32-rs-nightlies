#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Global configuration register"]
    pub sai_gcr: SAI_GCR,
    #[doc = "0x04 - Configuration register 1"]
    pub sai_acr1: SAI_ACR1,
    #[doc = "0x08 - Configuration register 2"]
    pub sai_acr2: SAI_ACR2,
    #[doc = "0x0c - This register has no meaning in and SPDIF audio protocol"]
    pub sai_afrcr: SAI_AFRCR,
    #[doc = "0x10 - This register has no meaning in and SPDIF audio protocol"]
    pub sai_aslotr: SAI_ASLOTR,
    #[doc = "0x14 - Interrupt mask register"]
    pub sai_aim: SAI_AIM,
    #[doc = "0x18 - Status register"]
    pub sai_asr: SAI_ASR,
    #[doc = "0x1c - Clear flag register"]
    pub sai_aclrfr: SAI_ACLRFR,
    #[doc = "0x20 - Data register"]
    pub sai_adr: SAI_ADR,
    #[doc = "0x24 - Configuration register 1"]
    pub sai_bcr1: SAI_BCR1,
    #[doc = "0x28 - Configuration register 2"]
    pub sai_bcr2: SAI_BCR2,
    #[doc = "0x2c - This register has no meaning in and SPDIF audio protocol"]
    pub sai_bfrcr: SAI_BFRCR,
    #[doc = "0x30 - This register has no meaning in and SPDIF audio protocol"]
    pub sai_bslotr: SAI_BSLOTR,
    #[doc = "0x34 - Interrupt mask register"]
    pub sai_bim: SAI_BIM,
    #[doc = "0x38 - Status register"]
    pub sai_bsr: SAI_BSR,
    #[doc = "0x3c - Clear flag register"]
    pub sai_bclrfr: SAI_BCLRFR,
    #[doc = "0x40 - Data register"]
    pub sai_bdr: SAI_BDR,
    #[doc = "0x44 - PDM control register"]
    pub sai_pdmcr: SAI_PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub sai_pdmdly: SAI_PDMDLY,
    _reserved19: [u8; 932usize],
    #[doc = "0x3f0 - SAI hardware configuration register"]
    pub sai_hwcfgr: SAI_HWCFGR,
    #[doc = "0x3f4 - SAI version register"]
    pub sai_verr: SAI_VERR,
    #[doc = "0x3f8 - SAI identification register"]
    pub sai_ipidr: SAI_IPIDR,
    #[doc = "0x3fc - SAI size identification register"]
    pub sai_sidr: SAI_SIDR,
}
#[doc = "Global configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_gcr](sai_gcr) module"]
pub type SAI_GCR = crate::Reg<u32, _SAI_GCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_GCR;
#[doc = "`read()` method returns [sai_gcr::R](sai_gcr::R) reader structure"]
impl crate::Readable for SAI_GCR {}
#[doc = "`write(|w| ..)` method takes [sai_gcr::W](sai_gcr::W) writer structure"]
impl crate::Writable for SAI_GCR {}
#[doc = "Global configuration register"]
pub mod sai_gcr;
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr1](sai_acr1) module"]
pub type SAI_ACR1 = crate::Reg<u32, _SAI_ACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACR1;
#[doc = "`read()` method returns [sai_acr1::R](sai_acr1::R) reader structure"]
impl crate::Readable for SAI_ACR1 {}
#[doc = "`write(|w| ..)` method takes [sai_acr1::W](sai_acr1::W) writer structure"]
impl crate::Writable for SAI_ACR1 {}
#[doc = "Configuration register 1"]
pub mod sai_acr1;
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr2](sai_acr2) module"]
pub type SAI_ACR2 = crate::Reg<u32, _SAI_ACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACR2;
#[doc = "`read()` method returns [sai_acr2::R](sai_acr2::R) reader structure"]
impl crate::Readable for SAI_ACR2 {}
#[doc = "`write(|w| ..)` method takes [sai_acr2::W](sai_acr2::W) writer structure"]
impl crate::Writable for SAI_ACR2 {}
#[doc = "Configuration register 2"]
pub mod sai_acr2;
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_afrcr](sai_afrcr) module"]
pub type SAI_AFRCR = crate::Reg<u32, _SAI_AFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_AFRCR;
#[doc = "`read()` method returns [sai_afrcr::R](sai_afrcr::R) reader structure"]
impl crate::Readable for SAI_AFRCR {}
#[doc = "`write(|w| ..)` method takes [sai_afrcr::W](sai_afrcr::W) writer structure"]
impl crate::Writable for SAI_AFRCR {}
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_afrcr;
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aslotr](sai_aslotr) module"]
pub type SAI_ASLOTR = crate::Reg<u32, _SAI_ASLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ASLOTR;
#[doc = "`read()` method returns [sai_aslotr::R](sai_aslotr::R) reader structure"]
impl crate::Readable for SAI_ASLOTR {}
#[doc = "`write(|w| ..)` method takes [sai_aslotr::W](sai_aslotr::W) writer structure"]
impl crate::Writable for SAI_ASLOTR {}
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_aslotr;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aim](sai_aim) module"]
pub type SAI_AIM = crate::Reg<u32, _SAI_AIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_AIM;
#[doc = "`read()` method returns [sai_aim::R](sai_aim::R) reader structure"]
impl crate::Readable for SAI_AIM {}
#[doc = "`write(|w| ..)` method takes [sai_aim::W](sai_aim::W) writer structure"]
impl crate::Writable for SAI_AIM {}
#[doc = "Interrupt mask register"]
pub mod sai_aim;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_asr](sai_asr) module"]
pub type SAI_ASR = crate::Reg<u32, _SAI_ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ASR;
#[doc = "`read()` method returns [sai_asr::R](sai_asr::R) reader structure"]
impl crate::Readable for SAI_ASR {}
#[doc = "Status register"]
pub mod sai_asr;
#[doc = "Clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_aclrfr](sai_aclrfr) module"]
pub type SAI_ACLRFR = crate::Reg<u32, _SAI_ACLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ACLRFR;
#[doc = "`write(|w| ..)` method takes [sai_aclrfr::W](sai_aclrfr::W) writer structure"]
impl crate::Writable for SAI_ACLRFR {}
#[doc = "Clear flag register"]
pub mod sai_aclrfr;
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_adr](sai_adr) module"]
pub type SAI_ADR = crate::Reg<u32, _SAI_ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_ADR;
#[doc = "`read()` method returns [sai_adr::R](sai_adr::R) reader structure"]
impl crate::Readable for SAI_ADR {}
#[doc = "`write(|w| ..)` method takes [sai_adr::W](sai_adr::W) writer structure"]
impl crate::Writable for SAI_ADR {}
#[doc = "Data register"]
pub mod sai_adr;
#[doc = "Configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bcr1](sai_bcr1) module"]
pub type SAI_BCR1 = crate::Reg<u32, _SAI_BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCR1;
#[doc = "`read()` method returns [sai_bcr1::R](sai_bcr1::R) reader structure"]
impl crate::Readable for SAI_BCR1 {}
#[doc = "`write(|w| ..)` method takes [sai_bcr1::W](sai_bcr1::W) writer structure"]
impl crate::Writable for SAI_BCR1 {}
#[doc = "Configuration register 1"]
pub mod sai_bcr1;
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bcr2](sai_bcr2) module"]
pub type SAI_BCR2 = crate::Reg<u32, _SAI_BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCR2;
#[doc = "`read()` method returns [sai_bcr2::R](sai_bcr2::R) reader structure"]
impl crate::Readable for SAI_BCR2 {}
#[doc = "`write(|w| ..)` method takes [sai_bcr2::W](sai_bcr2::W) writer structure"]
impl crate::Writable for SAI_BCR2 {}
#[doc = "Configuration register 2"]
pub mod sai_bcr2;
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bfrcr](sai_bfrcr) module"]
pub type SAI_BFRCR = crate::Reg<u32, _SAI_BFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BFRCR;
#[doc = "`read()` method returns [sai_bfrcr::R](sai_bfrcr::R) reader structure"]
impl crate::Readable for SAI_BFRCR {}
#[doc = "`write(|w| ..)` method takes [sai_bfrcr::W](sai_bfrcr::W) writer structure"]
impl crate::Writable for SAI_BFRCR {}
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bfrcr;
#[doc = "This register has no meaning in and SPDIF audio protocol\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bslotr](sai_bslotr) module"]
pub type SAI_BSLOTR = crate::Reg<u32, _SAI_BSLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BSLOTR;
#[doc = "`read()` method returns [sai_bslotr::R](sai_bslotr::R) reader structure"]
impl crate::Readable for SAI_BSLOTR {}
#[doc = "`write(|w| ..)` method takes [sai_bslotr::W](sai_bslotr::W) writer structure"]
impl crate::Writable for SAI_BSLOTR {}
#[doc = "This register has no meaning in and SPDIF audio protocol"]
pub mod sai_bslotr;
#[doc = "Interrupt mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bim](sai_bim) module"]
pub type SAI_BIM = crate::Reg<u32, _SAI_BIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BIM;
#[doc = "`read()` method returns [sai_bim::R](sai_bim::R) reader structure"]
impl crate::Readable for SAI_BIM {}
#[doc = "`write(|w| ..)` method takes [sai_bim::W](sai_bim::W) writer structure"]
impl crate::Writable for SAI_BIM {}
#[doc = "Interrupt mask register"]
pub mod sai_bim;
#[doc = "Status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bsr](sai_bsr) module"]
pub type SAI_BSR = crate::Reg<u32, _SAI_BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BSR;
#[doc = "`read()` method returns [sai_bsr::R](sai_bsr::R) reader structure"]
impl crate::Readable for SAI_BSR {}
#[doc = "Status register"]
pub mod sai_bsr;
#[doc = "Clear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bclrfr](sai_bclrfr) module"]
pub type SAI_BCLRFR = crate::Reg<u32, _SAI_BCLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BCLRFR;
#[doc = "`write(|w| ..)` method takes [sai_bclrfr::W](sai_bclrfr::W) writer structure"]
impl crate::Writable for SAI_BCLRFR {}
#[doc = "Clear flag register"]
pub mod sai_bclrfr;
#[doc = "Data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_bdr](sai_bdr) module"]
pub type SAI_BDR = crate::Reg<u32, _SAI_BDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_BDR;
#[doc = "`read()` method returns [sai_bdr::R](sai_bdr::R) reader structure"]
impl crate::Readable for SAI_BDR {}
#[doc = "`write(|w| ..)` method takes [sai_bdr::W](sai_bdr::W) writer structure"]
impl crate::Writable for SAI_BDR {}
#[doc = "Data register"]
pub mod sai_bdr;
#[doc = "PDM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_pdmcr](sai_pdmcr) module"]
pub type SAI_PDMCR = crate::Reg<u32, _SAI_PDMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_PDMCR;
#[doc = "`read()` method returns [sai_pdmcr::R](sai_pdmcr::R) reader structure"]
impl crate::Readable for SAI_PDMCR {}
#[doc = "`write(|w| ..)` method takes [sai_pdmcr::W](sai_pdmcr::W) writer structure"]
impl crate::Writable for SAI_PDMCR {}
#[doc = "PDM control register"]
pub mod sai_pdmcr;
#[doc = "PDM delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_pdmdly](sai_pdmdly) module"]
pub type SAI_PDMDLY = crate::Reg<u32, _SAI_PDMDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_PDMDLY;
#[doc = "`read()` method returns [sai_pdmdly::R](sai_pdmdly::R) reader structure"]
impl crate::Readable for SAI_PDMDLY {}
#[doc = "`write(|w| ..)` method takes [sai_pdmdly::W](sai_pdmdly::W) writer structure"]
impl crate::Writable for SAI_PDMDLY {}
#[doc = "PDM delay register"]
pub mod sai_pdmdly;
#[doc = "SAI hardware configuration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_hwcfgr](sai_hwcfgr) module"]
pub type SAI_HWCFGR = crate::Reg<u32, _SAI_HWCFGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_HWCFGR;
#[doc = "`read()` method returns [sai_hwcfgr::R](sai_hwcfgr::R) reader structure"]
impl crate::Readable for SAI_HWCFGR {}
#[doc = "SAI hardware configuration register"]
pub mod sai_hwcfgr;
#[doc = "SAI version register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_verr](sai_verr) module"]
pub type SAI_VERR = crate::Reg<u32, _SAI_VERR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_VERR;
#[doc = "`read()` method returns [sai_verr::R](sai_verr::R) reader structure"]
impl crate::Readable for SAI_VERR {}
#[doc = "SAI version register"]
pub mod sai_verr;
#[doc = "SAI identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_ipidr](sai_ipidr) module"]
pub type SAI_IPIDR = crate::Reg<u32, _SAI_IPIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_IPIDR;
#[doc = "`read()` method returns [sai_ipidr::R](sai_ipidr::R) reader structure"]
impl crate::Readable for SAI_IPIDR {}
#[doc = "SAI identification register"]
pub mod sai_ipidr;
#[doc = "SAI size identification register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_sidr](sai_sidr) module"]
pub type SAI_SIDR = crate::Reg<u32, _SAI_SIDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SAI_SIDR;
#[doc = "`read()` method returns [sai_sidr::R](sai_sidr::R) reader structure"]
impl crate::Readable for SAI_SIDR {}
#[doc = "SAI size identification register"]
pub mod sai_sidr;
