#[doc = "Register `MTLRxQMPOCR` reader"]
pub struct R(crate::R<MTLRXQMPOCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLRXQMPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLRXQMPOCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLRXQMPOCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OVFPKTCNT` reader - Overflow Packet Counter"]
pub struct OVFPKTCNT_R(crate::FieldReader<u16, u16>);
impl OVFPKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        OVFPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFPKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFCNTOVF` reader - Overflow Counter Overflow Bit"]
pub struct OVFCNTOVF_R(crate::FieldReader<bool, bool>);
impl OVFCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISPKTCNT` reader - Missed Packet Counter"]
pub struct MISPKTCNT_R(crate::FieldReader<u16, u16>);
impl MISPKTCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MISPKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISPKTCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MISCNTOVF` reader - Missed Packet Counter Overflow Bit"]
pub struct MISCNTOVF_R(crate::FieldReader<bool, bool>);
impl MISCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MISCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qmpocr](index.html) module"]
pub struct MTLRXQMPOCR_SPEC;
impl crate::RegisterSpec for MTLRXQMPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qmpocr::R](R) reader structure"]
impl crate::Readable for MTLRXQMPOCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MTLRxQMPOCR to value 0"]
impl crate::Resettable for MTLRXQMPOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
