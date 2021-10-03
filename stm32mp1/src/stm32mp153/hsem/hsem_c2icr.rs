#[doc = "Register `HSEM_C2ICR` reader"]
pub struct R(crate::R<HSEM_C2ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSEM_C2ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSEM_C2ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSEM_C2ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSEM_C2ICR` writer"]
pub struct W(crate::W<HSEM_C2ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSEM_C2ICR_SPEC>;
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
impl From<crate::W<HSEM_C2ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSEM_C2ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISC` reader - ISC"]
pub struct ISC_R(crate::FieldReader<u32, u32>);
impl ISC_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISC` writer - ISC"]
pub struct ISC_W<'a> {
    w: &'a mut W,
}
impl<'a> ISC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    pub fn isc(&self) -> ISC_R {
        ISC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - ISC"]
    #[inline(always)]
    pub fn isc(&mut self) -> ISC_W {
        ISC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM i2terrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsem_c2icr](index.html) module"]
pub struct HSEM_C2ICR_SPEC;
impl crate::RegisterSpec for HSEM_C2ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsem_c2icr::R](R) reader structure"]
impl crate::Readable for HSEM_C2ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsem_c2icr::W](W) writer structure"]
impl crate::Writable for HSEM_C2ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSEM_C2ICR to value 0"]
impl crate::Resettable for HSEM_C2ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
