#[doc = "Register `K2LR` writer"]
pub struct W(crate::W<K2LR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<K2LR_SPEC>;
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
impl From<crate::W<K2LR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<K2LR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `b121` writer - b121"]
pub struct B121_W<'a> {
    w: &'a mut W,
}
impl<'a> B121_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
#[doc = "Field `k` writer - k96"]
pub struct K_W<'a> {
    w: &'a mut W,
}
impl<'a> K_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7fff_ffff) | (value as u32 & 0x7fff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bit 25 - b121"]
    #[inline(always)]
    pub fn b121(&mut self) -> B121_W {
        B121_W { w: self }
    }
    #[doc = "Bits 0:30 - k96"]
    #[inline(always)]
    pub fn k(&mut self) -> K_W {
        K_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "key registers\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [k2lr](index.html) module"]
pub struct K2LR_SPEC;
impl crate::RegisterSpec for K2LR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [k2lr::W](W) writer structure"]
impl crate::Writable for K2LR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets K2LR to value 0"]
impl crate::Resettable for K2LR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
