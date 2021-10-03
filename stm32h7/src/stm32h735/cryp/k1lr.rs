#[doc = "Register `K1LR` writer"]
pub struct W(crate::W<K1LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K1LR_SPEC>;
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
impl From<crate::W<K1LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K1LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `k1` writer - k160"]
pub struct K1_W<'a> {
    w: &'a mut W,
}
impl<'a> K1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - k160"]
    #[inline(always)]
    pub fn k1(&mut self) -> K1_W {
        K1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k1lr](index.html) module"]
pub struct K1LR_SPEC;
impl crate::RegisterSpec for K1LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k1lr::W](W) writer structure"]
impl crate::Writable for K1LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K1LR to value 0"]
impl crate::Resettable for K1LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
