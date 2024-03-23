#[doc = "Register `MTLRxQMPOCR` reader"]
pub type R = crate::R<MTLRX_QMPOCRrs>;
#[doc = "Field `OVFPKTCNT` reader - Overflow Packet Counter"]
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit"]
pub type OVFCNTOVF_R = crate::BitReader;
#[doc = "Field `MISPKTCNT` reader - Missed Packet Counter"]
pub type MISPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit"]
pub type MISCNTOVF_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qmpocr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QMPOCRrs;
impl crate::RegisterSpec for MTLRX_QMPOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qmpocr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QMPOCRrs {}
#[doc = "`reset()` method sets MTLRxQMPOCR to value 0"]
impl crate::Resettable for MTLRX_QMPOCRrs {
    const RESET_VALUE: u32 = 0;
}
