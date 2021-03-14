#[doc = "Reader of register RCC_MP_BOOTCR"]
pub type R = crate::R<u32, super::RCC_MP_BOOTCR>;
#[doc = "Writer for register RCC_MP_BOOTCR"]
pub type W = crate::W<u32, super::RCC_MP_BOOTCR>;
#[doc = "Register RCC_MP_BOOTCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_BOOTCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `MCU_BEN`"]
pub type MCU_BEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MCU_BEN`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `MPU_BEN`"]
pub type MPU_BEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MPU_BEN`"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
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
}
