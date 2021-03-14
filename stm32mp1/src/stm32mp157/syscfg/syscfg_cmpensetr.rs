#[doc = "Reader of register SYSCFG_CMPENSETR"]
pub type R = crate::R<u32, super::SYSCFG_CMPENSETR>;
#[doc = "Writer for register SYSCFG_CMPENSETR"]
pub type W = crate::W<u32, super::SYSCFG_CMPENSETR>;
#[doc = "Register SYSCFG_CMPENSETR `reset()`'s with value 0"]
impl crate::ResetValue for super::SYSCFG_CMPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MPU_EN`"]
pub type MPU_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPU_EN`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MCU_EN`"]
pub type MCU_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_EN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
}
