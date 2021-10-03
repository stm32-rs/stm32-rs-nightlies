#[doc = "Register `LTDC_L2CR` reader"]
pub struct R(crate::R<LTDC_L2CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2CR` writer"]
pub struct W(crate::W<LTDC_L2CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2CR_SPEC>;
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
impl From<crate::W<LTDC_L2CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LEN` reader - LEN"]
pub struct LEN_R(crate::FieldReader<bool, bool>);
impl LEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEN` writer - LEN"]
pub struct LEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEN_W<'a> {
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
#[doc = "Field `COLKEN` reader - COLKEN"]
pub struct COLKEN_R(crate::FieldReader<bool, bool>);
impl COLKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        COLKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COLKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COLKEN` writer - COLKEN"]
pub struct COLKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> COLKEN_W<'a> {
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
#[doc = "Field `CLUTEN` reader - CLUTEN"]
pub struct CLUTEN_R(crate::FieldReader<bool, bool>);
impl CLUTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLUTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLUTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLUTEN` writer - CLUTEN"]
pub struct CLUTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLUTEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEN"]
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W {
        LEN_W { w: self }
    }
    #[doc = "Bit 1 - COLKEN"]
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W {
        COLKEN_W { w: self }
    }
    #[doc = "Bit 4 - CLUTEN"]
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W {
        CLUTEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC layer 2 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2cr](index.html) module"]
pub struct LTDC_L2CR_SPEC;
impl crate::RegisterSpec for LTDC_L2CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2cr::R](R) reader structure"]
impl crate::Readable for LTDC_L2CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2cr::W](W) writer structure"]
impl crate::Writable for LTDC_L2CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L2CR to value 0"]
impl crate::Resettable for LTDC_L2CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
