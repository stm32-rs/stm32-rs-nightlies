#[doc = "Register `TAMP_ATSEEDR` writer"]
pub struct W(crate::W<TAMP_ATSEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_ATSEEDR_SPEC>;
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
impl From<crate::W<TAMP_ATSEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_ATSEEDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEED` writer - SEED"]
pub struct SEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SEED_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - SEED"]
    #[inline(always)]
    pub fn seed(&mut self) -> SEED_W {
        SEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_atseedr](index.html) module"]
pub struct TAMP_ATSEEDR_SPEC;
impl crate::RegisterSpec for TAMP_ATSEEDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tamp_atseedr::W](W) writer structure"]
impl crate::Writable for TAMP_ATSEEDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_ATSEEDR to value 0"]
impl crate::Resettable for TAMP_ATSEEDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
