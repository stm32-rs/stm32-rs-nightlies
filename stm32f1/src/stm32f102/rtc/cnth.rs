#[doc = "Register `CNTH` reader"]
pub struct R(crate::R<CNTH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNTH` writer"]
pub struct W(crate::W<CNTH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTH_SPEC>;
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
impl From<crate::W<CNTH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNTH` reader - RTC counter register high"]
pub struct CNTH_R(crate::FieldReader<u16, u16>);
impl CNTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        CNTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CNTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CNTH` writer - RTC counter register high"]
pub struct CNTH_W<'a> {
    w: &'a mut W,
}
impl<'a> CNTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&self) -> CNTH_R {
        CNTH_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC counter register high"]
    #[inline(always)]
    pub fn cnth(&mut self) -> CNTH_W {
        CNTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Counter Register High\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cnth](index.html) module"]
pub struct CNTH_SPEC;
impl crate::RegisterSpec for CNTH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cnth::R](R) reader structure"]
impl crate::Readable for CNTH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cnth::W](W) writer structure"]
impl crate::Writable for CNTH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNTH to value 0"]
impl crate::Resettable for CNTH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
