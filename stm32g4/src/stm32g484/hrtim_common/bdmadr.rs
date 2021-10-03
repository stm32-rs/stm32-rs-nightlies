#[doc = "Register `BDMADR` writer"]
pub struct W(crate::W<BDMADR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDMADR_SPEC>;
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
impl From<crate::W<BDMADR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDMADR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BDMADR` writer - Burst DMA Data register"]
pub struct BDMADR_W<'a> {
    w: &'a mut W,
}
impl<'a> BDMADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - Burst DMA Data register"]
    #[inline(always)]
    pub fn bdmadr(&mut self) -> BDMADR_W {
        BDMADR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Burst DMA Data Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdmadr](index.html) module"]
pub struct BDMADR_SPEC;
impl crate::RegisterSpec for BDMADR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [bdmadr::W](W) writer structure"]
impl crate::Writable for BDMADR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BDMADR to value 0"]
impl crate::Resettable for BDMADR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
