#[doc = "Register `HDP_GPOSET` writer"]
pub struct W(crate::W<HDP_GPOSET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HDP_GPOSET_SPEC>;
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
impl From<crate::W<HDP_GPOSET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HDP_GPOSET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HDPGPOSET` writer - HDPGPOSET"]
pub struct HDPGPOSET_W<'a> {
    w: &'a mut W,
}
impl<'a> HDPGPOSET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:7 - HDPGPOSET"]
    #[inline(always)]
    pub fn hdpgposet(&mut self) -> HDPGPOSET_W {
        HDPGPOSET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HDP GPO set\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hdp_gposet](index.html) module"]
pub struct HDP_GPOSET_SPEC;
impl crate::RegisterSpec for HDP_GPOSET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [hdp_gposet::W](W) writer structure"]
impl crate::Writable for HDP_GPOSET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HDP_GPOSET to value 0"]
impl crate::Resettable for HDP_GPOSET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
