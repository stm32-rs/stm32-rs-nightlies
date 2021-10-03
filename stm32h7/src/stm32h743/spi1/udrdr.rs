#[doc = "Register `UDRDR` reader"]
pub struct R(crate::R<UDRDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UDRDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UDRDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UDRDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UDRDR` writer"]
pub struct W(crate::W<UDRDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UDRDR_SPEC>;
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
impl From<crate::W<UDRDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UDRDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UDRDR` reader - Data at slave underrun condition"]
pub struct UDRDR_R(crate::FieldReader<u32, u32>);
impl UDRDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        UDRDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UDRDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UDRDR` writer - Data at slave underrun condition"]
pub struct UDRDR_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    pub fn udrdr(&self) -> UDRDR_R {
        UDRDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data at slave underrun condition"]
    #[inline(always)]
    pub fn udrdr(&mut self) -> UDRDR_W {
        UDRDR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Underrun Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [udrdr](index.html) module"]
pub struct UDRDR_SPEC;
impl crate::RegisterSpec for UDRDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [udrdr::R](R) reader structure"]
impl crate::Readable for UDRDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [udrdr::W](W) writer structure"]
impl crate::Writable for UDRDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UDRDR to value 0"]
impl crate::Resettable for UDRDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
