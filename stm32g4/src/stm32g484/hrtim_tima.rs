#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Timerx Control Register"]
    pub timacr: TIMACR,
    #[doc = "0x04 - Timerx Interrupt Status Register"]
    pub timaisr: TIMAISR,
    #[doc = "0x08 - Timerx Interrupt Clear Register"]
    pub timaicr: TIMAICR,
    #[doc = "0x0c - TIMxDIER"]
    pub timadier: TIMADIER,
    #[doc = "0x10 - Timerx Counter Register"]
    pub cntar: CNTAR,
    #[doc = "0x14 - Timerx Period Register"]
    pub perar: PERAR,
    #[doc = "0x18 - Timerx Repetition Register"]
    pub repar: REPAR,
    #[doc = "0x1c - Timerx Compare 1 Register"]
    pub cmp1ar: CMP1AR,
    #[doc = "0x20 - Timerx Compare 1 Compound Register"]
    pub cmp1car: CMP1CAR,
    #[doc = "0x24 - Timerx Compare 2 Register"]
    pub cmp2ar: CMP2AR,
    #[doc = "0x28 - Timerx Compare 3 Register"]
    pub cmp3ar: CMP3AR,
    #[doc = "0x2c - Timerx Compare 4 Register"]
    pub cmp4ar: CMP4AR,
    #[doc = "0x30 - Timerx Capture 1 Register"]
    pub cpt1ar: CPT1AR,
    #[doc = "0x34 - Timerx Capture 2 Register"]
    pub cpt2ar: CPT2AR,
    #[doc = "0x38 - Timerx Deadtime Register"]
    pub dtar: DTAR,
    #[doc = "0x3c - Timerx Output1 Set Register"]
    pub seta1r: SETA1R,
    #[doc = "0x40 - Timerx Output1 Reset Register"]
    pub rsta1r: RSTA1R,
    #[doc = "0x44 - Timerx Output2 Set Register"]
    pub seta2r: SETA2R,
    #[doc = "0x48 - Timerx Output2 Reset Register"]
    pub rsta2r: RSTA2R,
    #[doc = "0x4c - Timerx External Event Filtering Register 1"]
    pub eefar1: EEFAR1,
    #[doc = "0x50 - Timerx External Event Filtering Register 2"]
    pub eefar2: EEFAR2,
    #[doc = "0x54 - TimerA Reset Register"]
    pub rstar: RSTAR,
    #[doc = "0x58 - Timerx Chopper Register"]
    pub chpar: CHPAR,
    #[doc = "0x5c - Timerx Capture 2 Control Register"]
    pub cpt1acr: CPT1ACR,
    #[doc = "0x60 - CPT2xCR"]
    pub cpt2acr: CPT2ACR,
    #[doc = "0x64 - Timerx Output Register"]
    pub outar: OUTAR,
    #[doc = "0x68 - Timerx Fault Register"]
    pub fltar: FLTAR,
    #[doc = "0x6c - HRTIM Timerx Control Register 2"]
    pub timacr2: TIMACR2,
    #[doc = "0x70 - HRTIM Timerx External Event Filtering Register 3"]
    pub aeefr3: AEEFR3,
}
#[doc = "Timerx Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timacr](timacr) module"]
pub type TIMACR = crate::Reg<u32, _TIMACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMACR;
#[doc = "`read()` method returns [timacr::R](timacr::R) reader structure"]
impl crate::Readable for TIMACR {}
#[doc = "`write(|w| ..)` method takes [timacr::W](timacr::W) writer structure"]
impl crate::Writable for TIMACR {}
#[doc = "Timerx Control Register"]
pub mod timacr;
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timaisr](timaisr) module"]
pub type TIMAISR = crate::Reg<u32, _TIMAISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMAISR;
#[doc = "`read()` method returns [timaisr::R](timaisr::R) reader structure"]
impl crate::Readable for TIMAISR {}
#[doc = "Timerx Interrupt Status Register"]
pub mod timaisr;
#[doc = "Timerx Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timaicr](timaicr) module"]
pub type TIMAICR = crate::Reg<u32, _TIMAICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMAICR;
#[doc = "`write(|w| ..)` method takes [timaicr::W](timaicr::W) writer structure"]
impl crate::Writable for TIMAICR {}
#[doc = "Timerx Interrupt Clear Register"]
pub mod timaicr;
#[doc = "TIMxDIER\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timadier](timadier) module"]
pub type TIMADIER = crate::Reg<u32, _TIMADIER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMADIER;
#[doc = "`read()` method returns [timadier::R](timadier::R) reader structure"]
impl crate::Readable for TIMADIER {}
#[doc = "`write(|w| ..)` method takes [timadier::W](timadier::W) writer structure"]
impl crate::Writable for TIMADIER {}
#[doc = "TIMxDIER"]
pub mod timadier;
#[doc = "Timerx Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cntar](cntar) module"]
pub type CNTAR = crate::Reg<u32, _CNTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CNTAR;
#[doc = "`read()` method returns [cntar::R](cntar::R) reader structure"]
impl crate::Readable for CNTAR {}
#[doc = "`write(|w| ..)` method takes [cntar::W](cntar::W) writer structure"]
impl crate::Writable for CNTAR {}
#[doc = "Timerx Counter Register"]
pub mod cntar;
#[doc = "Timerx Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perar](perar) module"]
pub type PERAR = crate::Reg<u32, _PERAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PERAR;
#[doc = "`read()` method returns [perar::R](perar::R) reader structure"]
impl crate::Readable for PERAR {}
#[doc = "`write(|w| ..)` method takes [perar::W](perar::W) writer structure"]
impl crate::Writable for PERAR {}
#[doc = "Timerx Period Register"]
pub mod perar;
#[doc = "Timerx Repetition Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [repar](repar) module"]
pub type REPAR = crate::Reg<u32, _REPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _REPAR;
#[doc = "`read()` method returns [repar::R](repar::R) reader structure"]
impl crate::Readable for REPAR {}
#[doc = "`write(|w| ..)` method takes [repar::W](repar::W) writer structure"]
impl crate::Writable for REPAR {}
#[doc = "Timerx Repetition Register"]
pub mod repar;
#[doc = "Timerx Compare 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1ar](cmp1ar) module"]
pub type CMP1AR = crate::Reg<u32, _CMP1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1AR;
#[doc = "`read()` method returns [cmp1ar::R](cmp1ar::R) reader structure"]
impl crate::Readable for CMP1AR {}
#[doc = "`write(|w| ..)` method takes [cmp1ar::W](cmp1ar::W) writer structure"]
impl crate::Writable for CMP1AR {}
#[doc = "Timerx Compare 1 Register"]
pub mod cmp1ar;
#[doc = "Timerx Compare 1 Compound Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1car](cmp1car) module"]
pub type CMP1CAR = crate::Reg<u32, _CMP1CAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP1CAR;
#[doc = "`read()` method returns [cmp1car::R](cmp1car::R) reader structure"]
impl crate::Readable for CMP1CAR {}
#[doc = "`write(|w| ..)` method takes [cmp1car::W](cmp1car::W) writer structure"]
impl crate::Writable for CMP1CAR {}
#[doc = "Timerx Compare 1 Compound Register"]
pub mod cmp1car;
#[doc = "Timerx Compare 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp2ar](cmp2ar) module"]
pub type CMP2AR = crate::Reg<u32, _CMP2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP2AR;
#[doc = "`read()` method returns [cmp2ar::R](cmp2ar::R) reader structure"]
impl crate::Readable for CMP2AR {}
#[doc = "`write(|w| ..)` method takes [cmp2ar::W](cmp2ar::W) writer structure"]
impl crate::Writable for CMP2AR {}
#[doc = "Timerx Compare 2 Register"]
pub mod cmp2ar;
#[doc = "Timerx Compare 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp3ar](cmp3ar) module"]
pub type CMP3AR = crate::Reg<u32, _CMP3AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP3AR;
#[doc = "`read()` method returns [cmp3ar::R](cmp3ar::R) reader structure"]
impl crate::Readable for CMP3AR {}
#[doc = "`write(|w| ..)` method takes [cmp3ar::W](cmp3ar::W) writer structure"]
impl crate::Writable for CMP3AR {}
#[doc = "Timerx Compare 3 Register"]
pub mod cmp3ar;
#[doc = "Timerx Compare 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp4ar](cmp4ar) module"]
pub type CMP4AR = crate::Reg<u32, _CMP4AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CMP4AR;
#[doc = "`read()` method returns [cmp4ar::R](cmp4ar::R) reader structure"]
impl crate::Readable for CMP4AR {}
#[doc = "`write(|w| ..)` method takes [cmp4ar::W](cmp4ar::W) writer structure"]
impl crate::Writable for CMP4AR {}
#[doc = "Timerx Compare 4 Register"]
pub mod cmp4ar;
#[doc = "Timerx Capture 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1ar](cpt1ar) module"]
pub type CPT1AR = crate::Reg<u32, _CPT1AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1AR;
#[doc = "`read()` method returns [cpt1ar::R](cpt1ar::R) reader structure"]
impl crate::Readable for CPT1AR {}
#[doc = "Timerx Capture 1 Register"]
pub mod cpt1ar;
#[doc = "Timerx Capture 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2ar](cpt2ar) module"]
pub type CPT2AR = crate::Reg<u32, _CPT2AR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2AR;
#[doc = "`read()` method returns [cpt2ar::R](cpt2ar::R) reader structure"]
impl crate::Readable for CPT2AR {}
#[doc = "Timerx Capture 2 Register"]
pub mod cpt2ar;
#[doc = "Timerx Deadtime Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtar](dtar) module"]
pub type DTAR = crate::Reg<u32, _DTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTAR;
#[doc = "`read()` method returns [dtar::R](dtar::R) reader structure"]
impl crate::Readable for DTAR {}
#[doc = "`write(|w| ..)` method takes [dtar::W](dtar::W) writer structure"]
impl crate::Writable for DTAR {}
#[doc = "Timerx Deadtime Register"]
pub mod dtar;
#[doc = "Timerx Output1 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seta1r](seta1r) module"]
pub type SETA1R = crate::Reg<u32, _SETA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETA1R;
#[doc = "`read()` method returns [seta1r::R](seta1r::R) reader structure"]
impl crate::Readable for SETA1R {}
#[doc = "`write(|w| ..)` method takes [seta1r::W](seta1r::W) writer structure"]
impl crate::Writable for SETA1R {}
#[doc = "Timerx Output1 Set Register"]
pub mod seta1r;
#[doc = "Timerx Output1 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsta1r](rsta1r) module"]
pub type RSTA1R = crate::Reg<u32, _RSTA1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTA1R;
#[doc = "`read()` method returns [rsta1r::R](rsta1r::R) reader structure"]
impl crate::Readable for RSTA1R {}
#[doc = "`write(|w| ..)` method takes [rsta1r::W](rsta1r::W) writer structure"]
impl crate::Writable for RSTA1R {}
#[doc = "Timerx Output1 Reset Register"]
pub mod rsta1r;
#[doc = "Timerx Output2 Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seta2r](seta2r) module"]
pub type SETA2R = crate::Reg<u32, _SETA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SETA2R;
#[doc = "`read()` method returns [seta2r::R](seta2r::R) reader structure"]
impl crate::Readable for SETA2R {}
#[doc = "`write(|w| ..)` method takes [seta2r::W](seta2r::W) writer structure"]
impl crate::Writable for SETA2R {}
#[doc = "Timerx Output2 Set Register"]
pub mod seta2r;
#[doc = "Timerx Output2 Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsta2r](rsta2r) module"]
pub type RSTA2R = crate::Reg<u32, _RSTA2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTA2R;
#[doc = "`read()` method returns [rsta2r::R](rsta2r::R) reader structure"]
impl crate::Readable for RSTA2R {}
#[doc = "`write(|w| ..)` method takes [rsta2r::W](rsta2r::W) writer structure"]
impl crate::Writable for RSTA2R {}
#[doc = "Timerx Output2 Reset Register"]
pub mod rsta2r;
#[doc = "Timerx External Event Filtering Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefar1](eefar1) module"]
pub type EEFAR1 = crate::Reg<u32, _EEFAR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFAR1;
#[doc = "`read()` method returns [eefar1::R](eefar1::R) reader structure"]
impl crate::Readable for EEFAR1 {}
#[doc = "`write(|w| ..)` method takes [eefar1::W](eefar1::W) writer structure"]
impl crate::Writable for EEFAR1 {}
#[doc = "Timerx External Event Filtering Register 1"]
pub mod eefar1;
#[doc = "Timerx External Event Filtering Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eefar2](eefar2) module"]
pub type EEFAR2 = crate::Reg<u32, _EEFAR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EEFAR2;
#[doc = "`read()` method returns [eefar2::R](eefar2::R) reader structure"]
impl crate::Readable for EEFAR2 {}
#[doc = "`write(|w| ..)` method takes [eefar2::W](eefar2::W) writer structure"]
impl crate::Writable for EEFAR2 {}
#[doc = "Timerx External Event Filtering Register 2"]
pub mod eefar2;
#[doc = "TimerA Reset Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstar](rstar) module"]
pub type RSTAR = crate::Reg<u32, _RSTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RSTAR;
#[doc = "`read()` method returns [rstar::R](rstar::R) reader structure"]
impl crate::Readable for RSTAR {}
#[doc = "`write(|w| ..)` method takes [rstar::W](rstar::W) writer structure"]
impl crate::Writable for RSTAR {}
#[doc = "TimerA Reset Register"]
pub mod rstar;
#[doc = "Timerx Chopper Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpar](chpar) module"]
pub type CHPAR = crate::Reg<u32, _CHPAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CHPAR;
#[doc = "`read()` method returns [chpar::R](chpar::R) reader structure"]
impl crate::Readable for CHPAR {}
#[doc = "`write(|w| ..)` method takes [chpar::W](chpar::W) writer structure"]
impl crate::Writable for CHPAR {}
#[doc = "Timerx Chopper Register"]
pub mod chpar;
#[doc = "Timerx Capture 2 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt1acr](cpt1acr) module"]
pub type CPT1ACR = crate::Reg<u32, _CPT1ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT1ACR;
#[doc = "`read()` method returns [cpt1acr::R](cpt1acr::R) reader structure"]
impl crate::Readable for CPT1ACR {}
#[doc = "`write(|w| ..)` method takes [cpt1acr::W](cpt1acr::W) writer structure"]
impl crate::Writable for CPT1ACR {}
#[doc = "Timerx Capture 2 Control Register"]
pub mod cpt1acr;
#[doc = "CPT2xCR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpt2acr](cpt2acr) module"]
pub type CPT2ACR = crate::Reg<u32, _CPT2ACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CPT2ACR;
#[doc = "`read()` method returns [cpt2acr::R](cpt2acr::R) reader structure"]
impl crate::Readable for CPT2ACR {}
#[doc = "`write(|w| ..)` method takes [cpt2acr::W](cpt2acr::W) writer structure"]
impl crate::Writable for CPT2ACR {}
#[doc = "CPT2xCR"]
pub mod cpt2acr;
#[doc = "Timerx Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [outar](outar) module"]
pub type OUTAR = crate::Reg<u32, _OUTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OUTAR;
#[doc = "`read()` method returns [outar::R](outar::R) reader structure"]
impl crate::Readable for OUTAR {}
#[doc = "`write(|w| ..)` method takes [outar::W](outar::W) writer structure"]
impl crate::Writable for OUTAR {}
#[doc = "Timerx Output Register"]
pub mod outar;
#[doc = "Timerx Fault Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltar](fltar) module"]
pub type FLTAR = crate::Reg<u32, _FLTAR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTAR;
#[doc = "`read()` method returns [fltar::R](fltar::R) reader structure"]
impl crate::Readable for FLTAR {}
#[doc = "`write(|w| ..)` method takes [fltar::W](fltar::W) writer structure"]
impl crate::Writable for FLTAR {}
#[doc = "Timerx Fault Register"]
pub mod fltar;
#[doc = "HRTIM Timerx Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timacr2](timacr2) module"]
pub type TIMACR2 = crate::Reg<u32, _TIMACR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _TIMACR2;
#[doc = "`read()` method returns [timacr2::R](timacr2::R) reader structure"]
impl crate::Readable for TIMACR2 {}
#[doc = "`write(|w| ..)` method takes [timacr2::W](timacr2::W) writer structure"]
impl crate::Writable for TIMACR2 {}
#[doc = "HRTIM Timerx Control Register 2"]
pub mod timacr2;
#[doc = "HRTIM Timerx External Event Filtering Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aeefr3](aeefr3) module"]
pub type AEEFR3 = crate::Reg<u32, _AEEFR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AEEFR3;
#[doc = "`read()` method returns [aeefr3::R](aeefr3::R) reader structure"]
impl crate::Readable for AEEFR3 {}
#[doc = "`write(|w| ..)` method takes [aeefr3::W](aeefr3::W) writer structure"]
impl crate::Writable for AEEFR3 {}
#[doc = "HRTIM Timerx External Event Filtering Register 3"]
pub mod aeefr3;
