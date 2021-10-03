#[doc = "Register `MTLOMR` reader"]
pub struct R(crate::R<MTLOMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MTLOMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MTLOMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MTLOMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MTLOMR` writer"]
pub struct W(crate::W<MTLOMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTLOMR_SPEC>;
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
impl From<crate::W<MTLOMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTLOMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTXSTS` reader - Drop Transmit Status"]
pub struct DTXSTS_R(crate::FieldReader<bool, bool>);
impl DTXSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTXSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTXSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTXSTS` writer - Drop Transmit Status"]
pub struct DTXSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTXSTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CNTPRST` reader - Counters Preset"]
pub struct CNTPRST_R(crate::FieldReader<bool, bool>);
impl CNTPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTPRST` writer - Counters Preset"]
pub struct CNTPRST_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTPRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `CNTCLR` reader - Counters Reset"]
pub struct CNTCLR_R(crate::FieldReader<bool, bool>);
impl CNTCLR_R {
    pub(crate) fn new(bits: bool) -> Self {
        CNTCLR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTCLR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTCLR` writer - Counters Reset"]
pub struct CNTCLR_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTCLR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    pub fn dtxsts(&self) -> DTXSTS_R {
        DTXSTS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&self) -> CNTPRST_R {
        CNTPRST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    pub fn cntclr(&self) -> CNTCLR_R {
        CNTCLR_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Drop Transmit Status"]
    #[inline(always)]
    pub fn dtxsts(&mut self) -> DTXSTS_W {
        DTXSTS_W { w: self }
    }
    #[doc = "Bit 8 - Counters Preset"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CNTPRST_W {
        CNTPRST_W { w: self }
    }
    #[doc = "Bit 9 - Counters Reset"]
    #[inline(always)]
    pub fn cntclr(&mut self) -> CNTCLR_W {
        CNTCLR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operating mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtlomr](index.html) module"]
pub struct MTLOMR_SPEC;
impl crate::RegisterSpec for MTLOMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mtlomr::R](R) reader structure"]
impl crate::Readable for MTLOMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mtlomr::W](W) writer structure"]
impl crate::Writable for MTLOMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MTLOMR to value 0"]
impl crate::Resettable for MTLOMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
