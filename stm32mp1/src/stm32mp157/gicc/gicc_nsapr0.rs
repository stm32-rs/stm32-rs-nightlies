#[doc = "Register `GICC_NSAPR0` reader"]
pub struct R(crate::R<GICC_NSAPR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICC_NSAPR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICC_NSAPR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICC_NSAPR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GICC_NSAPR0` writer"]
pub struct W(crate::W<GICC_NSAPR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_NSAPR0_SPEC>;
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
impl From<crate::W<GICC_NSAPR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_NSAPR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSAPR0` reader - NSAPR0"]
pub struct NSAPR0_R(crate::FieldReader<u32, u32>);
impl NSAPR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        NSAPR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSAPR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSAPR0` writer - NSAPR0"]
pub struct NSAPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> NSAPR0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - NSAPR0"]
    #[inline(always)]
    pub fn nsapr0(&self) -> NSAPR0_R {
        NSAPR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - NSAPR0"]
    #[inline(always)]
    pub fn nsapr0(&mut self) -> NSAPR0_W {
        NSAPR0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC non-secure active priority register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_nsapr0](index.html) module"]
pub struct GICC_NSAPR0_SPEC;
impl crate::RegisterSpec for GICC_NSAPR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicc_nsapr0::R](R) reader structure"]
impl crate::Readable for GICC_NSAPR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gicc_nsapr0::W](W) writer structure"]
impl crate::Writable for GICC_NSAPR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICC_NSAPR0 to value 0"]
impl crate::Resettable for GICC_NSAPR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
