#[doc = "Register `MACRWKPFR` reader"]
pub struct R(crate::R<MACRWKPFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACRWKPFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACRWKPFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACRWKPFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACRWKPFR` writer"]
pub struct W(crate::W<MACRWKPFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACRWKPFR_SPEC>;
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
impl From<crate::W<MACRWKPFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACRWKPFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MACRWKPFR` reader - Remote wakeup packet filter"]
pub struct MACRWKPFR_R(crate::FieldReader<u32, u32>);
impl MACRWKPFR_R {
    pub(crate) fn new(bits: u32) -> Self {
        MACRWKPFR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MACRWKPFR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MACRWKPFR` writer - Remote wakeup packet filter"]
pub struct MACRWKPFR_W<'a> {
    w: &'a mut W,
}
impl<'a> MACRWKPFR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Remote wakeup packet filter"]
    #[inline(always)]
    pub fn macrwkpfr(&self) -> MACRWKPFR_R {
        MACRWKPFR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Remote wakeup packet filter"]
    #[inline(always)]
    pub fn macrwkpfr(&mut self) -> MACRWKPFR_W {
        MACRWKPFR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Remove wakeup packet filter register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macrwkpfr](index.html) module"]
pub struct MACRWKPFR_SPEC;
impl crate::RegisterSpec for MACRWKPFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macrwkpfr::R](R) reader structure"]
impl crate::Readable for MACRWKPFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macrwkpfr::W](W) writer structure"]
impl crate::Writable for MACRWKPFR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACRWKPFR to value 0"]
impl crate::Resettable for MACRWKPFR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
