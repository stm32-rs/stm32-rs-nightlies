#[doc = "Register `M3FAR` reader"]
pub struct R(crate::R<M3FAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M3FAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M3FAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M3FAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `M3FAR` writer"]
pub struct W(crate::W<M3FAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<M3FAR_SPEC>;
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
impl From<crate::W<M3FAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<M3FAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FADD` reader - ECC error failing address"]
pub struct FADD_R(crate::FieldReader<u32, u32>);
impl FADD_R {
    pub(crate) fn new(bits: u32) -> Self {
        FADD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FADD_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FADD` writer - ECC error failing address"]
pub struct FADD_W<'a> {
    w: &'a mut W,
}
impl<'a> FADD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&self) -> FADD_R {
        FADD_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ECC error failing address"]
    #[inline(always)]
    pub fn fadd(&mut self) -> FADD_W {
        FADD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RAMECC monitor x failing address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m3far](index.html) module"]
pub struct M3FAR_SPEC;
impl crate::RegisterSpec for M3FAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m3far::R](R) reader structure"]
impl crate::Readable for M3FAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [m3far::W](W) writer structure"]
impl crate::Writable for M3FAR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets M3FAR to value 0"]
impl crate::Resettable for M3FAR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
