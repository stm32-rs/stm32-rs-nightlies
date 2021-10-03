#[doc = "Register `PWRCR` reader"]
pub struct R(crate::R<PWRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRCR` writer"]
pub struct W(crate::W<PWRCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRCR_SPEC>;
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
impl From<crate::W<PWRCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub struct ODEN_R(crate::FieldReader<u8, u8>);
impl ODEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        ODEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ODEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub struct ODEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> ODEN_R {
        ODEN_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&mut self) -> ODEN_W {
        ODEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG power control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrcr](index.html) module"]
pub struct PWRCR_SPEC;
impl crate::RegisterSpec for PWRCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrcr::R](R) reader structure"]
impl crate::Readable for PWRCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrcr::W](W) writer structure"]
impl crate::Writable for PWRCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PWRCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
