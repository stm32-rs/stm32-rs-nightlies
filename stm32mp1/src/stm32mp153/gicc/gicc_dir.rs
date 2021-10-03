#[doc = "Register `GICC_DIR` writer"]
pub struct W(crate::W<GICC_DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GICC_DIR_SPEC>;
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
impl From<crate::W<GICC_DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GICC_DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERRUPT_ID` writer - INTERRUPT_ID"]
pub struct INTERRUPT_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `CPUID` writer - CPUID"]
pub struct CPUID_W<'a> {
    w: &'a mut W,
}
impl<'a> CPUID_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:9 - INTERRUPT_ID"]
    #[inline(always)]
    pub fn interrupt_id(&mut self) -> INTERRUPT_ID_W {
        INTERRUPT_ID_W { w: self }
    }
    #[doc = "Bit 10 - CPUID"]
    #[inline(always)]
    pub fn cpuid(&mut self) -> CPUID_W {
        CPUID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GICC deactivate interrupt register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicc_dir](index.html) module"]
pub struct GICC_DIR_SPEC;
impl crate::RegisterSpec for GICC_DIR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gicc_dir::W](W) writer structure"]
impl crate::Writable for GICC_DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GICC_DIR to value 0"]
impl crate::Resettable for GICC_DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
