#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timccr: TIMCCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timcisr: TIMCISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timcicr: TIMCICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timcdier: TIMCDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntcr: CNTCR,
    #[doc = "0x14 - Timerx Period Register"]
    pub percr: PERCR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repcr: REPCR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1cr: CMP1CR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1ccr: CMP1CCR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2cr: CMP2CR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3cr: CMP3CR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4cr: CMP4CR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1cr: CPT1CR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2cr: CPT2CR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtcr: DTCR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setc1r: SETC1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rstc1r: RSTC1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setc2r: SETC2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstc2r: RSTC2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefcr1: EEFCR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefcr2: EEFCR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstcr: RSTCR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpcr: CHPCR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ccr: CPT1CCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ccr: CPT2CCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outcr: OUTCR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltcr: FLTCR,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timccr2: TIMCCR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub ceefr3: CEEFR3,
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timccr](timccr) module"]
pub type TIMCCR = crate::Reg<u32, _TIMCCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCCR;
#[doc = "`read()` method returns [timccr::R](timccr::R) reader structure"]
impl crate::Readable for TIMCCR {}
#[doc = "`write(|w| ..)` method takes [timccr::W](timccr::W) writer structure"]
impl crate::Writable for TIMCCR {}
#[doc = "Timerx Control Register"]
pub mod timccr;
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcisr](timcisr) module"]
pub type TIMCISR = crate::Reg<u32, _TIMCISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCISR;
#[doc = "`read()` method returns [timcisr::R](timcisr::R) reader structure"]
impl crate::Readable for TIMCISR {}
#[doc = "Timerx Interrupt Status Register"]
pub mod timcisr;
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcicr](timcicr) module"]
pub type TIMCICR = crate::Reg<u32, _TIMCICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCICR;
#[doc = "`write(|w| ..)` method takes [timcicr::W](timcicr::W) writer structure"]
impl crate::Writable for TIMCICR {}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timcicr;
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timcdier](timcdier) module"]
pub type TIMCDIER = crate::Reg<u32, _TIMCDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCDIER;
#[doc = "`read()` method returns [timcdier::R](timcdier::R) reader structure"]
impl crate::Readable for TIMCDIER {}
#[doc = "`write(|w| ..)` method takes [timcdier::W](timcdier::W) writer structure"]
impl crate::Writable for TIMCDIER {}
#[doc = "TIMxDIER"]
pub mod timcdier;
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntcr](cntcr) module"]
pub type CNTCR = crate::Reg<u32, _CNTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTCR;
#[doc = "`read()` method returns [cntcr::R](cntcr::R) reader structure"]
impl crate::Readable for CNTCR {}
#[doc = "`write(|w| ..)` method takes [cntcr::W](cntcr::W) writer structure"]
impl crate::Writable for CNTCR {}
#[doc = "Timerx Counter Register"]
pub mod cntcr;
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [percr](percr) module"]
pub type PERCR = crate::Reg<u32, _PERCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERCR;
#[doc = "`read()` method returns [percr::R](percr::R) reader structure"]
impl crate::Readable for PERCR {}
#[doc = "`write(|w| ..)` method takes [percr::W](percr::W) writer structure"]
impl crate::Writable for PERCR {}
#[doc = "Timerx Period Register"]
pub mod percr;
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repcr](repcr) module"]
pub type REPCR = crate::Reg<u32, _REPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPCR;
#[doc = "`read()` method returns [repcr::R](repcr::R) reader structure"]
impl crate::Readable for REPCR {}
#[doc = "`write(|w| ..)` method takes [repcr::W](repcr::W) writer structure"]
impl crate::Writable for REPCR {}
#[doc = "Timerx Repetition Register"]
pub mod repcr;
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1cr](cmp1cr) module"]
pub type CMP1CR = crate::Reg<u32, _CMP1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CR;
#[doc = "`read()` method returns [cmp1cr::R](cmp1cr::R) reader structure"]
impl crate::Readable for CMP1CR {}
#[doc = "`write(|w| ..)` method takes [cmp1cr::W](cmp1cr::W) writer structure"]
impl crate::Writable for CMP1CR {}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1cr;
#[doc = "Timerx Compare 1 Compound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1ccr](cmp1ccr) module"]
pub type CMP1CCR = crate::Reg<u32, _CMP1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CCR;
#[doc = "`read()` method returns [cmp1ccr::R](cmp1ccr::R) reader structure"]
impl crate::Readable for CMP1CCR {}
#[doc = "`write(|w| ..)` method takes [cmp1ccr::W](cmp1ccr::W) writer structure"]
impl crate::Writable for CMP1CCR {}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1ccr;
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2cr](cmp2cr) module"]
pub type CMP2CR = crate::Reg<u32, _CMP2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2CR;
#[doc = "`read()` method returns [cmp2cr::R](cmp2cr::R) reader structure"]
impl crate::Readable for CMP2CR {}
#[doc = "`write(|w| ..)` method takes [cmp2cr::W](cmp2cr::W) writer structure"]
impl crate::Writable for CMP2CR {}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2cr;
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3cr](cmp3cr) module"]
pub type CMP3CR = crate::Reg<u32, _CMP3CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3CR;
#[doc = "`read()` method returns [cmp3cr::R](cmp3cr::R) reader structure"]
impl crate::Readable for CMP3CR {}
#[doc = "`write(|w| ..)` method takes [cmp3cr::W](cmp3cr::W) writer structure"]
impl crate::Writable for CMP3CR {}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3cr;
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4cr](cmp4cr) module"]
pub type CMP4CR = crate::Reg<u32, _CMP4CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4CR;
#[doc = "`read()` method returns [cmp4cr::R](cmp4cr::R) reader structure"]
impl crate::Readable for CMP4CR {}
#[doc = "`write(|w| ..)` method takes [cmp4cr::W](cmp4cr::W) writer structure"]
impl crate::Writable for CMP4CR {}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4cr;
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1cr](cpt1cr) module"]
pub type CPT1CR = crate::Reg<u32, _CPT1CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1CR;
#[doc = "`read()` method returns [cpt1cr::R](cpt1cr::R) reader structure"]
impl crate::Readable for CPT1CR {}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1cr;
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2cr](cpt2cr) module"]
pub type CPT2CR = crate::Reg<u32, _CPT2CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2CR;
#[doc = "`read()` method returns [cpt2cr::R](cpt2cr::R) reader structure"]
impl crate::Readable for CPT2CR {}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2cr;
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcr](dtcr) module"]
pub type DTCR = crate::Reg<u32, _DTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTCR;
#[doc = "`read()` method returns [dtcr::R](dtcr::R) reader structure"]
impl crate::Readable for DTCR {}
#[doc = "`write(|w| ..)` method takes [dtcr::W](dtcr::W) writer structure"]
impl crate::Writable for DTCR {}
#[doc = "Timerx Deadtime Register"]
pub mod dtcr;
#[doc = "Timerx Output1 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setc1r](setc1r) module"]
pub type SETC1R = crate::Reg<u32, _SETC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETC1R;
#[doc = "`read()` method returns [setc1r::R](setc1r::R) reader structure"]
impl crate::Readable for SETC1R {}
#[doc = "`write(|w| ..)` method takes [setc1r::W](setc1r::W) writer structure"]
impl crate::Writable for SETC1R {}
#[doc = "Timerx Output1 Set Register"]
pub mod setc1r;
#[doc = "Timerx Output1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc1r](rstc1r) module"]
pub type RSTC1R = crate::Reg<u32, _RSTC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC1R;
#[doc = "`read()` method returns [rstc1r::R](rstc1r::R) reader structure"]
impl crate::Readable for RSTC1R {}
#[doc = "`write(|w| ..)` method takes [rstc1r::W](rstc1r::W) writer structure"]
impl crate::Writable for RSTC1R {}
#[doc = "Timerx Output1 Reset Register"]
pub mod rstc1r;
#[doc = "Timerx Output2 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setc2r](setc2r) module"]
pub type SETC2R = crate::Reg<u32, _SETC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETC2R;
#[doc = "`read()` method returns [setc2r::R](setc2r::R) reader structure"]
impl crate::Readable for SETC2R {}
#[doc = "`write(|w| ..)` method takes [setc2r::W](setc2r::W) writer structure"]
impl crate::Writable for SETC2R {}
#[doc = "Timerx Output2 Set Register"]
pub mod setc2r;
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstc2r](rstc2r) module"]
pub type RSTC2R = crate::Reg<u32, _RSTC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTC2R;
#[doc = "`read()` method returns [rstc2r::R](rstc2r::R) reader structure"]
impl crate::Readable for RSTC2R {}
#[doc = "`write(|w| ..)` method takes [rstc2r::W](rstc2r::W) writer structure"]
impl crate::Writable for RSTC2R {}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstc2r;
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefcr1](eefcr1) module"]
pub type EEFCR1 = crate::Reg<u32, _EEFCR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFCR1;
#[doc = "`read()` method returns [eefcr1::R](eefcr1::R) reader structure"]
impl crate::Readable for EEFCR1 {}
#[doc = "`write(|w| ..)` method takes [eefcr1::W](eefcr1::W) writer structure"]
impl crate::Writable for EEFCR1 {}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefcr1;
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefcr2](eefcr2) module"]
pub type EEFCR2 = crate::Reg<u32, _EEFCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFCR2;
#[doc = "`read()` method returns [eefcr2::R](eefcr2::R) reader structure"]
impl crate::Readable for EEFCR2 {}
#[doc = "`write(|w| ..)` method takes [eefcr2::W](eefcr2::W) writer structure"]
impl crate::Writable for EEFCR2 {}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefcr2;
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcr](rstcr) module"]
pub type RSTCR = crate::Reg<u32, _RSTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTCR;
#[doc = "`read()` method returns [rstcr::R](rstcr::R) reader structure"]
impl crate::Readable for RSTCR {}
#[doc = "`write(|w| ..)` method takes [rstcr::W](rstcr::W) writer structure"]
impl crate::Writable for RSTCR {}
#[doc = "TimerA Reset Register"]
pub mod rstcr;
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpcr](chpcr) module"]
pub type CHPCR = crate::Reg<u32, _CHPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPCR;
#[doc = "`read()` method returns [chpcr::R](chpcr::R) reader structure"]
impl crate::Readable for CHPCR {}
#[doc = "`write(|w| ..)` method takes [chpcr::W](chpcr::W) writer structure"]
impl crate::Writable for CHPCR {}
#[doc = "Timerx Chopper Register"]
pub mod chpcr;
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1ccr](cpt1ccr) module"]
pub type CPT1CCR = crate::Reg<u32, _CPT1CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1CCR;
#[doc = "`read()` method returns [cpt1ccr::R](cpt1ccr::R) reader structure"]
impl crate::Readable for CPT1CCR {}
#[doc = "`write(|w| ..)` method takes [cpt1ccr::W](cpt1ccr::W) writer structure"]
impl crate::Writable for CPT1CCR {}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ccr;
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2ccr](cpt2ccr) module"]
pub type CPT2CCR = crate::Reg<u32, _CPT2CCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2CCR;
#[doc = "`read()` method returns [cpt2ccr::R](cpt2ccr::R) reader structure"]
impl crate::Readable for CPT2CCR {}
#[doc = "`write(|w| ..)` method takes [cpt2ccr::W](cpt2ccr::W) writer structure"]
impl crate::Writable for CPT2CCR {}
#[doc = "CPT2xCR"]
pub mod cpt2ccr;
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outcr](outcr) module"]
pub type OUTCR = crate::Reg<u32, _OUTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTCR;
#[doc = "`read()` method returns [outcr::R](outcr::R) reader structure"]
impl crate::Readable for OUTCR {}
#[doc = "`write(|w| ..)` method takes [outcr::W](outcr::W) writer structure"]
impl crate::Writable for OUTCR {}
#[doc = "Timerx Output Register"]
pub mod outcr;
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltcr](fltcr) module"]
pub type FLTCR = crate::Reg<u32, _FLTCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTCR;
#[doc = "`read()` method returns [fltcr::R](fltcr::R) reader structure"]
impl crate::Readable for FLTCR {}
#[doc = "`write(|w| ..)` method takes [fltcr::W](fltcr::W) writer structure"]
impl crate::Writable for FLTCR {}
#[doc = "Timerx Fault Register"]
pub mod fltcr;
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timccr2](timccr2) module"]
pub type TIMCCR2 = crate::Reg<u32, _TIMCCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMCCR2;
#[doc = "`read()` method returns [timccr2::R](timccr2::R) reader structure"]
impl crate::Readable for TIMCCR2 {}
#[doc = "`write(|w| ..)` method takes [timccr2::W](timccr2::W) writer structure"]
impl crate::Writable for TIMCCR2 {}
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timccr2;
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ceefr3](ceefr3) module"]
pub type CEEFR3 = crate::Reg<u32, _CEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CEEFR3;
#[doc = "`read()` method returns [ceefr3::R](ceefr3::R) reader structure"]
impl crate::Readable for CEEFR3 {}
#[doc = "`write(|w| ..)` method takes [ceefr3::W](ceefr3::W) writer structure"]
impl crate::Writable for CEEFR3 {}
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod ceefr3;
