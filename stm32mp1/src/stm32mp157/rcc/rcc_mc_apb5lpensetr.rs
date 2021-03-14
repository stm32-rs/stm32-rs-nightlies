#[doc = "Reader of register RCC_MC_APB5LPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_APB5LPENSETR>;
#[doc = "Writer for register RCC_MC_APB5LPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_APB5LPENSETR>;
#[doc = "Register RCC_MC_APB5LPENSETR `reset()`'s with value 0x0011_391d"]
impl crate::ResetValue for super::RCC_MC_APB5LPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_391d
    }
}
#[doc = "Reader of field `SPI6LPEN`"]
pub type SPI6LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI6LPEN`"]
pub struct SPI6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6LPEN_W<'a> {
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
#[doc = "Reader of field `I2C4LPEN`"]
pub type I2C4LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4LPEN`"]
pub struct I2C4LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4LPEN_W<'a> {
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
#[doc = "Reader of field `I2C6LPEN`"]
pub type I2C6LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C6LPEN`"]
pub struct I2C6LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6LPEN_W<'a> {
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
#[doc = "Reader of field `USART1LPEN`"]
pub type USART1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1LPEN`"]
pub struct USART1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1LPEN_W<'a> {
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
#[doc = "Reader of field `RTCAPBLPEN`"]
pub type RTCAPBLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAPBLPEN`"]
pub struct RTCAPBLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBLPEN_W<'a> {
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
#[doc = "Reader of field `TZC1LPEN`"]
pub type TZC1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZC1LPEN`"]
pub struct TZC1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1LPEN_W<'a> {
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
#[doc = "Reader of field `TZC2LPEN`"]
pub type TZC2LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZC2LPEN`"]
pub struct TZC2LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2LPEN_W<'a> {
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
#[doc = "Reader of field `TZPCLPEN`"]
pub type TZPCLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZPCLPEN`"]
pub struct TZPCLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCLPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `BSECLPEN`"]
pub type BSECLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSECLPEN`"]
pub struct BSECLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECLPEN_W<'a> {
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
#[doc = "Reader of field `STGENLPEN`"]
pub type STGENLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STGENLPEN`"]
pub struct STGENLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENLPEN_W<'a> {
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
#[doc = "Reader of field `STGENSTPEN`"]
pub type STGENSTPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STGENSTPEN`"]
pub struct STGENSTPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENSTPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&self) -> I2C6LPEN_R {
        I2C6LPEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&self) -> TZC1LPEN_R {
        TZC1LPEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&self) -> TZC2LPEN_R {
        TZC2LPEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&self) -> TZPCLPEN_R {
        TZPCLPEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&self) -> BSECLPEN_R {
        BSECLPEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&self) -> STGENLPEN_R {
        STGENLPEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&self) -> STGENSTPEN_R {
        STGENSTPEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6LPEN"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W {
        SPI6LPEN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4LPEN"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W {
        I2C4LPEN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6LPEN"]
    #[inline(always)]
    pub fn i2c6lpen(&mut self) -> I2C6LPEN_W {
        I2C6LPEN_W { w: self }
    }
    #[doc = "Bit 4 - USART1LPEN"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W {
        USART1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBLPEN"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W {
        RTCAPBLPEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1LPEN"]
    #[inline(always)]
    pub fn tzc1lpen(&mut self) -> TZC1LPEN_W {
        TZC1LPEN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2LPEN"]
    #[inline(always)]
    pub fn tzc2lpen(&mut self) -> TZC2LPEN_W {
        TZC2LPEN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCLPEN"]
    #[inline(always)]
    pub fn tzpclpen(&mut self) -> TZPCLPEN_W {
        TZPCLPEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECLPEN"]
    #[inline(always)]
    pub fn bseclpen(&mut self) -> BSECLPEN_W {
        BSECLPEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENLPEN"]
    #[inline(always)]
    pub fn stgenlpen(&mut self) -> STGENLPEN_W {
        STGENLPEN_W { w: self }
    }
    #[doc = "Bit 21 - STGENSTPEN"]
    #[inline(always)]
    pub fn stgenstpen(&mut self) -> STGENSTPEN_W {
        STGENSTPEN_W { w: self }
    }
}
