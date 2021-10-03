#[doc = "Register `DDRPERFM_ICR` writer"]
pub struct W(crate::W<DDRPERFM_ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPERFM_ICR_SPEC>;
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
impl From<crate::W<DDRPERFM_ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPERFM_ICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OVF` writer - OVF"]
pub struct OVF_W<'a> {
    w: &'a mut W,
}
impl<'a> OVF_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - OVF"]
    #[inline(always)]
    pub fn ovf(&mut self) -> OVF_W {
        OVF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write-only register. A read request returns all zeros\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrperfm_icr](index.html) module"]
pub struct DDRPERFM_ICR_SPEC;
impl crate::RegisterSpec for DDRPERFM_ICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ddrperfm_icr::W](W) writer structure"]
impl crate::Writable for DDRPERFM_ICR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPERFM_ICR to value 0"]
impl crate::Resettable for DDRPERFM_ICR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
