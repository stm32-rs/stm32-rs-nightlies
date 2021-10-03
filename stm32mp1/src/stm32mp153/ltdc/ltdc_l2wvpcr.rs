#[doc = "Register `LTDC_L2WVPCR` reader"]
pub struct R(crate::R<LTDC_L2WVPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LTDC_L2WVPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LTDC_L2WVPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LTDC_L2WVPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LTDC_L2WVPCR` writer"]
pub struct W(crate::W<LTDC_L2WVPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LTDC_L2WVPCR_SPEC>;
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
impl From<crate::W<LTDC_L2WVPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LTDC_L2WVPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WVSTPOS` reader - WVSTPOS"]
pub struct WVSTPOS_R(crate::FieldReader<u16, u16>);
impl WVSTPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVSTPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVSTPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WVSTPOS` writer - WVSTPOS"]
pub struct WVSTPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSTPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
#[doc = "Field `WVSPPOS` reader - WVSPPOS"]
pub struct WVSPPOS_R(crate::FieldReader<u16, u16>);
impl WVSPPOS_R {
    pub(crate) fn new(bits: u16) -> Self {
        WVSPPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WVSPPOS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WVSPPOS` writer - WVSPPOS"]
pub struct WVSPPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> WVSPPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&self) -> WVSTPOS_R {
        WVSTPOS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&self) -> WVSPPOS_R {
        WVSPPOS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - WVSTPOS"]
    #[inline(always)]
    pub fn wvstpos(&mut self) -> WVSTPOS_W {
        WVSTPOS_W { w: self }
    }
    #[doc = "Bits 16:27 - WVSPPOS"]
    #[inline(always)]
    pub fn wvsppos(&mut self) -> WVSPPOS_W {
        WVSPPOS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register defines the vertical position (first and last line) of the layer1 or 2 window. The first visible line of a frame is the programmed value of AVBP\\[11:0\\]
bits + 1 in the register LTDC_BPCR register. The last visible line of a frame is the programmed value of AAH\\[11:0\\]
bits in the LTDC_AWCR register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ltdc_l2wvpcr](index.html) module"]
pub struct LTDC_L2WVPCR_SPEC;
impl crate::RegisterSpec for LTDC_L2WVPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ltdc_l2wvpcr::R](R) reader structure"]
impl crate::Readable for LTDC_L2WVPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ltdc_l2wvpcr::W](W) writer structure"]
impl crate::Writable for LTDC_L2WVPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LTDC_L2WVPCR to value 0"]
impl crate::Resettable for LTDC_L2WVPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
