#[doc = "Register `PRLH` writer"]
pub struct W(crate::W<PRLH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLH_SPEC>;
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
impl From<crate::W<PRLH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRLH` writer - RTC Prescaler Load Register High"]
pub struct PRLH_W<'a> {
    w: &'a mut W,
}
impl<'a> PRLH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:3 - RTC Prescaler Load Register High"]
    #[inline(always)]
    pub fn prlh(&mut self) -> PRLH_W {
        PRLH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Prescaler Load Register High\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prlh](index.html) module"]
pub struct PRLH_SPEC;
impl crate::RegisterSpec for PRLH_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prlh::W](W) writer structure"]
impl crate::Writable for PRLH_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRLH to value 0"]
impl crate::Resettable for PRLH_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
