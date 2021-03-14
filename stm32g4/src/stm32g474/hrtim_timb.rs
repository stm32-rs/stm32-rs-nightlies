#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timbcr: TIMBCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timbisr: TIMBISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timbicr: TIMBICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timbdier: TIMBDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntr: CNTR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perbr: PERBR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repbr: REPBR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1br: CMP1BR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cbr: CMP1CBR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2br: CMP2BR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3br: CMP3BR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4br: CMP4BR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1br: CPT1BR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2br: CPT2BR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtbr: DTBR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setb1r: SETB1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstb1r: RSTB1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setb2r: SETB2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstb2r: RSTB2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefbr1: EEFBR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefbr2: EEFBR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstbr: RSTBR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpbr: CHPBR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1bcr: CPT1BCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2bcr: CPT2BCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outbr: OUTBR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltbr: FLTBR,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timbcr2: TIMBCR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub beefr3: BEEFR3,
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbcr](timbcr) module"]
pub type TIMBCR = crate::Reg<u32, _TIMBCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBCR;
#[doc = "`read()` method returns [timbcr::R](timbcr::R) reader structure"]
impl crate::Readable for TIMBCR {}
#[doc = "`write(|w| ..)` method takes [timbcr::W](timbcr::W) writer structure"]
impl crate::Writable for TIMBCR {}
#[doc = "Timerx Control Register"]
pub mod timbcr;
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbisr](timbisr) module"]
pub type TIMBISR = crate::Reg<u32, _TIMBISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBISR;
#[doc = "`read()` method returns [timbisr::R](timbisr::R) reader structure"]
impl crate::Readable for TIMBISR {}
#[doc = "Timerx Interrupt Status Register"]
pub mod timbisr;
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbicr](timbicr) module"]
pub type TIMBICR = crate::Reg<u32, _TIMBICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBICR;
#[doc = "`write(|w| ..)` method takes [timbicr::W](timbicr::W) writer structure"]
impl crate::Writable for TIMBICR {}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timbicr;
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbdier](timbdier) module"]
pub type TIMBDIER = crate::Reg<u32, _TIMBDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBDIER;
#[doc = "`read()` method returns [timbdier::R](timbdier::R) reader structure"]
impl crate::Readable for TIMBDIER {}
#[doc = "`write(|w| ..)` method takes [timbdier::W](timbdier::W) writer structure"]
impl crate::Writable for TIMBDIER {}
#[doc = "TIMxDIER"]
pub mod timbdier;
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntr](cntr) module"]
pub type CNTR = crate::Reg<u32, _CNTR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTR;
#[doc = "`read()` method returns [cntr::R](cntr::R) reader structure"]
impl crate::Readable for CNTR {}
#[doc = "`write(|w| ..)` method takes [cntr::W](cntr::W) writer structure"]
impl crate::Writable for CNTR {}
#[doc = "Timerx Counter Register"]
pub mod cntr;
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perbr](perbr) module"]
pub type PERBR = crate::Reg<u32, _PERBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERBR;
#[doc = "`read()` method returns [perbr::R](perbr::R) reader structure"]
impl crate::Readable for PERBR {}
#[doc = "`write(|w| ..)` method takes [perbr::W](perbr::W) writer structure"]
impl crate::Writable for PERBR {}
#[doc = "Timerx Period Register"]
pub mod perbr;
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repbr](repbr) module"]
pub type REPBR = crate::Reg<u32, _REPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPBR;
#[doc = "`read()` method returns [repbr::R](repbr::R) reader structure"]
impl crate::Readable for REPBR {}
#[doc = "`write(|w| ..)` method takes [repbr::W](repbr::W) writer structure"]
impl crate::Writable for REPBR {}
#[doc = "Timerx Repetition Register"]
pub mod repbr;
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1br](cmp1br) module"]
pub type CMP1BR = crate::Reg<u32, _CMP1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1BR;
#[doc = "`read()` method returns [cmp1br::R](cmp1br::R) reader structure"]
impl crate::Readable for CMP1BR {}
#[doc = "`write(|w| ..)` method takes [cmp1br::W](cmp1br::W) writer structure"]
impl crate::Writable for CMP1BR {}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1br;
#[doc = "Timerx Compare 1 Compound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1cbr](cmp1cbr) module"]
pub type CMP1CBR = crate::Reg<u32, _CMP1CBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CBR;
#[doc = "`read()` method returns [cmp1cbr::R](cmp1cbr::R) reader structure"]
impl crate::Readable for CMP1CBR {}
#[doc = "`write(|w| ..)` method takes [cmp1cbr::W](cmp1cbr::W) writer structure"]
impl crate::Writable for CMP1CBR {}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cbr;
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2br](cmp2br) module"]
pub type CMP2BR = crate::Reg<u32, _CMP2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2BR;
#[doc = "`read()` method returns [cmp2br::R](cmp2br::R) reader structure"]
impl crate::Readable for CMP2BR {}
#[doc = "`write(|w| ..)` method takes [cmp2br::W](cmp2br::W) writer structure"]
impl crate::Writable for CMP2BR {}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2br;
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3br](cmp3br) module"]
pub type CMP3BR = crate::Reg<u32, _CMP3BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3BR;
#[doc = "`read()` method returns [cmp3br::R](cmp3br::R) reader structure"]
impl crate::Readable for CMP3BR {}
#[doc = "`write(|w| ..)` method takes [cmp3br::W](cmp3br::W) writer structure"]
impl crate::Writable for CMP3BR {}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3br;
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4br](cmp4br) module"]
pub type CMP4BR = crate::Reg<u32, _CMP4BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4BR;
#[doc = "`read()` method returns [cmp4br::R](cmp4br::R) reader structure"]
impl crate::Readable for CMP4BR {}
#[doc = "`write(|w| ..)` method takes [cmp4br::W](cmp4br::W) writer structure"]
impl crate::Writable for CMP4BR {}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4br;
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1br](cpt1br) module"]
pub type CPT1BR = crate::Reg<u32, _CPT1BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1BR;
#[doc = "`read()` method returns [cpt1br::R](cpt1br::R) reader structure"]
impl crate::Readable for CPT1BR {}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1br;
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2br](cpt2br) module"]
pub type CPT2BR = crate::Reg<u32, _CPT2BR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2BR;
#[doc = "`read()` method returns [cpt2br::R](cpt2br::R) reader structure"]
impl crate::Readable for CPT2BR {}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2br;
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtbr](dtbr) module"]
pub type DTBR = crate::Reg<u32, _DTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTBR;
#[doc = "`read()` method returns [dtbr::R](dtbr::R) reader structure"]
impl crate::Readable for DTBR {}
#[doc = "`write(|w| ..)` method takes [dtbr::W](dtbr::W) writer structure"]
impl crate::Writable for DTBR {}
#[doc = "Timerx Deadtime Register"]
pub mod dtbr;
#[doc = "Timerx Output1 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setb1r](setb1r) module"]
pub type SETB1R = crate::Reg<u32, _SETB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETB1R;
#[doc = "`read()` method returns [setb1r::R](setb1r::R) reader structure"]
impl crate::Readable for SETB1R {}
#[doc = "`write(|w| ..)` method takes [setb1r::W](setb1r::W) writer structure"]
impl crate::Writable for SETB1R {}
#[doc = "Timerx Output1 Set Register"]
pub mod setb1r;
#[doc = "Timerx Output1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstb1r](rstb1r) module"]
pub type RSTB1R = crate::Reg<u32, _RSTB1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTB1R;
#[doc = "`read()` method returns [rstb1r::R](rstb1r::R) reader structure"]
impl crate::Readable for RSTB1R {}
#[doc = "`write(|w| ..)` method takes [rstb1r::W](rstb1r::W) writer structure"]
impl crate::Writable for RSTB1R {}
#[doc = "Timerx Output1 Reset Register"]
pub mod rstb1r;
#[doc = "Timerx Output2 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setb2r](setb2r) module"]
pub type SETB2R = crate::Reg<u32, _SETB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETB2R;
#[doc = "`read()` method returns [setb2r::R](setb2r::R) reader structure"]
impl crate::Readable for SETB2R {}
#[doc = "`write(|w| ..)` method takes [setb2r::W](setb2r::W) writer structure"]
impl crate::Writable for SETB2R {}
#[doc = "Timerx Output2 Set Register"]
pub mod setb2r;
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstb2r](rstb2r) module"]
pub type RSTB2R = crate::Reg<u32, _RSTB2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTB2R;
#[doc = "`read()` method returns [rstb2r::R](rstb2r::R) reader structure"]
impl crate::Readable for RSTB2R {}
#[doc = "`write(|w| ..)` method takes [rstb2r::W](rstb2r::W) writer structure"]
impl crate::Writable for RSTB2R {}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstb2r;
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefbr1](eefbr1) module"]
pub type EEFBR1 = crate::Reg<u32, _EEFBR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFBR1;
#[doc = "`read()` method returns [eefbr1::R](eefbr1::R) reader structure"]
impl crate::Readable for EEFBR1 {}
#[doc = "`write(|w| ..)` method takes [eefbr1::W](eefbr1::W) writer structure"]
impl crate::Writable for EEFBR1 {}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefbr1;
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefbr2](eefbr2) module"]
pub type EEFBR2 = crate::Reg<u32, _EEFBR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFBR2;
#[doc = "`read()` method returns [eefbr2::R](eefbr2::R) reader structure"]
impl crate::Readable for EEFBR2 {}
#[doc = "`write(|w| ..)` method takes [eefbr2::W](eefbr2::W) writer structure"]
impl crate::Writable for EEFBR2 {}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefbr2;
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstbr](rstbr) module"]
pub type RSTBR = crate::Reg<u32, _RSTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTBR;
#[doc = "`read()` method returns [rstbr::R](rstbr::R) reader structure"]
impl crate::Readable for RSTBR {}
#[doc = "`write(|w| ..)` method takes [rstbr::W](rstbr::W) writer structure"]
impl crate::Writable for RSTBR {}
#[doc = "TimerA Reset Register"]
pub mod rstbr;
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpbr](chpbr) module"]
pub type CHPBR = crate::Reg<u32, _CHPBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPBR;
#[doc = "`read()` method returns [chpbr::R](chpbr::R) reader structure"]
impl crate::Readable for CHPBR {}
#[doc = "`write(|w| ..)` method takes [chpbr::W](chpbr::W) writer structure"]
impl crate::Writable for CHPBR {}
#[doc = "Timerx Chopper Register"]
pub mod chpbr;
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1bcr](cpt1bcr) module"]
pub type CPT1BCR = crate::Reg<u32, _CPT1BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1BCR;
#[doc = "`read()` method returns [cpt1bcr::R](cpt1bcr::R) reader structure"]
impl crate::Readable for CPT1BCR {}
#[doc = "`write(|w| ..)` method takes [cpt1bcr::W](cpt1bcr::W) writer structure"]
impl crate::Writable for CPT1BCR {}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1bcr;
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2bcr](cpt2bcr) module"]
pub type CPT2BCR = crate::Reg<u32, _CPT2BCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2BCR;
#[doc = "`read()` method returns [cpt2bcr::R](cpt2bcr::R) reader structure"]
impl crate::Readable for CPT2BCR {}
#[doc = "`write(|w| ..)` method takes [cpt2bcr::W](cpt2bcr::W) writer structure"]
impl crate::Writable for CPT2BCR {}
#[doc = "CPT2xCR"]
pub mod cpt2bcr;
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outbr](outbr) module"]
pub type OUTBR = crate::Reg<u32, _OUTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTBR;
#[doc = "`read()` method returns [outbr::R](outbr::R) reader structure"]
impl crate::Readable for OUTBR {}
#[doc = "`write(|w| ..)` method takes [outbr::W](outbr::W) writer structure"]
impl crate::Writable for OUTBR {}
#[doc = "Timerx Output Register"]
pub mod outbr;
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltbr](fltbr) module"]
pub type FLTBR = crate::Reg<u32, _FLTBR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTBR;
#[doc = "`read()` method returns [fltbr::R](fltbr::R) reader structure"]
impl crate::Readable for FLTBR {}
#[doc = "`write(|w| ..)` method takes [fltbr::W](fltbr::W) writer structure"]
impl crate::Writable for FLTBR {}
#[doc = "Timerx Fault Register"]
pub mod fltbr;
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timbcr2](timbcr2) module"]
pub type TIMBCR2 = crate::Reg<u32, _TIMBCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMBCR2;
#[doc = "`read()` method returns [timbcr2::R](timbcr2::R) reader structure"]
impl crate::Readable for TIMBCR2 {}
#[doc = "`write(|w| ..)` method takes [timbcr2::W](timbcr2::W) writer structure"]
impl crate::Writable for TIMBCR2 {}
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timbcr2;
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [beefr3](beefr3) module"]
pub type BEEFR3 = crate::Reg<u32, _BEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BEEFR3;
#[doc = "`read()` method returns [beefr3::R](beefr3::R) reader structure"]
impl crate::Readable for BEEFR3 {}
#[doc = "`write(|w| ..)` method takes [beefr3::W](beefr3::W) writer structure"]
impl crate::Writable for BEEFR3 {}
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod beefr3;
