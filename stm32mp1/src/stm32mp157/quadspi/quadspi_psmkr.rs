#[doc = "Register `QUADSPI_PSMKR` reader"]
pub struct R(crate::R<QUADSPI_PSMKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_PSMKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_PSMKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_PSMKR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_PSMKR` writer"]
pub struct W(crate::W<QUADSPI_PSMKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_PSMKR_SPEC>;
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
impl From<crate::W<QUADSPI_PSMKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_PSMKR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - MASK"]
pub struct MASK_R(crate::FieldReader<u32, u32>);
impl MASK_R {
    pub(crate) fn new(bits: u32) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - MASK"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - MASK"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - MASK"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI polling status mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_psmkr](index.html) module"]
pub struct QUADSPI_PSMKR_SPEC;
impl crate::RegisterSpec for QUADSPI_PSMKR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_psmkr::R](R) reader structure"]
impl crate::Readable for QUADSPI_PSMKR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_psmkr::W](W) writer structure"]
impl crate::Writable for QUADSPI_PSMKR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUADSPI_PSMKR to value 0"]
impl crate::Resettable for QUADSPI_PSMKR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
