#[doc = "Reader of register C2IMR1"]
pub type R = crate::R<u32, super::C2IMR1>;
#[doc = "Writer for register C2IMR1"]
pub type W = crate::W<u32, super::C2IMR1>;
#[doc = "Register C2IMR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C2IMR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCSTAMP`"]
pub type RTCSTAMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCSTAMP`"]
pub struct RTCSTAMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCSTAMP_W<'a> {
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
#[doc = "Reader of field `RTCWKUP`"]
pub type RTCWKUP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCWKUP`"]
pub struct RTCWKUP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCWKUP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `RTCALARM`"]
pub type RTCALARM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCALARM`"]
pub struct RTCALARM_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCALARM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `RCC`"]
pub type RCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCC`"]
pub struct RCC_W<'a> {
    w: &'a mut W,
}
impl<'a> RCC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `FLASH`"]
pub type FLASH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASH`"]
pub struct FLASH_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `PKA`"]
pub type PKA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKA`"]
pub struct PKA_W<'a> {
    w: &'a mut W,
}
impl<'a> PKA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `RNG`"]
pub type RNG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG`"]
pub struct RNG_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AES1`"]
pub type AES1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AES1`"]
pub struct AES1_W<'a> {
    w: &'a mut W,
}
impl<'a> AES1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `COMP`"]
pub type COMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `COMP`"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `ADC`"]
pub type ADC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC`"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&self) -> RTCSTAMP_R {
        RTCSTAMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&self) -> RTCWKUP_R {
        RTCWKUP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&self) -> RTCALARM_R {
        RTCALARM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&self) -> RCC_R {
        RCC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&self) -> PKA_R {
        PKA_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&self) -> RNG_R {
        RNG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&self) -> AES1_R {
        AES1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral RTCSTAMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcstamp(&mut self) -> RTCSTAMP_W {
        RTCSTAMP_W { w: self }
    }
    #[doc = "Bit 3 - Peripheral RTCWKUP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcwkup(&mut self) -> RTCWKUP_W {
        RTCWKUP_W { w: self }
    }
    #[doc = "Bit 4 - Peripheral RTCALARM interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rtcalarm(&mut self) -> RTCALARM_W {
        RTCALARM_W { w: self }
    }
    #[doc = "Bit 5 - Peripheral RCC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rcc(&mut self) -> RCC_W {
        RCC_W { w: self }
    }
    #[doc = "Bit 6 - Peripheral FLASH interrupt mask to CPU2"]
    #[inline(always)]
    pub fn flash(&mut self) -> FLASH_W {
        FLASH_W { w: self }
    }
    #[doc = "Bit 8 - Peripheral PKA interrupt mask to CPU2"]
    #[inline(always)]
    pub fn pka(&mut self) -> PKA_W {
        PKA_W { w: self }
    }
    #[doc = "Bit 9 - Peripheral RNG interrupt mask to CPU2"]
    #[inline(always)]
    pub fn rng(&mut self) -> RNG_W {
        RNG_W { w: self }
    }
    #[doc = "Bit 10 - Peripheral AES1 interrupt mask to CPU2"]
    #[inline(always)]
    pub fn aes1(&mut self) -> AES1_W {
        AES1_W { w: self }
    }
    #[doc = "Bit 11 - Peripheral COMP interrupt mask to CPU2"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Bit 12 - Peripheral ADC interrupt mask to CPU2"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
}
