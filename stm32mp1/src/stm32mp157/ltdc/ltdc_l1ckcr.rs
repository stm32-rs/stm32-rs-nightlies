#[doc = "Register `LTDC_L1CKCR` reader"]
pub struct R(crate::R<LTDC_L1CKCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L1CKCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L1CKCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L1CKCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L1CKCR` writer"]
pub struct W(crate::W<LTDC_L1CKCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CKCR_SPEC>;
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
impl From<crate::W<LTDC_L1CKCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CKCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKBLUE` reader - CKBLUE"]
pub struct CKBLUE_R(crate::FieldReader<u8, u8>);
impl CKBLUE_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKBLUE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKBLUE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKBLUE` writer - CKBLUE"]
pub struct CKBLUE_W<'a> {
    w: &'a mut W,
}
impl<'a> CKBLUE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `CKGREEN` reader - CKGREEN"]
pub struct CKGREEN_R(crate::FieldReader<u8, u8>);
impl CKGREEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKGREEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKGREEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKGREEN` writer - CKGREEN"]
pub struct CKGREEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CKGREEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `CKRED` reader - CKRED"]
pub struct CKRED_R(crate::FieldReader<u8, u8>);
impl CKRED_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKRED_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKRED` writer - CKRED"]
pub struct CKRED_W<'a> {
    w: &'a mut W,
}
impl<'a> CKRED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CKBLUE"]
    #[inline(always)]
    pub fn ckblue(&self) -> CKBLUE_R {
        CKBLUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - CKGREEN"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CKGREEN_R {
        CKGREEN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - CKRED"]
    #[inline(always)]
    pub fn ckred(&self) -> CKRED_R {
        CKRED_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CKBLUE"]
    #[inline(always)]
    pub fn ckblue(&mut self) -> CKBLUE_W {
        CKBLUE_W { w: self }
    }
    #[doc = "Bits 8:15 - CKGREEN"]
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CKGREEN_W {
        CKGREEN_W { w: self }
    }
    #[doc = "Bits 16:23 - CKRED"]
    #[inline(always)]
    pub fn ckred(&mut self) -> CKRED_W {
        CKRED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the color key value (RGB), that is used by the color keying.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1ckcr](index.html) module"]
pub struct LTDC_L1CKCR_SPEC;
impl crate::RegisterSpec for LTDC_L1CKCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l1ckcr::R](R) reader structure"]
impl crate::Readable for LTDC_L1CKCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1ckcr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CKCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L1CKCR to value 0"]
impl crate::Resettable for LTDC_L1CKCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
