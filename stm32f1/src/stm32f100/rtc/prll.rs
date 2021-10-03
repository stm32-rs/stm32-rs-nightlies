#[doc = "Register `PRLL` writer"]
pub struct W(crate::W<PRLL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRLL_SPEC>;
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
impl From<crate::W<PRLL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRLL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRLL` writer - RTC Prescaler Divider Register Low"]
pub struct PRLL_W<'a> {
    w: &'a mut W,
}
impl<'a> PRLL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:15 - RTC Prescaler Divider Register Low"]
    #[inline(always)]
    pub fn prll(&mut self) -> PRLL_W {
        PRLL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Prescaler Load Register Low\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prll](index.html) module"]
pub struct PRLL_SPEC;
impl crate::RegisterSpec for PRLL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [prll::W](W) writer structure"]
impl crate::Writable for PRLL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PRLL to value 0x8000"]
impl crate::Resettable for PRLL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
