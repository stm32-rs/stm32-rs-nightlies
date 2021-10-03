#[doc = "Register `LTDC_L1CFBLR` reader"]
pub struct R(crate::R<LTDC_L1CFBLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L1CFBLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L1CFBLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L1CFBLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L1CFBLR` writer"]
pub struct W(crate::W<LTDC_L1CFBLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L1CFBLR_SPEC>;
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
impl From<crate::W<LTDC_L1CFBLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L1CFBLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFBLL` reader - CFBLL"]
pub struct CFBLL_R(crate::FieldReader<u16, u16>);
impl CFBLL_R {
    pub(crate) fn new(bits: u16) -> Self {
        CFBLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFBLL_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFBLL` writer - CFBLL"]
pub struct CFBLL_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
#[doc = "Field `CFBP` reader - CFBP"]
pub struct CFBP_R(crate::FieldReader<u16, u16>);
impl CFBP_R {
    pub(crate) fn new(bits: u16) -> Self {
        CFBP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CFBP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CFBP` writer - CFBP"]
pub struct CFBP_W<'a> {
    w: &'a mut W,
}
impl<'a> CFBP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | ((value as u32 & 0x3fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - CFBLL"]
    #[inline(always)]
    pub fn cfbll(&mut self) -> CFBLL_W {
        CFBLL_W { w: self }
    }
    #[doc = "Bits 16:29 - CFBP"]
    #[inline(always)]
    pub fn cfbp(&mut self) -> CFBP_W {
        CFBP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the color frame buffer line length and pitch.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l1cfblr](index.html) module"]
pub struct LTDC_L1CFBLR_SPEC;
impl crate::RegisterSpec for LTDC_L1CFBLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l1cfblr::R](R) reader structure"]
impl crate::Readable for LTDC_L1CFBLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l1cfblr::W](W) writer structure"]
impl crate::Writable for LTDC_L1CFBLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L1CFBLR to value 0"]
impl crate::Resettable for LTDC_L1CFBLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
