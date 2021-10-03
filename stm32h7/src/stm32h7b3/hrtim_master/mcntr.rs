#[doc = "Register `MCNTR` reader"]
pub struct R(crate::R<MCNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCNTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCNTR` writer"]
pub struct W(crate::W<MCNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCNTR_SPEC>;
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
impl From<crate::W<MCNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCNTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCNT` reader - Counter value"]
pub struct MCNT_R(crate::FieldReader<u16, u16>);
impl MCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        MCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCNT` writer - Counter value"]
pub struct MCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> MCNT_W {
        MCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Timer Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcntr](index.html) module"]
pub struct MCNTR_SPEC;
impl crate::RegisterSpec for MCNTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcntr::R](R) reader structure"]
impl crate::Readable for MCNTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcntr::W](W) writer structure"]
impl crate::Writable for MCNTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCNTR to value 0"]
impl crate::Resettable for MCNTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}