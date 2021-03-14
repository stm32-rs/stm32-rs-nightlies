#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timfcr: TIMFCR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timfisr: TIMFISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timficr: TIMFICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timfdier: TIMFDIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntfr: CNTFR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perfr: PERFR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repfr: REPFR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1fr: CMP1FR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1cfr: CMP1CFR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2fr: CMP2FR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3fr: CMP3FR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4fr: CMP4FR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1fr: CPT1FR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2fr: CPT2FR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtfr: DTFR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub setf1r: SETF1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rste1r: RSTE1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub setf2r: SETF2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rstf2r: RSTF2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eeffr1: EEFFR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eeffr2: EEFFR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstfr: RSTFR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpfr: CHPFR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1fcr: CPT1FCR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2fcr: CPT2FCR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outfr: OUTFR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltfr: FLTFR,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timfcr2: TIMFCR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub feefr3: FEEFR3,
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timfcr](timfcr) module"]
pub type TIMFCR = crate::Reg<u32, _TIMFCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFCR;
#[doc = "`read()` method returns [timfcr::R](timfcr::R) reader structure"]
impl crate::Readable for TIMFCR {}
#[doc = "`write(|w| ..)` method takes [timfcr::W](timfcr::W) writer structure"]
impl crate::Writable for TIMFCR {}
#[doc = "Timerx Control Register"]
pub mod timfcr;
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timfisr](timfisr) module"]
pub type TIMFISR = crate::Reg<u32, _TIMFISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFISR;
#[doc = "`read()` method returns [timfisr::R](timfisr::R) reader structure"]
impl crate::Readable for TIMFISR {}
#[doc = "Timerx Interrupt Status Register"]
pub mod timfisr;
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timficr](timficr) module"]
pub type TIMFICR = crate::Reg<u32, _TIMFICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFICR;
#[doc = "`write(|w| ..)` method takes [timficr::W](timficr::W) writer structure"]
impl crate::Writable for TIMFICR {}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timficr;
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timfdier](timfdier) module"]
pub type TIMFDIER = crate::Reg<u32, _TIMFDIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFDIER;
#[doc = "`read()` method returns [timfdier::R](timfdier::R) reader structure"]
impl crate::Readable for TIMFDIER {}
#[doc = "`write(|w| ..)` method takes [timfdier::W](timfdier::W) writer structure"]
impl crate::Writable for TIMFDIER {}
#[doc = "TIMxDIER"]
pub mod timfdier;
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntfr](cntfr) module"]
pub type CNTFR = crate::Reg<u32, _CNTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTFR;
#[doc = "`read()` method returns [cntfr::R](cntfr::R) reader structure"]
impl crate::Readable for CNTFR {}
#[doc = "`write(|w| ..)` method takes [cntfr::W](cntfr::W) writer structure"]
impl crate::Writable for CNTFR {}
#[doc = "Timerx Counter Register"]
pub mod cntfr;
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perfr](perfr) module"]
pub type PERFR = crate::Reg<u32, _PERFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERFR;
#[doc = "`read()` method returns [perfr::R](perfr::R) reader structure"]
impl crate::Readable for PERFR {}
#[doc = "`write(|w| ..)` method takes [perfr::W](perfr::W) writer structure"]
impl crate::Writable for PERFR {}
#[doc = "Timerx Period Register"]
pub mod perfr;
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repfr](repfr) module"]
pub type REPFR = crate::Reg<u32, _REPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPFR;
#[doc = "`read()` method returns [repfr::R](repfr::R) reader structure"]
impl crate::Readable for REPFR {}
#[doc = "`write(|w| ..)` method takes [repfr::W](repfr::W) writer structure"]
impl crate::Writable for REPFR {}
#[doc = "Timerx Repetition Register"]
pub mod repfr;
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1fr](cmp1fr) module"]
pub type CMP1FR = crate::Reg<u32, _CMP1FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1FR;
#[doc = "`read()` method returns [cmp1fr::R](cmp1fr::R) reader structure"]
impl crate::Readable for CMP1FR {}
#[doc = "`write(|w| ..)` method takes [cmp1fr::W](cmp1fr::W) writer structure"]
impl crate::Writable for CMP1FR {}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1fr;
#[doc = "Timerx Compare 1 Compound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1cfr](cmp1cfr) module"]
pub type CMP1CFR = crate::Reg<u32, _CMP1CFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CFR;
#[doc = "`read()` method returns [cmp1cfr::R](cmp1cfr::R) reader structure"]
impl crate::Readable for CMP1CFR {}
#[doc = "`write(|w| ..)` method takes [cmp1cfr::W](cmp1cfr::W) writer structure"]
impl crate::Writable for CMP1CFR {}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1cfr;
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2fr](cmp2fr) module"]
pub type CMP2FR = crate::Reg<u32, _CMP2FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2FR;
#[doc = "`read()` method returns [cmp2fr::R](cmp2fr::R) reader structure"]
impl crate::Readable for CMP2FR {}
#[doc = "`write(|w| ..)` method takes [cmp2fr::W](cmp2fr::W) writer structure"]
impl crate::Writable for CMP2FR {}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2fr;
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3fr](cmp3fr) module"]
pub type CMP3FR = crate::Reg<u32, _CMP3FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3FR;
#[doc = "`read()` method returns [cmp3fr::R](cmp3fr::R) reader structure"]
impl crate::Readable for CMP3FR {}
#[doc = "`write(|w| ..)` method takes [cmp3fr::W](cmp3fr::W) writer structure"]
impl crate::Writable for CMP3FR {}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3fr;
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4fr](cmp4fr) module"]
pub type CMP4FR = crate::Reg<u32, _CMP4FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4FR;
#[doc = "`read()` method returns [cmp4fr::R](cmp4fr::R) reader structure"]
impl crate::Readable for CMP4FR {}
#[doc = "`write(|w| ..)` method takes [cmp4fr::W](cmp4fr::W) writer structure"]
impl crate::Writable for CMP4FR {}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4fr;
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1fr](cpt1fr) module"]
pub type CPT1FR = crate::Reg<u32, _CPT1FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1FR;
#[doc = "`read()` method returns [cpt1fr::R](cpt1fr::R) reader structure"]
impl crate::Readable for CPT1FR {}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1fr;
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2fr](cpt2fr) module"]
pub type CPT2FR = crate::Reg<u32, _CPT2FR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2FR;
#[doc = "`read()` method returns [cpt2fr::R](cpt2fr::R) reader structure"]
impl crate::Readable for CPT2FR {}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2fr;
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfr](dtfr) module"]
pub type DTFR = crate::Reg<u32, _DTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTFR;
#[doc = "`read()` method returns [dtfr::R](dtfr::R) reader structure"]
impl crate::Readable for DTFR {}
#[doc = "`write(|w| ..)` method takes [dtfr::W](dtfr::W) writer structure"]
impl crate::Writable for DTFR {}
#[doc = "Timerx Deadtime Register"]
pub mod dtfr;
#[doc = "Timerx Output1 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setf1r](setf1r) module"]
pub type SETF1R = crate::Reg<u32, _SETF1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETF1R;
#[doc = "`read()` method returns [setf1r::R](setf1r::R) reader structure"]
impl crate::Readable for SETF1R {}
#[doc = "`write(|w| ..)` method takes [setf1r::W](setf1r::W) writer structure"]
impl crate::Writable for SETF1R {}
#[doc = "Timerx Output1 Set Register"]
pub mod setf1r;
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
#[doc = "Timerx Output2 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [setf2r](setf2r) module"]
pub type SETF2R = crate::Reg<u32, _SETF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETF2R;
#[doc = "`read()` method returns [setf2r::R](setf2r::R) reader structure"]
impl crate::Readable for SETF2R {}
#[doc = "`write(|w| ..)` method takes [setf2r::W](setf2r::W) writer structure"]
impl crate::Writable for SETF2R {}
#[doc = "Timerx Output2 Set Register"]
pub mod setf2r;
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstf2r](rstf2r) module"]
pub type RSTF2R = crate::Reg<u32, _RSTF2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTF2R;
#[doc = "`read()` method returns [rstf2r::R](rstf2r::R) reader structure"]
impl crate::Readable for RSTF2R {}
#[doc = "`write(|w| ..)` method takes [rstf2r::W](rstf2r::W) writer structure"]
impl crate::Writable for RSTF2R {}
#[doc = "Timerx Output2 Reset Register"]
pub mod rstf2r;
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeffr1](eeffr1) module"]
pub type EEFFR1 = crate::Reg<u32, _EEFFR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFFR1;
#[doc = "`read()` method returns [eeffr1::R](eeffr1::R) reader structure"]
impl crate::Readable for EEFFR1 {}
#[doc = "`write(|w| ..)` method takes [eeffr1::W](eeffr1::W) writer structure"]
impl crate::Writable for EEFFR1 {}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eeffr1;
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eeffr2](eeffr2) module"]
pub type EEFFR2 = crate::Reg<u32, _EEFFR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFFR2;
#[doc = "`read()` method returns [eeffr2::R](eeffr2::R) reader structure"]
impl crate::Readable for EEFFR2 {}
#[doc = "`write(|w| ..)` method takes [eeffr2::W](eeffr2::W) writer structure"]
impl crate::Writable for EEFFR2 {}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eeffr2;
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstfr](rstfr) module"]
pub type RSTFR = crate::Reg<u32, _RSTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTFR;
#[doc = "`read()` method returns [rstfr::R](rstfr::R) reader structure"]
impl crate::Readable for RSTFR {}
#[doc = "`write(|w| ..)` method takes [rstfr::W](rstfr::W) writer structure"]
impl crate::Writable for RSTFR {}
#[doc = "TimerA Reset Register"]
pub mod rstfr;
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpfr](chpfr) module"]
pub type CHPFR = crate::Reg<u32, _CHPFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPFR;
#[doc = "`read()` method returns [chpfr::R](chpfr::R) reader structure"]
impl crate::Readable for CHPFR {}
#[doc = "`write(|w| ..)` method takes [chpfr::W](chpfr::W) writer structure"]
impl crate::Writable for CHPFR {}
#[doc = "Timerx Chopper Register"]
pub mod chpfr;
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1fcr](cpt1fcr) module"]
pub type CPT1FCR = crate::Reg<u32, _CPT1FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1FCR;
#[doc = "`read()` method returns [cpt1fcr::R](cpt1fcr::R) reader structure"]
impl crate::Readable for CPT1FCR {}
#[doc = "`write(|w| ..)` method takes [cpt1fcr::W](cpt1fcr::W) writer structure"]
impl crate::Writable for CPT1FCR {}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1fcr;
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2fcr](cpt2fcr) module"]
pub type CPT2FCR = crate::Reg<u32, _CPT2FCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2FCR;
#[doc = "`read()` method returns [cpt2fcr::R](cpt2fcr::R) reader structure"]
impl crate::Readable for CPT2FCR {}
#[doc = "`write(|w| ..)` method takes [cpt2fcr::W](cpt2fcr::W) writer structure"]
impl crate::Writable for CPT2FCR {}
#[doc = "CPT2xCR"]
pub mod cpt2fcr;
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outfr](outfr) module"]
pub type OUTFR = crate::Reg<u32, _OUTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTFR;
#[doc = "`read()` method returns [outfr::R](outfr::R) reader structure"]
impl crate::Readable for OUTFR {}
#[doc = "`write(|w| ..)` method takes [outfr::W](outfr::W) writer structure"]
impl crate::Writable for OUTFR {}
#[doc = "Timerx Output Register"]
pub mod outfr;
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltfr](fltfr) module"]
pub type FLTFR = crate::Reg<u32, _FLTFR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTFR;
#[doc = "`read()` method returns [fltfr::R](fltfr::R) reader structure"]
impl crate::Readable for FLTFR {}
#[doc = "`write(|w| ..)` method takes [fltfr::W](fltfr::W) writer structure"]
impl crate::Writable for FLTFR {}
#[doc = "Timerx Fault Register"]
pub mod fltfr;
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timfcr2](timfcr2) module"]
pub type TIMFCR2 = crate::Reg<u32, _TIMFCR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMFCR2;
#[doc = "`read()` method returns [timfcr2::R](timfcr2::R) reader structure"]
impl crate::Readable for TIMFCR2 {}
#[doc = "`write(|w| ..)` method takes [timfcr2::W](timfcr2::W) writer structure"]
impl crate::Writable for TIMFCR2 {}
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timfcr2;
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [feefr3](feefr3) module"]
pub type FEEFR3 = crate::Reg<u32, _FEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FEEFR3;
#[doc = "`read()` method returns [feefr3::R](feefr3::R) reader structure"]
impl crate::Readable for FEEFR3 {}
#[doc = "`write(|w| ..)` method takes [feefr3::W](feefr3::W) writer structure"]
impl crate::Writable for FEEFR3 {}
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod feefr3;
