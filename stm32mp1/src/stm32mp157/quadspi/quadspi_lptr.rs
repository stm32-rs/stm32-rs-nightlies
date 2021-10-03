#[doc = "Register `QUADSPI_LPTR` reader"]
pub struct R(crate::R<QUADSPI_LPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<QUADSPI_LPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<QUADSPI_LPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<QUADSPI_LPTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `QUADSPI_LPTR` writer"]
pub struct W(crate::W<QUADSPI_LPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<QUADSPI_LPTR_SPEC>;
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
impl From<crate::W<QUADSPI_LPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<QUADSPI_LPTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMEOUT` reader - TIMEOUT"]
pub struct TIMEOUT_R(crate::FieldReader<u16, u16>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - TIMEOUT"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TIMEOUT"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "QUADSPI low-power timeout register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [quadspi_lptr](index.html) module"]
pub struct QUADSPI_LPTR_SPEC;
impl crate::RegisterSpec for QUADSPI_LPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [quadspi_lptr::R](R) reader structure"]
impl crate::Readable for QUADSPI_LPTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [quadspi_lptr::W](W) writer structure"]
impl crate::Writable for QUADSPI_LPTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets QUADSPI_LPTR to value 0"]
impl crate::Resettable for QUADSPI_LPTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
