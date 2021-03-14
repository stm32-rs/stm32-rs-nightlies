#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 4usize],
    #[doc = "0x04 - AConfiguration register 1"]
    pub acr1: ACR1,
    #[doc = "0x08 - AConfiguration register 2"]
    pub acr2: ACR2,
    #[doc = "0x0c - AFRCR"]
    pub afrcr: AFRCR,
    #[doc = "0x10 - ASlot register"]
    pub aslotr: ASLOTR,
    #[doc = "0x14 - AInterrupt mask register2"]
    pub aim: AIM,
    #[doc = "0x18 - AStatus register"]
    pub asr: ASR,
    #[doc = "0x1c - AClear flag register"]
    pub aclrfr: ACLRFR,
    #[doc = "0x20 - AData register"]
    pub adr: ADR,
    #[doc = "0x24 - BConfiguration register 1"]
    pub bcr1: BCR1,
    #[doc = "0x28 - BConfiguration register 2"]
    pub bcr2: BCR2,
    #[doc = "0x2c - BFRCR"]
    pub bfrcr: BFRCR,
    #[doc = "0x30 - BSlot register"]
    pub bslotr: BSLOTR,
    #[doc = "0x34 - BInterrupt mask register2"]
    pub bim: BIM,
    #[doc = "0x38 - BStatus register"]
    pub bsr: BSR,
    #[doc = "0x3c - BClear flag register"]
    pub bclrfr: BCLRFR,
    #[doc = "0x40 - BData register"]
    pub bdr: BDR,
    #[doc = "0x44 - PDM control register"]
    pub pdmcr: PDMCR,
    #[doc = "0x48 - PDM delay register"]
    pub pdmdly: PDMDLY,
}
#[doc = "BConfiguration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr1](bcr1) module"]
pub type BCR1 = crate::Reg<u32, _BCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR1;
#[doc = "`read()` method returns [bcr1::R](bcr1::R) reader structure"]
impl crate::Readable for BCR1 {}
#[doc = "`write(|w| ..)` method takes [bcr1::W](bcr1::W) writer structure"]
impl crate::Writable for BCR1 {}
#[doc = "BConfiguration register 1"]
pub mod bcr1;
#[doc = "BConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bcr2](bcr2) module"]
pub type BCR2 = crate::Reg<u32, _BCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCR2;
#[doc = "`read()` method returns [bcr2::R](bcr2::R) reader structure"]
impl crate::Readable for BCR2 {}
#[doc = "`write(|w| ..)` method takes [bcr2::W](bcr2::W) writer structure"]
impl crate::Writable for BCR2 {}
#[doc = "BConfiguration register 2"]
pub mod bcr2;
#[doc = "BFRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bfrcr](bfrcr) module"]
pub type BFRCR = crate::Reg<u32, _BFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BFRCR;
#[doc = "`read()` method returns [bfrcr::R](bfrcr::R) reader structure"]
impl crate::Readable for BFRCR {}
#[doc = "`write(|w| ..)` method takes [bfrcr::W](bfrcr::W) writer structure"]
impl crate::Writable for BFRCR {}
#[doc = "BFRCR"]
pub mod bfrcr;
#[doc = "BSlot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bslotr](bslotr) module"]
pub type BSLOTR = crate::Reg<u32, _BSLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSLOTR;
#[doc = "`read()` method returns [bslotr::R](bslotr::R) reader structure"]
impl crate::Readable for BSLOTR {}
#[doc = "`write(|w| ..)` method takes [bslotr::W](bslotr::W) writer structure"]
impl crate::Writable for BSLOTR {}
#[doc = "BSlot register"]
pub mod bslotr;
#[doc = "BInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bim](bim) module"]
pub type BIM = crate::Reg<u32, _BIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BIM;
#[doc = "`read()` method returns [bim::R](bim::R) reader structure"]
impl crate::Readable for BIM {}
#[doc = "`write(|w| ..)` method takes [bim::W](bim::W) writer structure"]
impl crate::Writable for BIM {}
#[doc = "BInterrupt mask register2"]
pub mod bim;
#[doc = "BStatus register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bsr](bsr) module"]
pub type BSR = crate::Reg<u32, _BSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BSR;
#[doc = "`read()` method returns [bsr::R](bsr::R) reader structure"]
impl crate::Readable for BSR {}
#[doc = "BStatus register"]
pub mod bsr;
#[doc = "BClear flag register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bclrfr](bclrfr) module"]
pub type BCLRFR = crate::Reg<u32, _BCLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BCLRFR;
#[doc = "`write(|w| ..)` method takes [bclrfr::W](bclrfr::W) writer structure"]
impl crate::Writable for BCLRFR {}
#[doc = "BClear flag register"]
pub mod bclrfr;
#[doc = "BData register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdr](bdr) module"]
pub type BDR = crate::Reg<u32, _BDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDR;
#[doc = "`read()` method returns [bdr::R](bdr::R) reader structure"]
impl crate::Readable for BDR {}
#[doc = "`write(|w| ..)` method takes [bdr::W](bdr::W) writer structure"]
impl crate::Writable for BDR {}
#[doc = "BData register"]
pub mod bdr;
#[doc = "AConfiguration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr1](acr1) module"]
pub type ACR1 = crate::Reg<u32, _ACR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR1;
#[doc = "`read()` method returns [acr1::R](acr1::R) reader structure"]
impl crate::Readable for ACR1 {}
#[doc = "`write(|w| ..)` method takes [acr1::W](acr1::W) writer structure"]
impl crate::Writable for ACR1 {}
#[doc = "AConfiguration register 1"]
pub mod acr1;
#[doc = "AConfiguration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr2](acr2) module"]
pub type ACR2 = crate::Reg<u32, _ACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACR2;
#[doc = "`read()` method returns [acr2::R](acr2::R) reader structure"]
impl crate::Readable for ACR2 {}
#[doc = "`write(|w| ..)` method takes [acr2::W](acr2::W) writer structure"]
impl crate::Writable for ACR2 {}
#[doc = "AConfiguration register 2"]
pub mod acr2;
#[doc = "AFRCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [afrcr](afrcr) module"]
pub type AFRCR = crate::Reg<u32, _AFRCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AFRCR;
#[doc = "`read()` method returns [afrcr::R](afrcr::R) reader structure"]
impl crate::Readable for AFRCR {}
#[doc = "`write(|w| ..)` method takes [afrcr::W](afrcr::W) writer structure"]
impl crate::Writable for AFRCR {}
#[doc = "AFRCR"]
pub mod afrcr;
#[doc = "ASlot register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aslotr](aslotr) module"]
pub type ASLOTR = crate::Reg<u32, _ASLOTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASLOTR;
#[doc = "`read()` method returns [aslotr::R](aslotr::R) reader structure"]
impl crate::Readable for ASLOTR {}
#[doc = "`write(|w| ..)` method takes [aslotr::W](aslotr::W) writer structure"]
impl crate::Writable for ASLOTR {}
#[doc = "ASlot register"]
pub mod aslotr;
#[doc = "AInterrupt mask register2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aim](aim) module"]
pub type AIM = crate::Reg<u32, _AIM>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AIM;
#[doc = "`read()` method returns [aim::R](aim::R) reader structure"]
impl crate::Readable for AIM {}
#[doc = "`write(|w| ..)` method takes [aim::W](aim::W) writer structure"]
impl crate::Writable for AIM {}
#[doc = "AInterrupt mask register2"]
pub mod aim;
#[doc = "AStatus register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [asr](asr) module"]
pub type ASR = crate::Reg<u32, _ASR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ASR;
#[doc = "`read()` method returns [asr::R](asr::R) reader structure"]
impl crate::Readable for ASR {}
#[doc = "`write(|w| ..)` method takes [asr::W](asr::W) writer structure"]
impl crate::Writable for ASR {}
#[doc = "AStatus register"]
pub mod asr;
#[doc = "AClear flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aclrfr](aclrfr) module"]
pub type ACLRFR = crate::Reg<u32, _ACLRFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ACLRFR;
#[doc = "`read()` method returns [aclrfr::R](aclrfr::R) reader structure"]
impl crate::Readable for ACLRFR {}
#[doc = "`write(|w| ..)` method takes [aclrfr::W](aclrfr::W) writer structure"]
impl crate::Writable for ACLRFR {}
#[doc = "AClear flag register"]
pub mod aclrfr;
#[doc = "AData register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adr](adr) module"]
pub type ADR = crate::Reg<u32, _ADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADR;
#[doc = "`read()` method returns [adr::R](adr::R) reader structure"]
impl crate::Readable for ADR {}
#[doc = "`write(|w| ..)` method takes [adr::W](adr::W) writer structure"]
impl crate::Writable for ADR {}
#[doc = "AData register"]
pub mod adr;
#[doc = "PDM control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmcr](pdmcr) module"]
pub type PDMCR = crate::Reg<u32, _PDMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMCR;
#[doc = "`read()` method returns [pdmcr::R](pdmcr::R) reader structure"]
impl crate::Readable for PDMCR {}
#[doc = "`write(|w| ..)` method takes [pdmcr::W](pdmcr::W) writer structure"]
impl crate::Writable for PDMCR {}
#[doc = "PDM control register"]
pub mod pdmcr;
#[doc = "PDM delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmdly](pdmdly) module"]
pub type PDMDLY = crate::Reg<u32, _PDMDLY>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PDMDLY;
#[doc = "`read()` method returns [pdmdly::R](pdmdly::R) reader structure"]
impl crate::Readable for PDMDLY {}
#[doc = "`write(|w| ..)` method takes [pdmdly::W](pdmdly::W) writer structure"]
impl crate::Writable for PDMDLY {}
#[doc = "PDM delay register"]
pub mod pdmdly;
