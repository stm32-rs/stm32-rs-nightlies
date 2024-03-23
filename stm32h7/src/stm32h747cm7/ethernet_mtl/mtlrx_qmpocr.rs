#[doc = "Register `MTLRxQMPOCR` reader"]
pub type R = crate::R<MTLRX_QMPOCRrs>;
#[doc = "Register `MTLRxQMPOCR` writer"]
pub type W = crate::W<MTLRX_QMPOCRrs>;
#[doc = "Field `OVFPKTCNT` reader - Overflow Packet Counter"]
pub type OVFPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `OVFPKTCNT` writer - Overflow Packet Counter"]
pub type OVFPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit"]
pub type OVFCNTOVF_R = crate::BitReader;
#[doc = "Field `OVFCNTOVF` writer - Overflow Counter Overflow Bit"]
pub type OVFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MISPKTCNT` reader - Missed Packet Counter"]
pub type MISPKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MISPKTCNT` writer - Missed Packet Counter"]
pub type MISPKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit"]
pub type MISCNTOVF_R = crate::BitReader;
#[doc = "Field `MISCNTOVF` writer - Missed Packet Counter Overflow Bit"]
pub type MISCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
impl W {
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W<MTLRX_QMPOCRrs> {
        OVFPKTCNT_W::new(self, 0)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W<MTLRX_QMPOCRrs> {
        OVFCNTOVF_W::new(self, 11)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W<MTLRX_QMPOCRrs> {
        MISPKTCNT_W::new(self, 16)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    #[must_use]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W<MTLRX_QMPOCRrs> {
        MISCNTOVF_W::new(self, 27)
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qmpocr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrx_qmpocr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QMPOCRrs;
impl crate::RegisterSpec for MTLRX_QMPOCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qmpocr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QMPOCRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlrx_qmpocr::W`](W) writer structure"]
impl crate::Writable for MTLRX_QMPOCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLRxQMPOCR to value 0"]
impl crate::Resettable for MTLRX_QMPOCRrs {
    const RESET_VALUE: u32 = 0;
}
