#[doc = "Register `ALRL` writer"]
pub struct W(crate::W<ALRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ALRL_SPEC>;
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
impl From<crate::W<ALRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ALRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALRL` writer - RTC alarm register low"]
pub struct ALRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ALRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC alarm register low"]
    #[inline(always)]
    pub fn alrl(&mut self) -> ALRL_W {
        ALRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Alarm Register Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [alrl](index.html) module"]
pub struct ALRL_SPEC;
impl crate::RegisterSpec for ALRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [alrl::W](W) writer structure"]
impl crate::Writable for ALRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ALRL to value 0xffff"]
impl crate::Resettable for ALRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
