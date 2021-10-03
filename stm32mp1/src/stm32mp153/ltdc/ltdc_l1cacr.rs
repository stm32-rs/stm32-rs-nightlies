#[doc = "Register `LTDC_L1CACR` reader"]
pub struct R(crate::R<LTDC_L1CACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L1CACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L1CACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L1CACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L1CACR` writer"]
pub struct W(crate::W<LTDC_L1CACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CACR_SPEC>;
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
impl From<crate::W<LTDC_L1CACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CONSTA` reader - CONSTA"]
pub struct CONSTA_R(crate::FieldReader<u8, u8>);
impl CONSTA_R {
    pub(crate) fn new(bits: u8) -> Self {
        CONSTA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONSTA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONSTA` writer - CONSTA"]
pub struct CONSTA_W<'a> {
    w: &'a mut W,
}
impl<'a> CONSTA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    pub fn consta(&self) -> CONSTA_R {
        CONSTA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - CONSTA"]
    #[inline(always)]
    pub fn consta(&mut self) -> CONSTA_W {
        CONSTA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the constant alpha value (divided by 255 by hardware), that is used in the alpha blending. Refer to LTDC_LxBFCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cacr](index.html) module"]
pub struct LTDC_L1CACR_SPEC;
impl crate::RegisterSpec for LTDC_L1CACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l1cacr::R](R) reader structure"]
impl crate::Readable for LTDC_L1CACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cacr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CACR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L1CACR to value 0xff"]
impl crate::Resettable for LTDC_L1CACR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff
    }
}
