#[doc = "Register `SYSCFG_CMPENSETR` reader"]
pub struct R(crate::R<SYSCFG_CMPENSETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CMPENSETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CMPENSETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CMPENSETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CMPENSETR` writer"]
pub struct W(crate::W<SYSCFG_CMPENSETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CMPENSETR_SPEC>;
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
impl From<crate::W<SYSCFG_CMPENSETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CMPENSETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPU_EN` reader - MPU_EN"]
pub struct MPU_EN_R(crate::FieldReader<bool, bool>);
impl MPU_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MPU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MPU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MPU_EN` writer - MPU_EN"]
pub struct MPU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MPU_EN_W<'a> {
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
#[doc = "Field `MCU_EN` reader - MCU_EN"]
pub struct MCU_EN_R(crate::FieldReader<bool, bool>);
impl MCU_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCU_EN` writer - MCU_EN"]
pub struct MCU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MCU_EN_W<'a> {
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
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    pub fn mpu_en(&self) -> MPU_EN_R {
        MPU_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    pub fn mcu_en(&self) -> MCU_EN_R {
        MCU_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MPU_EN"]
    #[inline(always)]
    pub fn mpu_en(&mut self) -> MPU_EN_W {
        MPU_EN_W { w: self }
    }
    #[doc = "Bit 1 - MCU_EN"]
    #[inline(always)]
    pub fn mcu_en(&mut self) -> MCU_EN_W {
        MCU_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SYSCFG compensation cell enable set register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cmpensetr](index.html) module"]
pub struct SYSCFG_CMPENSETR_SPEC;
impl crate::RegisterSpec for SYSCFG_CMPENSETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cmpensetr::R](R) reader structure"]
impl crate::Readable for SYSCFG_CMPENSETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cmpensetr::W](W) writer structure"]
impl crate::Writable for SYSCFG_CMPENSETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYSCFG_CMPENSETR to value 0"]
impl crate::Resettable for SYSCFG_CMPENSETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
