#[doc = "Register `ALRH` writer"]
pub struct W(crate::W<ALRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRH_SPEC>;
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
impl From<crate::W<ALRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRH` writer - RTC alarm register high"]
pub struct ALRH_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register high"]
    #[inline(always)]
    pub fn alrh(&mut self) -> ALRH_W {
        ALRH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Register High\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrh](index.html) module"]
pub struct ALRH_SPEC;
impl crate::RegisterSpec for ALRH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alrh::W](W) writer structure"]
impl crate::Writable for ALRH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRH to value 0xffff"]
impl crate::Resettable for ALRH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
