#[doc = "Register `C1ICR` reader"]
pub struct R(crate::R<C1ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1ICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1ICR` writer"]
pub struct W(crate::W<C1ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1ICR_SPEC>;
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
impl From<crate::W<C1ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISCm` reader - CPU(n) semaphore m clear bit"]
pub struct ISCM_R(crate::FieldReader<u32, u32>);
impl ISCM_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISCM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISCM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISCm` writer - CPU(n) semaphore m clear bit"]
pub struct ISCM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISCM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&self) -> ISCM_R {
        ISCM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(n) semaphore m clear bit"]
    #[inline(always)]
    pub fn iscm(&mut self) -> ISCM_W {
        ISCM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1icr](index.html) module"]
pub struct C1ICR_SPEC;
impl crate::RegisterSpec for C1ICR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1icr::R](R) reader structure"]
impl crate::Readable for C1ICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1icr::W](W) writer structure"]
impl crate::Writable for C1ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1ICR to value 0"]
impl crate::Resettable for C1ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
