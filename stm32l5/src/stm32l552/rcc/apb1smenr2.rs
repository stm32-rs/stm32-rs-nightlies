#[doc = "Reader of register APB1SMENR2"]
pub type R = crate::R<u32, super::APB1SMENR2>;
#[doc = "Writer for register APB1SMENR2"]
pub type W = crate::W<u32, super::APB1SMENR2>;
#[doc = "Register APB1SMENR2 `reset()`'s with value 0x00a0_0223"]
impl crate::ResetValue for super::APB1SMENR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x00a0_0223
    }
}
#[doc = "Reader of field `LPUART1SMEN`"]
pub type LPUART1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPUART1SMEN`"]
pub struct LPUART1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SMEN_W<'a> {
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
#[doc = "Reader of field `I2C4SMEN`"]
pub type I2C4SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4SMEN`"]
pub struct I2C4SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SMEN_W<'a> {
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
#[doc = "Reader of field `LPTIM2SMEN`"]
pub type LPTIM2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM2SMEN`"]
pub struct LPTIM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SMEN_W<'a> {
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
#[doc = "Reader of field `LPTIM3SMEN`"]
pub type LPTIM3SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPTIM3SMEN`"]
pub struct LPTIM3SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SMEN_W<'a> {
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
#[doc = "Reader of field `FDCAN1SMEN`"]
pub type FDCAN1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FDCAN1SMEN`"]
pub struct FDCAN1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCAN1SMEN_W<'a> {
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
#[doc = "Reader of field `USBFSSMEN`"]
pub type USBFSSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USBFSSMEN`"]
pub struct USBFSSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USBFSSMEN_W<'a> {
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
#[doc = "Reader of field `UCPD1SMEN`"]
pub type UCPD1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UCPD1SMEN`"]
pub struct UCPD1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UCPD1SMEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&self) -> LPUART1SMEN_R {
        LPUART1SMEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&self) -> I2C4SMEN_R {
        I2C4SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LPTIM3SMEN"]
    #[inline(always)]
    pub fn lptim3smen(&self) -> LPTIM3SMEN_R {
        LPTIM3SMEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 9 - FDCAN1SMEN"]
    #[inline(always)]
    pub fn fdcan1smen(&self) -> FDCAN1SMEN_R {
        FDCAN1SMEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USBFSSMEN"]
    #[inline(always)]
    pub fn usbfssmen(&self) -> USBFSSMEN_R {
        USBFSSMEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 23 - UCPD1SMEN"]
    #[inline(always)]
    pub fn ucpd1smen(&self) -> UCPD1SMEN_R {
        UCPD1SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn lpuart1smen(&mut self) -> LPUART1SMEN_W {
        LPUART1SMEN_W { w: self }
    }
    #[doc = "Bit 1 - I2C4 clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn i2c4smen(&mut self) -> I2C4SMEN_W {
        I2C4SMEN_W { w: self }
    }
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    #[doc = "Bit 6 - LPTIM3SMEN"]
    #[inline(always)]
    pub fn lptim3smen(&mut self) -> LPTIM3SMEN_W {
        LPTIM3SMEN_W { w: self }
    }
    #[doc = "Bit 9 - FDCAN1SMEN"]
    #[inline(always)]
    pub fn fdcan1smen(&mut self) -> FDCAN1SMEN_W {
        FDCAN1SMEN_W { w: self }
    }
    #[doc = "Bit 21 - USBFSSMEN"]
    #[inline(always)]
    pub fn usbfssmen(&mut self) -> USBFSSMEN_W {
        USBFSSMEN_W { w: self }
    }
    #[doc = "Bit 23 - UCPD1SMEN"]
    #[inline(always)]
    pub fn ucpd1smen(&mut self) -> UCPD1SMEN_W {
        UCPD1SMEN_W { w: self }
    }
}
