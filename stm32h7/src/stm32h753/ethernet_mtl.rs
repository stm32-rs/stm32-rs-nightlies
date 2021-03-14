#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Operating mode Register"]
    pub mtlomr: MTLOMR,
    _reserved1: [u8; 28usize],
    #[doc = "0x20 - Interrupt status Register"]
    pub mtlisr: MTLISR,
    _reserved2: [u8; 220usize],
    #[doc = "0x100 - Tx queue operating mode Register"]
    pub mtltx_qomr: MTLTXQOMR,
    #[doc = "0x104 - Tx queue underflow register"]
    pub mtltx_qur: MTLTXQUR,
    #[doc = "0x108 - Tx queue debug Register"]
    pub mtltx_qdr: MTLTXQDR,
    _reserved5: [u8; 32usize],
    #[doc = "0x12c - Queue interrupt control status Register"]
    pub mtlqicsr: MTLQICSR,
    #[doc = "0x130 - Rx queue operating mode register"]
    pub mtlrx_qomr: MTLRXQOMR,
    #[doc = "0x134 - Rx queue missed packet and overflow counter register"]
    pub mtlrx_qmpocr: MTLRXQMPOCR,
    #[doc = "0x138 - Rx queue debug register"]
    pub mtlrx_qdr: MTLRXQDR,
}
#[doc = "Operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlomr](mtlomr) module"]
pub type MTLOMR = crate::Reg<u32, _MTLOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLOMR;
#[doc = "`read()` method returns [mtlomr::R](mtlomr::R) reader structure"]
impl crate::Readable for MTLOMR {}
#[doc = "`write(|w| ..)` method takes [mtlomr::W](mtlomr::W) writer structure"]
impl crate::Writable for MTLOMR {}
#[doc = "Operating mode Register"]
pub mod mtlomr;
#[doc = "Interrupt status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlisr](mtlisr) module"]
pub type MTLISR = crate::Reg<u32, _MTLISR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLISR;
#[doc = "`read()` method returns [mtlisr::R](mtlisr::R) reader structure"]
impl crate::Readable for MTLISR {}
#[doc = "Interrupt status Register"]
pub mod mtlisr;
#[doc = "Tx queue operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qomr](mtltx_qomr) module"]
pub type MTLTXQOMR = crate::Reg<u32, _MTLTXQOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLTXQOMR;
#[doc = "`read()` method returns [mtltx_qomr::R](mtltx_qomr::R) reader structure"]
impl crate::Readable for MTLTXQOMR {}
#[doc = "`write(|w| ..)` method takes [mtltx_qomr::W](mtltx_qomr::W) writer structure"]
impl crate::Writable for MTLTXQOMR {}
#[doc = "Tx queue operating mode Register"]
pub mod mtltx_qomr;
#[doc = "Tx queue underflow register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qur](mtltx_qur) module"]
pub type MTLTXQUR = crate::Reg<u32, _MTLTXQUR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLTXQUR;
#[doc = "`read()` method returns [mtltx_qur::R](mtltx_qur::R) reader structure"]
impl crate::Readable for MTLTXQUR {}
#[doc = "Tx queue underflow register"]
pub mod mtltx_qur;
#[doc = "Tx queue debug Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qdr](mtltx_qdr) module"]
pub type MTLTXQDR = crate::Reg<u32, _MTLTXQDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLTXQDR;
#[doc = "`read()` method returns [mtltx_qdr::R](mtltx_qdr::R) reader structure"]
impl crate::Readable for MTLTXQDR {}
#[doc = "Tx queue debug Register"]
pub mod mtltx_qdr;
#[doc = "Queue interrupt control status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlqicsr](mtlqicsr) module"]
pub type MTLQICSR = crate::Reg<u32, _MTLQICSR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLQICSR;
#[doc = "`read()` method returns [mtlqicsr::R](mtlqicsr::R) reader structure"]
impl crate::Readable for MTLQICSR {}
#[doc = "`write(|w| ..)` method takes [mtlqicsr::W](mtlqicsr::W) writer structure"]
impl crate::Writable for MTLQICSR {}
#[doc = "Queue interrupt control status Register"]
pub mod mtlqicsr;
#[doc = "Rx queue operating mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qomr](mtlrx_qomr) module"]
pub type MTLRXQOMR = crate::Reg<u32, _MTLRXQOMR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLRXQOMR;
#[doc = "`read()` method returns [mtlrx_qomr::R](mtlrx_qomr::R) reader structure"]
impl crate::Readable for MTLRXQOMR {}
#[doc = "`write(|w| ..)` method takes [mtlrx_qomr::W](mtlrx_qomr::W) writer structure"]
impl crate::Writable for MTLRXQOMR {}
#[doc = "Rx queue operating mode register"]
pub mod mtlrx_qomr;
#[doc = "Rx queue missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qmpocr](mtlrx_qmpocr) module"]
pub type MTLRXQMPOCR = crate::Reg<u32, _MTLRXQMPOCR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLRXQMPOCR;
#[doc = "`read()` method returns [mtlrx_qmpocr::R](mtlrx_qmpocr::R) reader structure"]
impl crate::Readable for MTLRXQMPOCR {}
#[doc = "Rx queue missed packet and overflow counter register"]
pub mod mtlrx_qmpocr;
#[doc = "Rx queue debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qdr](mtlrx_qdr) module"]
pub type MTLRXQDR = crate::Reg<u32, _MTLRXQDR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MTLRXQDR;
#[doc = "`read()` method returns [mtlrx_qdr::R](mtlrx_qdr::R) reader structure"]
impl crate::Readable for MTLRXQDR {}
#[doc = "Rx queue debug register"]
pub mod mtlrx_qdr;
