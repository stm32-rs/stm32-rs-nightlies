#[doc = "Register `MTLTxQUR` reader"]
pub struct R(crate::R<MTLTXQUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLTXQUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLTXQUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLTXQUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLTxQUR` writer"]
pub struct W(crate::W<MTLTXQUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLTXQUR_SPEC>;
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
impl From<crate::W<MTLTXQUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLTXQUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter"]
pub struct UFCNTOVF_R(crate::FieldReader<bool, bool>);
impl UFCNTOVF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UFCNTOVF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFCNTOVF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter"]
pub struct UFCNTOVF_W<'a> {
    w: &'a mut W,
}
impl<'a> UFCNTOVF_W<'a> {
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
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
pub struct UFFRMCNT_R(crate::FieldReader<u16, u16>);
impl UFFRMCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        UFFRMCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UFFRMCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UFFRMCNT` writer - Underflow Packet Counter"]
pub struct UFFRMCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UFFRMCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W {
        UFCNTOVF_W { w: self }
    }
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W {
        UFFRMCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tx queue underflow register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtltx_qur](index.html) module"]
pub struct MTLTXQUR_SPEC;
impl crate::RegisterSpec for MTLTXQUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtltx_qur::R](R) reader structure"]
impl crate::Readable for MTLTXQUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtltx_qur::W](W) writer structure"]
impl crate::Writable for MTLTXQUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MTLTXQUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
