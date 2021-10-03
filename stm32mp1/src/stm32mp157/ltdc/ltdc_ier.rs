#[doc = "Register `LTDC_IER` reader"]
pub struct R(crate::R<LTDC_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_IER` writer"]
pub struct W(crate::W<LTDC_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_IER_SPEC>;
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
impl From<crate::W<LTDC_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LIE` reader - LIE"]
pub struct LIE_R(crate::FieldReader<bool, bool>);
impl LIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LIE` writer - LIE"]
pub struct LIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LIE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `FUIE` reader - FUIE"]
pub struct FUIE_R(crate::FieldReader<bool, bool>);
impl FUIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FUIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FUIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FUIE` writer - FUIE"]
pub struct FUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FUIE_W<'a> {
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
#[doc = "Field `TERRIE` reader - TERRIE"]
pub struct TERRIE_R(crate::FieldReader<bool, bool>);
impl TERRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TERRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TERRIE` writer - TERRIE"]
pub struct TERRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TERRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `RRIE` reader - RRIE"]
pub struct RRIE_R(crate::FieldReader<bool, bool>);
impl RRIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RRIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RRIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RRIE` writer - RRIE"]
pub struct RRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RRIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    pub fn lie(&self) -> LIE_R {
        LIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    pub fn fuie(&self) -> FUIE_R {
        FUIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LIE"]
    #[inline(always)]
    pub fn lie(&mut self) -> LIE_W {
        LIE_W { w: self }
    }
    #[doc = "Bit 1 - FUIE"]
    #[inline(always)]
    pub fn fuie(&mut self) -> FUIE_W {
        FUIE_W { w: self }
    }
    #[doc = "Bit 2 - TERRIE"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W {
        TERRIE_W { w: self }
    }
    #[doc = "Bit 3 - RRIE"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W {
        RRIE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_ier](index.html) module"]
pub struct LTDC_IER_SPEC;
impl crate::RegisterSpec for LTDC_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_ier::R](R) reader structure"]
impl crate::Readable for LTDC_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_ier::W](W) writer structure"]
impl crate::Writable for LTDC_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_IER to value 0"]
impl crate::Resettable for LTDC_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
