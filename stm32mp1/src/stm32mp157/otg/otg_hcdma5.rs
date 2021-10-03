#[doc = "Register `OTG_HCDMA5` reader"]
pub struct R(crate::R<OTG_HCDMA5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCDMA5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCDMA5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCDMA5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCDMA5` writer"]
pub struct W(crate::W<OTG_HCDMA5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCDMA5_SPEC>;
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
impl From<crate::W<OTG_HCDMA5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCDMA5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAADDR` reader - DMAADDR"]
pub struct DMAADDR_R(crate::FieldReader<u32, u32>);
impl DMAADDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        DMAADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAADDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAADDR` writer - DMAADDR"]
pub struct DMAADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMAADDR"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DMAADDR_W {
        DMAADDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG host channel 5 DMA address register in buffer DMA \\[alternate\\]\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcdma5](index.html) module"]
pub struct OTG_HCDMA5_SPEC;
impl crate::RegisterSpec for OTG_HCDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcdma5::R](R) reader structure"]
impl crate::Readable for OTG_HCDMA5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcdma5::W](W) writer structure"]
impl crate::Writable for OTG_HCDMA5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HCDMA5 to value 0"]
impl crate::Resettable for OTG_HCDMA5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
