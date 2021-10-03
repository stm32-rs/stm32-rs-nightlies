#[doc = "Register `GICV_PMR` reader"]
pub struct R(crate::R<GICV_PMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICV_PMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICV_PMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICV_PMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICV_PMR` writer"]
pub struct W(crate::W<GICV_PMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICV_PMR_SPEC>;
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
impl From<crate::W<GICV_PMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICV_PMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIORITY` reader - PRIORITY"]
pub struct PRIORITY_R(crate::FieldReader<u8, u8>);
impl PRIORITY_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRIORITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRIORITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRIORITY` writer - PRIORITY"]
pub struct PRIORITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIORITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 3)) | ((value as u32 & 0x1f) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&self) -> PRIORITY_R {
        PRIORITY_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:7 - PRIORITY"]
    #[inline(always)]
    pub fn priority(&mut self) -> PRIORITY_W {
        PRIORITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICV VM priority mask register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicv_pmr](index.html) module"]
pub struct GICV_PMR_SPEC;
impl crate::RegisterSpec for GICV_PMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicv_pmr::R](R) reader structure"]
impl crate::Readable for GICV_PMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicv_pmr::W](W) writer structure"]
impl crate::Writable for GICV_PMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICV_PMR to value 0"]
impl crate::Resettable for GICV_PMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
