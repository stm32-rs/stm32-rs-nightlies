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
#[doc = "Register `MTLRxQMPOCR` writer"]
pub struct W(crate::W<MTLRXQMPOCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLRXQMPOCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MTLRXQMPOCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLRXQMPOCR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `MISCNTOVF` writer - Missed Packet Counter Overflow Bit"]
pub struct MISCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> MISCNTOVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
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
#[doc = "Field `MISPKTCNT` writer - Missed Packet Counter"]
pub struct MISPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MISPKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07ff << 16)) | ((value as u32 & 0x07ff) << 16);
        self.w
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
#[doc = "Field `OVFCNTOVF` writer - Overflow Counter Overflow Bit"]
pub struct OVFCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFCNTOVF_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
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
#[doc = "Field `OVFPKTCNT` writer - Overflow Packet Counter"]
pub struct OVFPKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFPKTCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MISPKTCNT_R {
        MISPKTCNT_R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OVFPKTCNT_R {
        OVFPKTCNT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 27 - Missed Packet Counter Overflow Bit"]
    #[inline(always)]
    pub fn miscntovf(&mut self) -> MISCNTOVF_W {
        MISCNTOVF_W { w: self }
    }
    #[doc = "Bits 16:26 - Missed Packet Counter"]
    #[inline(always)]
    pub fn mispktcnt(&mut self) -> MISPKTCNT_W {
        MISPKTCNT_W { w: self }
    }
    #[doc = "Bit 11 - Overflow Counter Overflow Bit"]
    #[inline(always)]
    pub fn ovfcntovf(&mut self) -> OVFCNTOVF_W {
        OVFCNTOVF_W { w: self }
    }
    #[doc = "Bits 0:10 - Overflow Packet Counter"]
    #[inline(always)]
    pub fn ovfpktcnt(&mut self) -> OVFPKTCNT_W {
        OVFPKTCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlrx_qmpocr](index.html) module"]
pub struct MTLRXQMPOCR_SPEC;
impl crate::RegisterSpec for MTLRXQMPOCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlrx_qmpocr::R](R) reader structure"]
impl crate::Readable for MTLRXQMPOCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlrx_qmpocr::W](W) writer structure"]
impl crate::Writable for MTLRXQMPOCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLRxQMPOCR to value 0"]
impl crate::Resettable for MTLRXQMPOCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
