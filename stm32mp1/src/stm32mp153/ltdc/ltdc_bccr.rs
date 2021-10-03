#[doc = "Register `LTDC_BCCR` reader"]
pub struct R(crate::R<LTDC_BCCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_BCCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_BCCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_BCCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_BCCR` writer"]
pub struct W(crate::W<LTDC_BCCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_BCCR_SPEC>;
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
impl From<crate::W<LTDC_BCCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_BCCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BCBLUE` reader - BCBLUE"]
pub struct BCBLUE_R(crate::FieldReader<u8, u8>);
impl BCBLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCBLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCBLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCBLUE` writer - BCBLUE"]
pub struct BCBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> BCBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `BCGREEN` reader - BCGREEN"]
pub struct BCGREEN_R(crate::FieldReader<u8, u8>);
impl BCGREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCGREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCGREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCGREEN` writer - BCGREEN"]
pub struct BCGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `BCRED` reader - BCRED"]
pub struct BCRED_R(crate::FieldReader<u8, u8>);
impl BCRED_R {
    pub(crate) fn new(bits: u8) -> Self {
        BCRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCRED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCRED` writer - BCRED"]
pub struct BCRED_W<'a> {
    w: &'a mut W,
}
impl<'a> BCRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    pub fn bcblue(&self) -> BCBLUE_R {
        BCBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BCGREEN_R {
        BCGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    pub fn bcred(&self) -> BCRED_R {
        BCRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - BCBLUE"]
    #[inline(always)]
    pub fn bcblue(&mut self) -> BCBLUE_W {
        BCBLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - BCGREEN"]
    #[inline(always)]
    pub fn bcgreen(&mut self) -> BCGREEN_W {
        BCGREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - BCRED"]
    #[inline(always)]
    pub fn bcred(&mut self) -> BCRED_W {
        BCRED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the background color (RGB888).\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_bccr](index.html) module"]
pub struct LTDC_BCCR_SPEC;
impl crate::RegisterSpec for LTDC_BCCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_bccr::R](R) reader structure"]
impl crate::Readable for LTDC_BCCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_bccr::W](W) writer structure"]
impl crate::Writable for LTDC_BCCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_BCCR to value 0"]
impl crate::Resettable for LTDC_BCCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
