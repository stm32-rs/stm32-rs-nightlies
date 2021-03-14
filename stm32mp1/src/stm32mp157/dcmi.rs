#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DCMI control register"]
    pub dcmi_cr: DCMI_CR,
    #[doc = "0x04 - DCMI status register"]
    pub dcmi_sr: DCMI_SR,
    #[doc = "0x08 - DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
    pub dcmi_ris: DCMI_RIS,
    #[doc = "0x0c - The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
    pub dcmi_ier: DCMI_IER,
    #[doc = "0x10 - This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
    pub dcmi_mis: DCMI_MIS,
    #[doc = "0x14 - The DCMI_ICR register is write-only."]
    pub dcmi_icr: DCMI_ICR,
    #[doc = "0x18 - DCMI embedded synchronization code register"]
    pub dcmi_escr: DCMI_ESCR,
    #[doc = "0x1c - DCMI embedded synchronization unmask register"]
    pub dcmi_esur: DCMI_ESUR,
    #[doc = "0x20 - DCMI crop window start"]
    pub dcmi_cwstrt: DCMI_CWSTRT,
    #[doc = "0x24 - DCMI crop window size"]
    pub dcmi_cwsize: DCMI_CWSIZE,
    #[doc = "0x28 - DCMI data register"]
    pub dcmi_dr: DCMI_DR,
}
#[doc = "DCMI control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cr](dcmi_cr) module"]
pub type DCMI_CR = crate::Reg<u32, _DCMI_CR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_CR;
#[doc = "`read()` method returns [dcmi_cr::R](dcmi_cr::R) reader structure"]
impl crate::Readable for DCMI_CR {}
#[doc = "`write(|w| ..)` method takes [dcmi_cr::W](dcmi_cr::W) writer structure"]
impl crate::Writable for DCMI_CR {}
#[doc = "DCMI control register"]
pub mod dcmi_cr;
#[doc = "DCMI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_sr](dcmi_sr) module"]
pub type DCMI_SR = crate::Reg<u32, _DCMI_SR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_SR;
#[doc = "`read()` method returns [dcmi_sr::R](dcmi_sr::R) reader structure"]
impl crate::Readable for DCMI_SR {}
#[doc = "DCMI status register"]
pub mod dcmi_sr;
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ris](dcmi_ris) module"]
pub type DCMI_RIS = crate::Reg<u32, _DCMI_RIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_RIS;
#[doc = "`read()` method returns [dcmi_ris::R](dcmi_ris::R) reader structure"]
impl crate::Readable for DCMI_RIS {}
#[doc = "DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this register returns the status of the corresponding interrupt before masking with the DCMI_IER register value."]
pub mod dcmi_ris;
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_ier](dcmi_ier) module"]
pub type DCMI_IER = crate::Reg<u32, _DCMI_IER>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_IER;
#[doc = "`read()` method returns [dcmi_ier::R](dcmi_ier::R) reader structure"]
impl crate::Readable for DCMI_IER {}
#[doc = "`write(|w| ..)` method takes [dcmi_ier::W](dcmi_ier::W) writer structure"]
impl crate::Writable for DCMI_IER {}
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write."]
pub mod dcmi_ier;
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_mis](dcmi_mis) module"]
pub type DCMI_MIS = crate::Reg<u32, _DCMI_MIS>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_MIS;
#[doc = "`read()` method returns [dcmi_mis::R](dcmi_mis::R) reader structure"]
impl crate::Readable for DCMI_MIS {}
#[doc = "This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set."]
pub mod dcmi_mis;
#[doc = "The DCMI_ICR register is write-only.\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_icr](dcmi_icr) module"]
pub type DCMI_ICR = crate::Reg<u32, _DCMI_ICR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_ICR;
#[doc = "`write(|w| ..)` method takes [dcmi_icr::W](dcmi_icr::W) writer structure"]
impl crate::Writable for DCMI_ICR {}
#[doc = "The DCMI_ICR register is write-only."]
pub mod dcmi_icr;
#[doc = "DCMI embedded synchronization code register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_escr](dcmi_escr) module"]
pub type DCMI_ESCR = crate::Reg<u32, _DCMI_ESCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_ESCR;
#[doc = "`read()` method returns [dcmi_escr::R](dcmi_escr::R) reader structure"]
impl crate::Readable for DCMI_ESCR {}
#[doc = "`write(|w| ..)` method takes [dcmi_escr::W](dcmi_escr::W) writer structure"]
impl crate::Writable for DCMI_ESCR {}
#[doc = "DCMI embedded synchronization code register"]
pub mod dcmi_escr;
#[doc = "DCMI embedded synchronization unmask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_esur](dcmi_esur) module"]
pub type DCMI_ESUR = crate::Reg<u32, _DCMI_ESUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_ESUR;
#[doc = "`read()` method returns [dcmi_esur::R](dcmi_esur::R) reader structure"]
impl crate::Readable for DCMI_ESUR {}
#[doc = "`write(|w| ..)` method takes [dcmi_esur::W](dcmi_esur::W) writer structure"]
impl crate::Writable for DCMI_ESUR {}
#[doc = "DCMI embedded synchronization unmask register"]
pub mod dcmi_esur;
#[doc = "DCMI crop window start\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cwstrt](dcmi_cwstrt) module"]
pub type DCMI_CWSTRT = crate::Reg<u32, _DCMI_CWSTRT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_CWSTRT;
#[doc = "`read()` method returns [dcmi_cwstrt::R](dcmi_cwstrt::R) reader structure"]
impl crate::Readable for DCMI_CWSTRT {}
#[doc = "`write(|w| ..)` method takes [dcmi_cwstrt::W](dcmi_cwstrt::W) writer structure"]
impl crate::Writable for DCMI_CWSTRT {}
#[doc = "DCMI crop window start"]
pub mod dcmi_cwstrt;
#[doc = "DCMI crop window size\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_cwsize](dcmi_cwsize) module"]
pub type DCMI_CWSIZE = crate::Reg<u32, _DCMI_CWSIZE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_CWSIZE;
#[doc = "`read()` method returns [dcmi_cwsize::R](dcmi_cwsize::R) reader structure"]
impl crate::Readable for DCMI_CWSIZE {}
#[doc = "`write(|w| ..)` method takes [dcmi_cwsize::W](dcmi_cwsize::W) writer structure"]
impl crate::Writable for DCMI_CWSIZE {}
#[doc = "DCMI crop window size"]
pub mod dcmi_cwsize;
#[doc = "DCMI data register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_dr](dcmi_dr) module"]
pub type DCMI_DR = crate::Reg<u32, _DCMI_DR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DCMI_DR;
#[doc = "`read()` method returns [dcmi_dr::R](dcmi_dr::R) reader structure"]
impl crate::Readable for DCMI_DR {}
#[doc = "DCMI data register"]
pub mod dcmi_dr;
