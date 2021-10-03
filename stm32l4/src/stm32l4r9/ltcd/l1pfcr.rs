#[doc = "Register `L1PFCR` reader"]
pub struct R(crate::R<L1PFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L1PFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L1PFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L1PFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `L1PFCR` writer"]
pub struct W(crate::W<L1PFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<L1PFCR_SPEC>;
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
impl From<crate::W<L1PFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<L1PFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PF` reader - Pixel Format"]
pub struct PF_R(crate::FieldReader<u8, u8>);
impl PF_R {
    pub(crate) fn new(bits: u8) -> Self {
        PF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PF_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PF` writer - Pixel Format"]
pub struct PF_W<'a> {
    w: &'a mut W,
}
impl<'a> PF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Pixel Format"]
    #[inline(always)]
    pub fn pf(&mut self) -> PF_W {
        PF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LTDC Layer Pixel Format Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l1pfcr](index.html) module"]
pub struct L1PFCR_SPEC;
impl crate::RegisterSpec for L1PFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l1pfcr::R](R) reader structure"]
impl crate::Readable for L1PFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [l1pfcr::W](W) writer structure"]
impl crate::Writable for L1PFCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets L1PFCR to value 0"]
impl crate::Resettable for L1PFCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
