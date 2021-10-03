#[doc = "Register `C1IER` reader"]
pub struct R(crate::R<C1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `C1IER` writer"]
pub struct W(crate::W<C1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IER_SPEC>;
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
impl From<crate::W<C1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ISEm` reader - CPU(n) semaphore m enable bit"]
pub struct ISEM_R(crate::FieldReader<u32, u32>);
impl ISEM_R {
    pub(crate) fn new(bits: u32) -> Self {
        ISEM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISEM_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ISEm` writer - CPU(n) semaphore m enable bit"]
pub struct ISEM_W<'a> {
    w: &'a mut W,
}
impl<'a> ISEM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CPU(n) semaphore m enable bit"]
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CPU(n) semaphore m enable bit"]
    #[inline(always)]
    pub fn isem(&mut self) -> ISEM_W {
        ISEM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HSEM Interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c1ier](index.html) module"]
pub struct C1IER_SPEC;
impl crate::RegisterSpec for C1IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c1ier::R](R) reader structure"]
impl crate::Readable for C1IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [c1ier::W](W) writer structure"]
impl crate::Writable for C1IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets C1IER to value 0"]
impl crate::Resettable for C1IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
