#[doc = "Register `RCC_MP_GCR` reader"]
pub struct R(crate::R<RCC_MP_GCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_GCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_GCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_GCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_GCR` writer"]
pub struct W(crate::W<RCC_MP_GCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_GCR_SPEC>;
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
impl From<crate::W<RCC_MP_GCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_GCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOT_MCU` reader - BOOT_MCU"]
pub struct BOOT_MCU_R(crate::FieldReader<bool, bool>);
impl BOOT_MCU_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOOT_MCU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOOT_MCU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOOT_MCU` writer - BOOT_MCU"]
pub struct BOOT_MCU_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOT_MCU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&self) -> BOOT_MCU_R {
        BOOT_MCU_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BOOT_MCU"]
    #[inline(always)]
    pub fn boot_mcu(&mut self) -> BOOT_MCU_W {
        BOOT_MCU_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "The register contains global control bits. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_gcr](index.html) module"]
pub struct RCC_MP_GCR_SPEC;
impl crate::RegisterSpec for RCC_MP_GCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_gcr::R](R) reader structure"]
impl crate::Readable for RCC_MP_GCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_gcr::W](W) writer structure"]
impl crate::Writable for RCC_MP_GCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_GCR to value 0"]
impl crate::Resettable for RCC_MP_GCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
