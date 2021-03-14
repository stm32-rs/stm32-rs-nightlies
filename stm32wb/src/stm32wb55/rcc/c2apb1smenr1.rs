#[doc = "Reader of register C2APB1SMENR1"]
pub type R = crate::R<u32, super::C2APB1SMENR1>;
#[doc = "Writer for register C2APB1SMENR1"]
pub type W = crate::W<u32, super::C2APB1SMENR1>;
#[doc = "Register C2APB1SMENR1 `reset()`'s with value 0x85a0_4601"]
impl crate::ResetValue for super::C2APB1SMENR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x85a0_4601
    }
}
#[doc = "Reader of field `LPTIM1SMEN`"]
pub type LPTIM1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM1SMEN`"]
pub struct LPTIM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `USBSMEN`"]
pub type USBSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBSMEN`"]
pub struct USBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `CRSMEN`"]
pub type CRSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRSMEN`"]
pub struct CRSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `I2C3SMEN`"]
pub type I2C3SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3SMEN`"]
pub struct I2C3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `I2C1SMEN`"]
pub type I2C1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1SMEN`"]
pub struct I2C1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `SPI2SMEN`"]
pub type SPI2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI2SMEN`"]
pub struct SPI2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI2SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `RTCAPBSMEN`"]
pub type RTCAPBSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAPBSMEN`"]
pub struct RTCAPBSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBSMEN_W<'a> {
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
#[doc = "Reader of field `LCDSMEN`"]
pub type LCDSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCDSMEN`"]
pub struct LCDSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LCDSMEN_W<'a> {
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
#[doc = "Reader of field `TIM2SMEN`"]
pub type TIM2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIM2SMEN`"]
pub struct TIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TIM2SMEN_W<'a> {
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
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&self) -> LPTIM1SMEN_R {
        LPTIM1SMEN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&self) -> USBSMEN_R {
        USBSMEN_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn crsmen(&self) -> CRSMEN_R {
        CRSMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&self) -> I2C3SMEN_R {
        I2C3SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&self) -> I2C1SMEN_R {
        I2C1SMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&self) -> SPI2SMEN_R {
        SPI2SMEN_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&self) -> RTCAPBSMEN_R {
        RTCAPBSMEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lcdsmen(&self) -> LCDSMEN_R {
        LCDSMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Low power timer 1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lptim1smen(&mut self) -> LPTIM1SMEN_W {
        LPTIM1SMEN_W { w: self }
    }
    #[doc = "Bit 26 - USB FS clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn usbsmen(&mut self) -> USBSMEN_W {
        USBSMEN_W { w: self }
    }
    #[doc = "Bit 24 - CRS clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn crsmen(&mut self) -> CRSMEN_W {
        CRSMEN_W { w: self }
    }
    #[doc = "Bit 23 - I2C3 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn i2c3smen(&mut self) -> I2C3SMEN_W {
        I2C3SMEN_W { w: self }
    }
    #[doc = "Bit 21 - I2C1 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn i2c1smen(&mut self) -> I2C1SMEN_W {
        I2C1SMEN_W { w: self }
    }
    #[doc = "Bit 14 - SPI2 clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn spi2smen(&mut self) -> SPI2SMEN_W {
        SPI2SMEN_W { w: self }
    }
    #[doc = "Bit 10 - RTC APB clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn rtcapbsmen(&mut self) -> RTCAPBSMEN_W {
        RTCAPBSMEN_W { w: self }
    }
    #[doc = "Bit 9 - LCD clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn lcdsmen(&mut self) -> LCDSMEN_W {
        LCDSMEN_W { w: self }
    }
    #[doc = "Bit 0 - TIM2 timer clocks enable during CPU2 Sleep mode"]
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W {
        TIM2SMEN_W { w: self }
    }
}
