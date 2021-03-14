#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timecr: TIMECR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timeisr: TIMEISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timeicr: TIMEICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timedier: TIMEDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cnter: CNTER,
    #[doc = "0x14 - Timerx Period Register"]
    pub perer: PERER,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub reper: REPER,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1er: CMP1ER,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cer: CMP1CER,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2er: CMP2ER,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3er: CMP3ER,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4er: CMP4ER,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1er: CPT1ER,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2er: CPT2ER,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dter: DTER,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub sete1r: SETE1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rste1r: RSTE1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub sete2r: SETE2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rste2r: RSTE2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefer1: EEFER1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefer2: EEFER2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rster: RSTER,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chper: CHPER,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1ecr: CPT1ECR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2ecr: CPT2ECR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outer: OUTER,
    #[doc = "0x68 - Timerx Fault Register"]
    pub flter: FLTER,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timecr2: TIMECR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub eeefr3: EEEFR3,
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecr](timecr) module"]
pub type TIMECR = crate::Reg<u32, _TIMECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECR;
#[doc = "`read()` method returns [timecr::R](timecr::R) reader structure"]
impl crate::Readable for TIMECR {}
#[doc = "`write(|w| ..)` method takes [timecr::W](timecr::W) writer structure"]
impl crate::Writable for TIMECR {}
#[doc = "Timerx Control Register"]
pub mod timecr;
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeisr](timeisr) module"]
pub type TIMEISR = crate::Reg<u32, _TIMEISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEISR;
#[doc = "`read()` method returns [timeisr::R](timeisr::R) reader structure"]
impl crate::Readable for TIMEISR {}
#[doc = "Timerx Interrupt Status Register"]
pub mod timeisr;
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeicr](timeicr) module"]
pub type TIMEICR = crate::Reg<u32, _TIMEICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEICR;
#[doc = "`write(|w| ..)` method takes [timeicr::W](timeicr::W) writer structure"]
impl crate::Writable for TIMEICR {}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timeicr;
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timedier](timedier) module"]
pub type TIMEDIER = crate::Reg<u32, _TIMEDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMEDIER;
#[doc = "`read()` method returns [timedier::R](timedier::R) reader structure"]
impl crate::Readable for TIMEDIER {}
#[doc = "`write(|w| ..)` method takes [timedier::W](timedier::W) writer structure"]
impl crate::Writable for TIMEDIER {}
#[doc = "TIMxDIER"]
pub mod timedier;
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnter](cnter) module"]
pub type CNTER = crate::Reg<u32, _CNTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTER;
#[doc = "`read()` method returns [cnter::R](cnter::R) reader structure"]
impl crate::Readable for CNTER {}
#[doc = "`write(|w| ..)` method takes [cnter::W](cnter::W) writer structure"]
impl crate::Writable for CNTER {}
#[doc = "Timerx Counter Register"]
pub mod cnter;
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perer](perer) module"]
pub type PERER = crate::Reg<u32, _PERER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERER;
#[doc = "`read()` method returns [perer::R](perer::R) reader structure"]
impl crate::Readable for PERER {}
#[doc = "`write(|w| ..)` method takes [perer::W](perer::W) writer structure"]
impl crate::Writable for PERER {}
#[doc = "Timerx Period Register"]
pub mod perer;
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reper](reper) module"]
pub type REPER = crate::Reg<u32, _REPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPER;
#[doc = "`read()` method returns [reper::R](reper::R) reader structure"]
impl crate::Readable for REPER {}
#[doc = "`write(|w| ..)` method takes [reper::W](reper::W) writer structure"]
impl crate::Writable for REPER {}
#[doc = "Timerx Repetition Register"]
pub mod reper;
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1er](cmp1er) module"]
pub type CMP1ER = crate::Reg<u32, _CMP1ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1ER;
#[doc = "`read()` method returns [cmp1er::R](cmp1er::R) reader structure"]
impl crate::Readable for CMP1ER {}
#[doc = "`write(|w| ..)` method takes [cmp1er::W](cmp1er::W) writer structure"]
impl crate::Writable for CMP1ER {}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1er;
#[doc = "Timerx Compare 1 Compound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1cer](cmp1cer) module"]
pub type CMP1CER = crate::Reg<u32, _CMP1CER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CER;
#[doc = "`read()` method returns [cmp1cer::R](cmp1cer::R) reader structure"]
impl crate::Readable for CMP1CER {}
#[doc = "`write(|w| ..)` method takes [cmp1cer::W](cmp1cer::W) writer structure"]
impl crate::Writable for CMP1CER {}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cer;
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2er](cmp2er) module"]
pub type CMP2ER = crate::Reg<u32, _CMP2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2ER;
#[doc = "`read()` method returns [cmp2er::R](cmp2er::R) reader structure"]
impl crate::Readable for CMP2ER {}
#[doc = "`write(|w| ..)` method takes [cmp2er::W](cmp2er::W) writer structure"]
impl crate::Writable for CMP2ER {}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2er;
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3er](cmp3er) module"]
pub type CMP3ER = crate::Reg<u32, _CMP3ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3ER;
#[doc = "`read()` method returns [cmp3er::R](cmp3er::R) reader structure"]
impl crate::Readable for CMP3ER {}
#[doc = "`write(|w| ..)` method takes [cmp3er::W](cmp3er::W) writer structure"]
impl crate::Writable for CMP3ER {}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3er;
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4er](cmp4er) module"]
pub type CMP4ER = crate::Reg<u32, _CMP4ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4ER;
#[doc = "`read()` method returns [cmp4er::R](cmp4er::R) reader structure"]
impl crate::Readable for CMP4ER {}
#[doc = "`write(|w| ..)` method takes [cmp4er::W](cmp4er::W) writer structure"]
impl crate::Writable for CMP4ER {}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4er;
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1er](cpt1er) module"]
pub type CPT1ER = crate::Reg<u32, _CPT1ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ER;
#[doc = "`read()` method returns [cpt1er::R](cpt1er::R) reader structure"]
impl crate::Readable for CPT1ER {}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1er;
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2er](cpt2er) module"]
pub type CPT2ER = crate::Reg<u32, _CPT2ER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ER;
#[doc = "`read()` method returns [cpt2er::R](cpt2er::R) reader structure"]
impl crate::Readable for CPT2ER {}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2er;
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dter](dter) module"]
pub type DTER = crate::Reg<u32, _DTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTER;
#[doc = "`read()` method returns [dter::R](dter::R) reader structure"]
impl crate::Readable for DTER {}
#[doc = "`write(|w| ..)` method takes [dter::W](dter::W) writer structure"]
impl crate::Writable for DTER {}
#[doc = "Timerx Deadtime Register"]
pub mod dter;
#[doc = "Timerx Output1 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sete1r](sete1r) module"]
pub type SETE1R = crate::Reg<u32, _SETE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETE1R;
#[doc = "`read()` method returns [sete1r::R](sete1r::R) reader structure"]
impl crate::Readable for SETE1R {}
#[doc = "`write(|w| ..)` method takes [sete1r::W](sete1r::W) writer structure"]
impl crate::Writable for SETE1R {}
#[doc = "Timerx Output1 Set Register"]
pub mod sete1r;
#[doc = "Timerx Output1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rste1r](rste1r) module"]
pub type RSTE1R = crate::Reg<u32, _RSTE1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTE1R;
#[doc = "`read()` method returns [rste1r::R](rste1r::R) reader structure"]
impl crate::Readable for RSTE1R {}
#[doc = "`write(|w| ..)` method takes [rste1r::W](rste1r::W) writer structure"]
impl crate::Writable for RSTE1R {}
#[doc = "Timerx Output1 Reset Register"]
pub mod rste1r;
#[doc = "Timerx Output2 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sete2r](sete2r) module"]
pub type SETE2R = crate::Reg<u32, _SETE2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETE2R;
#[doc = "`read()` method returns [sete2r::R](sete2r::R) reader structure"]
impl crate::Readable for SETE2R {}
#[doc = "`write(|w| ..)` method takes [sete2r::W](sete2r::W) writer structure"]
impl crate::Writable for SETE2R {}
#[doc = "Timerx Output2 Set Register"]
pub mod sete2r;
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rste2r](rste2r) module"]
pub type RSTE2R = crate::Reg<u32, _RSTE2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTE2R;
#[doc = "`read()` method returns [rste2r::R](rste2r::R) reader structure"]
impl crate::Readable for RSTE2R {}
#[doc = "`write(|w| ..)` method takes [rste2r::W](rste2r::W) writer structure"]
impl crate::Writable for RSTE2R {}
#[doc = "Timerx Output2 Reset Register"]
pub mod rste2r;
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefer1](eefer1) module"]
pub type EEFER1 = crate::Reg<u32, _EEFER1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFER1;
#[doc = "`read()` method returns [eefer1::R](eefer1::R) reader structure"]
impl crate::Readable for EEFER1 {}
#[doc = "`write(|w| ..)` method takes [eefer1::W](eefer1::W) writer structure"]
impl crate::Writable for EEFER1 {}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefer1;
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefer2](eefer2) module"]
pub type EEFER2 = crate::Reg<u32, _EEFER2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFER2;
#[doc = "`read()` method returns [eefer2::R](eefer2::R) reader structure"]
impl crate::Readable for EEFER2 {}
#[doc = "`write(|w| ..)` method takes [eefer2::W](eefer2::W) writer structure"]
impl crate::Writable for EEFER2 {}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefer2;
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rster](rster) module"]
pub type RSTER = crate::Reg<u32, _RSTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTER;
#[doc = "`read()` method returns [rster::R](rster::R) reader structure"]
impl crate::Readable for RSTER {}
#[doc = "`write(|w| ..)` method takes [rster::W](rster::W) writer structure"]
impl crate::Writable for RSTER {}
#[doc = "TimerA Reset Register"]
pub mod rster;
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chper](chper) module"]
pub type CHPER = crate::Reg<u32, _CHPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPER;
#[doc = "`read()` method returns [chper::R](chper::R) reader structure"]
impl crate::Readable for CHPER {}
#[doc = "`write(|w| ..)` method takes [chper::W](chper::W) writer structure"]
impl crate::Writable for CHPER {}
#[doc = "Timerx Chopper Register"]
pub mod chper;
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1ecr](cpt1ecr) module"]
pub type CPT1ECR = crate::Reg<u32, _CPT1ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ECR;
#[doc = "`read()` method returns [cpt1ecr::R](cpt1ecr::R) reader structure"]
impl crate::Readable for CPT1ECR {}
#[doc = "`write(|w| ..)` method takes [cpt1ecr::W](cpt1ecr::W) writer structure"]
impl crate::Writable for CPT1ECR {}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1ecr;
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2ecr](cpt2ecr) module"]
pub type CPT2ECR = crate::Reg<u32, _CPT2ECR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ECR;
#[doc = "`read()` method returns [cpt2ecr::R](cpt2ecr::R) reader structure"]
impl crate::Readable for CPT2ECR {}
#[doc = "`write(|w| ..)` method takes [cpt2ecr::W](cpt2ecr::W) writer structure"]
impl crate::Writable for CPT2ECR {}
#[doc = "CPT2xCR"]
pub mod cpt2ecr;
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outer](outer) module"]
pub type OUTER = crate::Reg<u32, _OUTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTER;
#[doc = "`read()` method returns [outer::R](outer::R) reader structure"]
impl crate::Readable for OUTER {}
#[doc = "`write(|w| ..)` method takes [outer::W](outer::W) writer structure"]
impl crate::Writable for OUTER {}
#[doc = "Timerx Output Register"]
pub mod outer;
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flter](flter) module"]
pub type FLTER = crate::Reg<u32, _FLTER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTER;
#[doc = "`read()` method returns [flter::R](flter::R) reader structure"]
impl crate::Readable for FLTER {}
#[doc = "`write(|w| ..)` method takes [flter::W](flter::W) writer structure"]
impl crate::Writable for FLTER {}
#[doc = "Timerx Fault Register"]
pub mod flter;
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timecr2](timecr2) module"]
pub type TIMECR2 = crate::Reg<u32, _TIMECR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMECR2;
#[doc = "`read()` method returns [timecr2::R](timecr2::R) reader structure"]
impl crate::Readable for TIMECR2 {}
#[doc = "`write(|w| ..)` method takes [timecr2::W](timecr2::W) writer structure"]
impl crate::Writable for TIMECR2 {}
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timecr2;
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeefr3](eeefr3) module"]
pub type EEEFR3 = crate::Reg<u32, _EEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEEFR3;
#[doc = "`read()` method returns [eeefr3::R](eeefr3::R) reader structure"]
impl crate::Readable for EEEFR3 {}
#[doc = "`write(|w| ..)` method takes [eeefr3::W](eeefr3::W) writer structure"]
impl crate::Writable for EEEFR3 {}
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod eeefr3;
