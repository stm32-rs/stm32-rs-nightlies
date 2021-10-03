#[doc = "Register `R1KEYR3` writer"]
pub struct W(crate::W<R1KEYR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<R1KEYR3_SPEC>;
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
impl From<crate::W<R1KEYR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<R1KEYR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGx_KEY` writer - REGx_KEY"]
pub struct REGX_KEY_W<'a> {
    w: &'a mut W,
}
impl<'a> REGX_KEY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:31 - REGx_KEY"]
    #[inline(always)]
    pub fn regx_key(&mut self) -> REGX_KEY_W {
        REGX_KEY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTFDEC region x key register 3\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [r1keyr3](index.html) module"]
pub struct R1KEYR3_SPEC;
impl crate::RegisterSpec for R1KEYR3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [r1keyr3::W](W) writer structure"]
impl crate::Writable for R1KEYR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets R1KEYR3 to value 0"]
impl crate::Resettable for R1KEYR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
