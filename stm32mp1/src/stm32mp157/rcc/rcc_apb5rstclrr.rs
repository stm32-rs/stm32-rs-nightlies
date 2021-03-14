#[doc = "Reader of register RCC_APB5RSTCLRR"]
pub type R = crate::R<u32, super::RCC_APB5RSTCLRR>;
#[doc = "Writer for register RCC_APB5RSTCLRR"]
pub type W = crate::W<u32, super::RCC_APB5RSTCLRR>;
#[doc = "Register RCC_APB5RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_APB5RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI6RST`"]
pub type SPI6RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI6RST`"]
pub struct SPI6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6RST_W<'a> {
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
#[doc = "Reader of field `I2C4RST`"]
pub type I2C4RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4RST`"]
pub struct I2C4RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4RST_W<'a> {
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
#[doc = "Reader of field `I2C6RST`"]
pub type I2C6RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C6RST`"]
pub struct I2C6RST_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6RST_W<'a> {
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
#[doc = "Reader of field `USART1RST`"]
pub type USART1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1RST`"]
pub struct USART1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1RST_W<'a> {
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
#[doc = "Reader of field `STGENRST`"]
pub type STGENRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STGENRST`"]
pub struct STGENRST_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&self) -> SPI6RST_R {
        SPI6RST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2C4RST_R {
        I2C4RST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&self) -> I2C6RST_R {
        I2C6RST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&self) -> STGENRST_R {
        STGENRST_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6RST"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> SPI6RST_W {
        SPI6RST_W { w: self }
    }
    #[doc = "Bit 2 - I2C4RST"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2C4RST_W {
        I2C4RST_W { w: self }
    }
    #[doc = "Bit 3 - I2C6RST"]
    #[inline(always)]
    pub fn i2c6rst(&mut self) -> I2C6RST_W {
        I2C6RST_W { w: self }
    }
    #[doc = "Bit 4 - USART1RST"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W {
        USART1RST_W { w: self }
    }
    #[doc = "Bit 20 - STGENRST"]
    #[inline(always)]
    pub fn stgenrst(&mut self) -> STGENRST_W {
        STGENRST_W { w: self }
    }
}
