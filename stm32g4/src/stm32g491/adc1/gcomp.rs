#[doc = "Register `GCOMP` reader"]
pub struct R(crate::R<GCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GCOMP` writer"]
pub struct W(crate::W<GCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GCOMP_SPEC>;
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
impl From<crate::W<GCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GCOMPCOEFF` reader - Gain compensation coefficient"]
pub struct GCOMPCOEFF_R(crate::FieldReader<u16, u16>);
impl GCOMPCOEFF_R {
    pub(crate) fn new(bits: u16) -> Self {
        GCOMPCOEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GCOMPCOEFF_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GCOMPCOEFF` writer - Gain compensation coefficient"]
pub struct GCOMPCOEFF_W<'a> {
    w: &'a mut W,
}
impl<'a> GCOMPCOEFF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | (value as u32 & 0x3fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    pub fn gcompcoeff(&self) -> GCOMPCOEFF_R {
        GCOMPCOEFF_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Gain compensation coefficient"]
    #[inline(always)]
    pub fn gcompcoeff(&mut self) -> GCOMPCOEFF_W {
        GCOMPCOEFF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Gain compensation Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gcomp](index.html) module"]
pub struct GCOMP_SPEC;
impl crate::RegisterSpec for GCOMP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gcomp::R](R) reader structure"]
impl crate::Readable for GCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gcomp::W](W) writer structure"]
impl crate::Writable for GCOMP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GCOMP to value 0"]
impl crate::Resettable for GCOMP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
