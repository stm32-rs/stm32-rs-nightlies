#[doc = "Register `RCC_MP_BOOTCR` reader"]
pub struct R(crate::R<RCC_MP_BOOTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MP_BOOTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MP_BOOTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MP_BOOTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MP_BOOTCR` writer"]
pub struct W(crate::W<RCC_MP_BOOTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MP_BOOTCR_SPEC>;
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
impl From<crate::W<RCC_MP_BOOTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MP_BOOTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCU_BEN` reader - MCU_BEN"]
pub struct MCU_BEN_R(crate::FieldReader<bool, bool>);
impl MCU_BEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_BEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_BEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_BEN` writer - MCU_BEN"]
pub struct MCU_BEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_BEN_W<'a> {
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
#[doc = "Field `MPU_BEN` reader - MPU_BEN"]
pub struct MPU_BEN_R(crate::FieldReader<bool, bool>);
impl MPU_BEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPU_BEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPU_BEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPU_BEN` writer - MPU_BEN"]
pub struct MPU_BEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_BEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    pub fn mcu_ben(&self) -> MCU_BEN_R {
        MCU_BEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    pub fn mpu_ben(&self) -> MPU_BEN_R {
        MPU_BEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCU_BEN"]
    #[inline(always)]
    pub fn mcu_ben(&mut self) -> MCU_BEN_W {
        MCU_BEN_W { w: self }
    }
    #[doc = "Bit 1 - MPU_BEN"]
    #[inline(always)]
    pub fn mpu_ben(&mut self) -> MPU_BEN_W {
        MPU_BEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the HOLD boot function when the system exits from Standby. Refer to Section: MCU HOLD_BOOT after processor reset. This register is reset when a system reset occurs, but not when the circuit exits from Standby (app_rst reset).If TZEN = , this register can only be modified in secure mode. This register can only be accessed by the MPU.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mp_bootcr](index.html) module"]
pub struct RCC_MP_BOOTCR_SPEC;
impl crate::RegisterSpec for RCC_MP_BOOTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mp_bootcr::R](R) reader structure"]
impl crate::Readable for RCC_MP_BOOTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mp_bootcr::W](W) writer structure"]
impl crate::Writable for RCC_MP_BOOTCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MP_BOOTCR to value 0"]
impl crate::Resettable for RCC_MP_BOOTCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
