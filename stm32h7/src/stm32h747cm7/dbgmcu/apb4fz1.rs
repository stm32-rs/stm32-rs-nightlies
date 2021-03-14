#[doc = "Reader of register APB4FZ1"]
pub type R = crate::R<u32, super::APB4FZ1>;
#[doc = "Writer for register APB4FZ1"]
pub type W = crate::W<u32, super::APB4FZ1>;
#[doc = "Register APB4FZ1 `reset()`'s with value 0"]
impl crate::ResetValue for super::APB4FZ1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `IWDG1`"]
pub type IWDG1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG1`"]
pub struct IWDG1_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `RTC`"]
pub type RTC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTC`"]
pub struct RTC_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `LPTIM5`"]
pub type LPTIM5_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM5`"]
pub struct LPTIM5_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM5_W<'a> {
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
#[doc = "Reader of field `LPTIM4`"]
pub type LPTIM4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM4`"]
pub struct LPTIM4_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM4_W<'a> {
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
#[doc = "Reader of field `LPTIM3`"]
pub type LPTIM3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3`"]
pub struct LPTIM3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3_W<'a> {
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
#[doc = "Reader of field `LPTIM2`"]
pub type LPTIM2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2`"]
pub struct LPTIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2_W<'a> {
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
#[doc = "Reader of field `I2C4`"]
pub type I2C4_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4`"]
pub struct I2C4_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `IWDG2`"]
pub type IWDG2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG2`"]
pub struct IWDG2_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug mode"]
    #[inline(always)]
    pub fn iwdg1(&self) -> IWDG1_R {
        IWDG1_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug mode"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug mode"]
    #[inline(always)]
    pub fn lptim5(&self) -> LPTIM5_R {
        LPTIM5_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug mode"]
    #[inline(always)]
    pub fn lptim4(&self) -> LPTIM4_R {
        LPTIM4_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug mode"]
    #[inline(always)]
    pub fn lptim3(&self) -> LPTIM3_R {
        LPTIM3_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug mode"]
    #[inline(always)]
    pub fn lptim2(&self) -> LPTIM2_R {
        LPTIM2_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug mode"]
    #[inline(always)]
    pub fn i2c4(&self) -> I2C4_R {
        I2C4_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Independent watchdog for D2 stop when Cortex-M7 in debug mode"]
    #[inline(always)]
    pub fn iwdg2(&self) -> IWDG2_R {
        IWDG2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug mode"]
    #[inline(always)]
    pub fn iwdg1(&mut self) -> IWDG1_W {
        IWDG1_W { w: self }
    }
    #[doc = "Bit 16 - RTC stop in debug mode"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W {
        RTC_W { w: self }
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug mode"]
    #[inline(always)]
    pub fn lptim5(&mut self) -> LPTIM5_W {
        LPTIM5_W { w: self }
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug mode"]
    #[inline(always)]
    pub fn lptim4(&mut self) -> LPTIM4_W {
        LPTIM4_W { w: self }
    }
    #[doc = "Bit 10 - LPTIM3 stop in debug mode"]
    #[inline(always)]
    pub fn lptim3(&mut self) -> LPTIM3_W {
        LPTIM3_W { w: self }
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug mode"]
    #[inline(always)]
    pub fn lptim2(&mut self) -> LPTIM2_W {
        LPTIM2_W { w: self }
    }
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug mode"]
    #[inline(always)]
    pub fn i2c4(&mut self) -> I2C4_W {
        I2C4_W { w: self }
    }
    #[doc = "Bit 19 - Independent watchdog for D2 stop when Cortex-M7 in debug mode"]
    #[inline(always)]
    pub fn iwdg2(&mut self) -> IWDG2_W {
        IWDG2_W { w: self }
    }
}
