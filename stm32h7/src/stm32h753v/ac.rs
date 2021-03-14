#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub itcmcr: ITCMCR,
    #[doc = "0x04 - Instruction and Data Tightly-Coupled Memory Control Registers"]
    pub dtcmcr: DTCMCR,
    #[doc = "0x08 - AHBP Control register"]
    pub ahbpcr: AHBPCR,
    #[doc = "0x0c - Auxiliary Cache Control register"]
    pub cacr: CACR,
    #[doc = "0x10 - AHB Slave Control register"]
    pub ahbscr: AHBSCR,
    _reserved5: [u8; 4usize],
    #[doc = "0x18 - Auxiliary Bus Fault Status register"]
    pub abfsr: ABFSR,
}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [itcmcr](itcmcr) module"]
pub type ITCMCR = crate::Reg<u32, _ITCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ITCMCR;
#[doc = "`read()` method returns [itcmcr::R](itcmcr::R) reader structure"]
impl crate::Readable for ITCMCR {}
#[doc = "`write(|w| ..)` method takes [itcmcr::W](itcmcr::W) writer structure"]
impl crate::Writable for ITCMCR {}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod itcmcr;
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtcmcr](dtcmcr) module"]
pub type DTCMCR = crate::Reg<u32, _DTCMCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DTCMCR;
#[doc = "`read()` method returns [dtcmcr::R](dtcmcr::R) reader structure"]
impl crate::Readable for DTCMCR {}
#[doc = "`write(|w| ..)` method takes [dtcmcr::W](dtcmcr::W) writer structure"]
impl crate::Writable for DTCMCR {}
#[doc = "Instruction and Data Tightly-Coupled Memory Control Registers"]
pub mod dtcmcr;
#[doc = "AHBP Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbpcr](ahbpcr) module"]
pub type AHBPCR = crate::Reg<u32, _AHBPCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBPCR;
#[doc = "`read()` method returns [ahbpcr::R](ahbpcr::R) reader structure"]
impl crate::Readable for AHBPCR {}
#[doc = "`write(|w| ..)` method takes [ahbpcr::W](ahbpcr::W) writer structure"]
impl crate::Writable for AHBPCR {}
#[doc = "AHBP Control register"]
pub mod ahbpcr;
#[doc = "Auxiliary Cache Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacr](cacr) module"]
pub type CACR = crate::Reg<u32, _CACR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CACR;
#[doc = "`read()` method returns [cacr::R](cacr::R) reader structure"]
impl crate::Readable for CACR {}
#[doc = "`write(|w| ..)` method takes [cacr::W](cacr::W) writer structure"]
impl crate::Writable for CACR {}
#[doc = "Auxiliary Cache Control register"]
pub mod cacr;
#[doc = "AHB Slave Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbscr](ahbscr) module"]
pub type AHBSCR = crate::Reg<u32, _AHBSCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _AHBSCR;
#[doc = "`read()` method returns [ahbscr::R](ahbscr::R) reader structure"]
impl crate::Readable for AHBSCR {}
#[doc = "`write(|w| ..)` method takes [ahbscr::W](ahbscr::W) writer structure"]
impl crate::Writable for AHBSCR {}
#[doc = "AHB Slave Control register"]
pub mod ahbscr;
#[doc = "Auxiliary Bus Fault Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [abfsr](abfsr) module"]
pub type ABFSR = crate::Reg<u32, _ABFSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ABFSR;
#[doc = "`read()` method returns [abfsr::R](abfsr::R) reader structure"]
impl crate::Readable for ABFSR {}
#[doc = "`write(|w| ..)` method takes [abfsr::W](abfsr::W) writer structure"]
impl crate::Writable for ABFSR {}
#[doc = "Auxiliary Bus Fault Status register"]
pub mod abfsr;
