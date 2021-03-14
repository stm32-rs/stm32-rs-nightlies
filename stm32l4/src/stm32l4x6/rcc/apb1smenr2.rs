#[doc = "Reader of register APB1SMENR2"]
pub type R = crate::R<u32, super::APB1SMENR2>;
#[doc = "Writer for register APB1SMENR2"]
pub type W = crate::W<u32, super::APB1SMENR2>;
#[doc = "Register APB1SMENR2 `reset()`'s with value 0x25"]
impl crate::ResetValue for super::APB1SMENR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x25
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
#[doc = "Reader of field `SWPMI1SMEN`"]
pub type SWPMI1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWPMI1SMEN`"]
pub struct SWPMI1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPMI1SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
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
impl R {
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&self) -> LPTIM2SMEN_R {
        LPTIM2SMEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn swpmi1smen(&self) -> SWPMI1SMEN_R {
        SWPMI1SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
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
}
impl W {
    #[doc = "Bit 5 - LPTIM2SMEN"]
    #[inline(always)]
    pub fn lptim2smen(&mut self) -> LPTIM2SMEN_W {
        LPTIM2SMEN_W { w: self }
    }
    #[doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes"]
    #[inline(always)]
    pub fn swpmi1smen(&mut self) -> SWPMI1SMEN_W {
        SWPMI1SMEN_W { w: self }
    }
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
}
