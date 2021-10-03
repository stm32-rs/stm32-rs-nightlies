#[doc = "Register `NSBOOTADD1R` writer"]
pub struct W(crate::W<NSBOOTADD1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NSBOOTADD1R_SPEC>;
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
impl From<crate::W<NSBOOTADD1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NSBOOTADD1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSBOOTADD1` writer - NSBOOTADD1"]
pub struct NSBOOTADD1_W<'a> {
    w: &'a mut W,
}
impl<'a> NSBOOTADD1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01ff_ffff << 7)) | ((value as u32 & 0x01ff_ffff) << 7);
        self.w
    }
}
impl W {
    #[doc = "Bits 7:31 - NSBOOTADD1"]
    #[inline(always)]
    pub fn nsbootadd1(&mut self) -> NSBOOTADD1_W {
        NSBOOTADD1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash non-secure boot address 1 register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nsbootadd1r](index.html) module"]
pub struct NSBOOTADD1R_SPEC;
impl crate::RegisterSpec for NSBOOTADD1R_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [nsbootadd1r::W](W) writer structure"]
impl crate::Writable for NSBOOTADD1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NSBOOTADD1R to value 0x0f"]
impl crate::Resettable for NSBOOTADD1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0f
    }
}
