#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    pub cr1: CR1,
    #[doc = "0x04 - Control Register 2"]
    pub cr2: CR2,
    #[doc = "0x08 - Interrupt Status Register"]
    pub isr: ISR,
    #[doc = "0x0c - Interrupt Clear Register"]
    pub icr: ICR,
    #[doc = "0x10 - Interrupt Enable Register"]
    pub ier: IER,
    #[doc = "0x14 - Output Enable Register"]
    pub oenr: OENR,
    #[doc = "0x18 - DISR"]
    pub odisr: ODISR,
    #[doc = "0x1c - Output Disable Status Register"]
    pub odsr: ODSR,
    #[doc = "0x20 - Burst Mode Control Register"]
    pub bmcr: BMCR,
    #[doc = "0x24 - BMTRGR"]
    pub bmtrgr: BMTRGR,
    #[doc = "0x28 - BMCMPR"]
    pub bmcmpr: BMCMPR,
    #[doc = "0x2c - Burst Mode Period Register"]
    pub bmper: BMPER,
    #[doc = "0x30 - Timer External Event Control Register 1"]
    pub eecr1: EECR1,
    #[doc = "0x34 - Timer External Event Control Register 2"]
    pub eecr2: EECR2,
    #[doc = "0x38 - Timer External Event Control Register 3"]
    pub eecr3: EECR3,
    #[doc = "0x3c - ADC Trigger 1 Register"]
    pub adc1r: ADC1R,
    #[doc = "0x40 - ADC Trigger 2 Register"]
    pub adc2r: ADC2R,
    #[doc = "0x44 - ADC Trigger 3 Register"]
    pub adc3r: ADC3R,
    #[doc = "0x48 - ADC Trigger 4 Register"]
    pub adc4r: ADC4R,
    #[doc = "0x4c - DLL Control Register"]
    pub dllcr: DLLCR,
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    pub fltinr1: FLTINR1,
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    pub fltinr2: FLTINR2,
    #[doc = "0x58 - BDMUPDR"]
    pub bdmupr: BDMUPR,
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    pub bdtaupr: BDTAUPR,
    #[doc = "0x60 - Burst DMA Timerx update Register"]
    pub bdtbupr: BDTBUPR,
    #[doc = "0x64 - Burst DMA Timerx update Register"]
    pub bdtcupr: BDTCUPR,
    #[doc = "0x68 - Burst DMA Timerx update Register"]
    pub bdtdupr: BDTDUPR,
    #[doc = "0x6c - Burst DMA Timerx update Register"]
    pub bdteupr: BDTEUPR,
    #[doc = "0x70 - Burst DMA Data Register"]
    pub bdmadr: BDMADR,
}
#[doc = "Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](cr1) module"]
pub type CR1 = crate::Reg<u32, _CR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR1;
#[doc = "`read()` method returns [cr1::R](cr1::R) reader structure"]
impl crate::Readable for CR1 {}
#[doc = "`write(|w| ..)` method takes [cr1::W](cr1::W) writer structure"]
impl crate::Writable for CR1 {}
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](cr2) module"]
pub type CR2 = crate::Reg<u32, _CR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CR2;
#[doc = "`read()` method returns [cr2::R](cr2::R) reader structure"]
impl crate::Readable for CR2 {}
#[doc = "`write(|w| ..)` method takes [cr2::W](cr2::W) writer structure"]
impl crate::Writable for CR2 {}
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](isr) module"]
pub type ISR = crate::Reg<u32, _ISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISR;
#[doc = "`read()` method returns [isr::R](isr::R) reader structure"]
impl crate::Readable for ISR {}
#[doc = "`write(|w| ..)` method takes [isr::W](isr::W) writer structure"]
impl crate::Writable for ISR {}
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "Interrupt Clear Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icr](icr) module"]
pub type ICR = crate::Reg<u32, _ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ICR;
#[doc = "`write(|w| ..)` method takes [icr::W](icr::W) writer structure"]
impl crate::Writable for ICR {}
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ier](ier) module"]
pub type IER = crate::Reg<u32, _IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IER;
#[doc = "`read()` method returns [ier::R](ier::R) reader structure"]
impl crate::Readable for IER {}
#[doc = "`write(|w| ..)` method takes [ier::W](ier::W) writer structure"]
impl crate::Writable for IER {}
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "Output Enable Register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oenr](oenr) module"]
pub type OENR = crate::Reg<u32, _OENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _OENR;
#[doc = "`write(|w| ..)` method takes [oenr::W](oenr::W) writer structure"]
impl crate::Writable for OENR {}
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "DISR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odisr](odisr) module"]
pub type ODISR = crate::Reg<u32, _ODISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODISR;
#[doc = "`read()` method returns [odisr::R](odisr::R) reader structure"]
impl crate::Readable for ODISR {}
#[doc = "`write(|w| ..)` method takes [odisr::W](odisr::W) writer structure"]
impl crate::Writable for ODISR {}
#[doc = "DISR"]
pub mod odisr;
#[doc = "Output Disable Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [odsr](odsr) module"]
pub type ODSR = crate::Reg<u32, _ODSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ODSR;
#[doc = "`read()` method returns [odsr::R](odsr::R) reader structure"]
impl crate::Readable for ODSR {}
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "Burst Mode Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcr](bmcr) module"]
pub type BMCR = crate::Reg<u32, _BMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCR;
#[doc = "`read()` method returns [bmcr::R](bmcr::R) reader structure"]
impl crate::Readable for BMCR {}
#[doc = "`write(|w| ..)` method takes [bmcr::W](bmcr::W) writer structure"]
impl crate::Writable for BMCR {}
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRGR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmtrgr](bmtrgr) module"]
pub type BMTRGR = crate::Reg<u32, _BMTRGR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMTRGR;
#[doc = "`read()` method returns [bmtrgr::R](bmtrgr::R) reader structure"]
impl crate::Readable for BMTRGR {}
#[doc = "`write(|w| ..)` method takes [bmtrgr::W](bmtrgr::W) writer structure"]
impl crate::Writable for BMTRGR {}
#[doc = "BMTRGR"]
pub mod bmtrgr;
#[doc = "BMCMPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmcmpr](bmcmpr) module"]
pub type BMCMPR = crate::Reg<u32, _BMCMPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMCMPR;
#[doc = "`read()` method returns [bmcmpr::R](bmcmpr::R) reader structure"]
impl crate::Readable for BMCMPR {}
#[doc = "`write(|w| ..)` method takes [bmcmpr::W](bmcmpr::W) writer structure"]
impl crate::Writable for BMCMPR {}
#[doc = "BMCMPR"]
pub mod bmcmpr;
#[doc = "Burst Mode Period Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bmper](bmper) module"]
pub type BMPER = crate::Reg<u32, _BMPER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BMPER;
#[doc = "`read()` method returns [bmper::R](bmper::R) reader structure"]
impl crate::Readable for BMPER {}
#[doc = "`write(|w| ..)` method takes [bmper::W](bmper::W) writer structure"]
impl crate::Writable for BMPER {}
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "Timer External Event Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr1](eecr1) module"]
pub type EECR1 = crate::Reg<u32, _EECR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR1;
#[doc = "`read()` method returns [eecr1::R](eecr1::R) reader structure"]
impl crate::Readable for EECR1 {}
#[doc = "`write(|w| ..)` method takes [eecr1::W](eecr1::W) writer structure"]
impl crate::Writable for EECR1 {}
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "Timer External Event Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr2](eecr2) module"]
pub type EECR2 = crate::Reg<u32, _EECR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR2;
#[doc = "`read()` method returns [eecr2::R](eecr2::R) reader structure"]
impl crate::Readable for EECR2 {}
#[doc = "`write(|w| ..)` method takes [eecr2::W](eecr2::W) writer structure"]
impl crate::Writable for EECR2 {}
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "Timer External Event Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eecr3](eecr3) module"]
pub type EECR3 = crate::Reg<u32, _EECR3>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _EECR3;
#[doc = "`read()` method returns [eecr3::R](eecr3::R) reader structure"]
impl crate::Readable for EECR3 {}
#[doc = "`write(|w| ..)` method takes [eecr3::W](eecr3::W) writer structure"]
impl crate::Writable for EECR3 {}
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC Trigger 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc1r](adc1r) module"]
pub type ADC1R = crate::Reg<u32, _ADC1R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC1R;
#[doc = "`read()` method returns [adc1r::R](adc1r::R) reader structure"]
impl crate::Readable for ADC1R {}
#[doc = "`write(|w| ..)` method takes [adc1r::W](adc1r::W) writer structure"]
impl crate::Writable for ADC1R {}
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC Trigger 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc2r](adc2r) module"]
pub type ADC2R = crate::Reg<u32, _ADC2R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC2R;
#[doc = "`read()` method returns [adc2r::R](adc2r::R) reader structure"]
impl crate::Readable for ADC2R {}
#[doc = "`write(|w| ..)` method takes [adc2r::W](adc2r::W) writer structure"]
impl crate::Writable for ADC2R {}
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC Trigger 3 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc3r](adc3r) module"]
pub type ADC3R = crate::Reg<u32, _ADC3R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC3R;
#[doc = "`read()` method returns [adc3r::R](adc3r::R) reader structure"]
impl crate::Readable for ADC3R {}
#[doc = "`write(|w| ..)` method takes [adc3r::W](adc3r::W) writer structure"]
impl crate::Writable for ADC3R {}
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC Trigger 4 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc4r](adc4r) module"]
pub type ADC4R = crate::Reg<u32, _ADC4R>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ADC4R;
#[doc = "`read()` method returns [adc4r::R](adc4r::R) reader structure"]
impl crate::Readable for ADC4R {}
#[doc = "`write(|w| ..)` method takes [adc4r::W](adc4r::W) writer structure"]
impl crate::Writable for ADC4R {}
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dllcr](dllcr) module"]
pub type DLLCR = crate::Reg<u32, _DLLCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DLLCR;
#[doc = "`read()` method returns [dllcr::R](dllcr::R) reader structure"]
impl crate::Readable for DLLCR {}
#[doc = "`write(|w| ..)` method takes [dllcr::W](dllcr::W) writer structure"]
impl crate::Writable for DLLCR {}
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "HRTIM Fault Input Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr1](fltinr1) module"]
pub type FLTINR1 = crate::Reg<u32, _FLTINR1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR1;
#[doc = "`read()` method returns [fltinr1::R](fltinr1::R) reader structure"]
impl crate::Readable for FLTINR1 {}
#[doc = "`write(|w| ..)` method takes [fltinr1::W](fltinr1::W) writer structure"]
impl crate::Writable for FLTINR1 {}
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "HRTIM Fault Input Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fltinr2](fltinr2) module"]
pub type FLTINR2 = crate::Reg<u32, _FLTINR2>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FLTINR2;
#[doc = "`read()` method returns [fltinr2::R](fltinr2::R) reader structure"]
impl crate::Readable for FLTINR2 {}
#[doc = "`write(|w| ..)` method takes [fltinr2::W](fltinr2::W) writer structure"]
impl crate::Writable for FLTINR2 {}
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmupr](bdmupr) module"]
pub type BDMUPR = crate::Reg<u32, _BDMUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMUPR;
#[doc = "`read()` method returns [bdmupr::R](bdmupr::R) reader structure"]
impl crate::Readable for BDMUPR {}
#[doc = "`write(|w| ..)` method takes [bdmupr::W](bdmupr::W) writer structure"]
impl crate::Writable for BDMUPR {}
#[doc = "BDMUPDR"]
pub mod bdmupr;
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtaupr](bdtaupr) module"]
pub type BDTAUPR = crate::Reg<u32, _BDTAUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTAUPR;
#[doc = "`read()` method returns [bdtaupr::R](bdtaupr::R) reader structure"]
impl crate::Readable for BDTAUPR {}
#[doc = "`write(|w| ..)` method takes [bdtaupr::W](bdtaupr::W) writer structure"]
impl crate::Writable for BDTAUPR {}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtaupr;
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtbupr](bdtbupr) module"]
pub type BDTBUPR = crate::Reg<u32, _BDTBUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTBUPR;
#[doc = "`read()` method returns [bdtbupr::R](bdtbupr::R) reader structure"]
impl crate::Readable for BDTBUPR {}
#[doc = "`write(|w| ..)` method takes [bdtbupr::W](bdtbupr::W) writer structure"]
impl crate::Writable for BDTBUPR {}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtbupr;
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtcupr](bdtcupr) module"]
pub type BDTCUPR = crate::Reg<u32, _BDTCUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTCUPR;
#[doc = "`read()` method returns [bdtcupr::R](bdtcupr::R) reader structure"]
impl crate::Readable for BDTCUPR {}
#[doc = "`write(|w| ..)` method takes [bdtcupr::W](bdtcupr::W) writer structure"]
impl crate::Writable for BDTCUPR {}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtcupr;
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdtdupr](bdtdupr) module"]
pub type BDTDUPR = crate::Reg<u32, _BDTDUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTDUPR;
#[doc = "`read()` method returns [bdtdupr::R](bdtdupr::R) reader structure"]
impl crate::Readable for BDTDUPR {}
#[doc = "`write(|w| ..)` method takes [bdtdupr::W](bdtdupr::W) writer structure"]
impl crate::Writable for BDTDUPR {}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtdupr;
#[doc = "Burst DMA Timerx update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdteupr](bdteupr) module"]
pub type BDTEUPR = crate::Reg<u32, _BDTEUPR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDTEUPR;
#[doc = "`read()` method returns [bdteupr::R](bdteupr::R) reader structure"]
impl crate::Readable for BDTEUPR {}
#[doc = "`write(|w| ..)` method takes [bdteupr::W](bdteupr::W) writer structure"]
impl crate::Writable for BDTEUPR {}
#[doc = "Burst DMA Timerx update Register"]
pub mod bdteupr;
#[doc = "Burst DMA Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmadr](bdmadr) module"]
pub type BDMADR = crate::Reg<u32, _BDMADR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _BDMADR;
#[doc = "`read()` method returns [bdmadr::R](bdmadr::R) reader structure"]
impl crate::Readable for BDMADR {}
#[doc = "`write(|w| ..)` method takes [bdmadr::W](bdmadr::W) writer structure"]
impl crate::Writable for BDMADR {}
#[doc = "Burst DMA Data Register"]
pub mod bdmadr;
