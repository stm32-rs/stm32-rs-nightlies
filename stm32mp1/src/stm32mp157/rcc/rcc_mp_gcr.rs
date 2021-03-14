#[doc = "Reader of register RCC_MP_GCR"]
pub type R = crate::R<u32, super::RCC_MP_GCR>;
#[doc = "Writer for register RCC_MP_GCR"]
pub type W = crate::W<u32, super::RCC_MP_GCR>;
#[doc = "Register RCC_MP_GCR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_GCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BOOT_MCU`"]
pub type BOOT_MCU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOT_MCU`"]
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
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
}
