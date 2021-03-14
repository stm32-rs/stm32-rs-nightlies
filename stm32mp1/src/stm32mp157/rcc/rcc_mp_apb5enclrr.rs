#[doc = "Reader of register RCC_MP_APB5ENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_APB5ENCLRR>;
#[doc = "Writer for register RCC_MP_APB5ENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_APB5ENCLRR>;
#[doc = "Register RCC_MP_APB5ENCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_MP_APB5ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPI6EN`"]
pub type SPI6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPI6EN`"]
pub struct SPI6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6EN_W<'a> {
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
#[doc = "Reader of field `I2C4EN`"]
pub type I2C4EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4EN`"]
pub struct I2C4EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4EN_W<'a> {
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
#[doc = "Reader of field `I2C6EN`"]
pub type I2C6EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C6EN`"]
pub struct I2C6EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C6EN_W<'a> {
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
#[doc = "Reader of field `USART1EN`"]
pub type USART1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USART1EN`"]
pub struct USART1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1EN_W<'a> {
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
#[doc = "Reader of field `RTCAPBEN`"]
pub type RTCAPBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTCAPBEN`"]
pub struct RTCAPBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCAPBEN_W<'a> {
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
#[doc = "Reader of field `TZC1EN`"]
pub type TZC1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZC1EN`"]
pub struct TZC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC1EN_W<'a> {
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
#[doc = "Reader of field `TZC2EN`"]
pub type TZC2EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZC2EN`"]
pub struct TZC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZC2EN_W<'a> {
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
#[doc = "Reader of field `TZPCEN`"]
pub type TZPCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZPCEN`"]
pub struct TZPCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TZPCEN_W<'a> {
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
#[doc = "Reader of field `IWDG1APBEN`"]
pub type IWDG1APBEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWDG1APBEN`"]
pub struct IWDG1APBEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IWDG1APBEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `BSECEN`"]
pub type BSECEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSECEN`"]
pub struct BSECEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BSECEN_W<'a> {
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
#[doc = "Reader of field `STGENEN`"]
pub type STGENEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `STGENEN`"]
pub struct STGENEN_W<'a> {
    w: &'a mut W,
}
impl<'a> STGENEN_W<'a> {
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
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&self) -> SPI6EN_R {
        SPI6EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2C4EN_R {
        I2C4EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&self) -> I2C6EN_R {
        I2C6EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&self) -> USART1EN_R {
        USART1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&self) -> TZC1EN_R {
        TZC1EN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&self) -> TZC2EN_R {
        TZC2EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&self) -> TZPCEN_R {
        TZPCEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWDG1APBEN"]
    #[inline(always)]
    pub fn iwdg1apben(&self) -> IWDG1APBEN_R {
        IWDG1APBEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&self) -> BSECEN_R {
        BSECEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&self) -> STGENEN_R {
        STGENEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI6EN"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> SPI6EN_W {
        SPI6EN_W { w: self }
    }
    #[doc = "Bit 2 - I2C4EN"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2C4EN_W {
        I2C4EN_W { w: self }
    }
    #[doc = "Bit 3 - I2C6EN"]
    #[inline(always)]
    pub fn i2c6en(&mut self) -> I2C6EN_W {
        I2C6EN_W { w: self }
    }
    #[doc = "Bit 4 - USART1EN"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> USART1EN_W {
        USART1EN_W { w: self }
    }
    #[doc = "Bit 8 - RTCAPBEN"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W {
        RTCAPBEN_W { w: self }
    }
    #[doc = "Bit 11 - TZC1EN"]
    #[inline(always)]
    pub fn tzc1en(&mut self) -> TZC1EN_W {
        TZC1EN_W { w: self }
    }
    #[doc = "Bit 12 - TZC2EN"]
    #[inline(always)]
    pub fn tzc2en(&mut self) -> TZC2EN_W {
        TZC2EN_W { w: self }
    }
    #[doc = "Bit 13 - TZPCEN"]
    #[inline(always)]
    pub fn tzpcen(&mut self) -> TZPCEN_W {
        TZPCEN_W { w: self }
    }
    #[doc = "Bit 15 - IWDG1APBEN"]
    #[inline(always)]
    pub fn iwdg1apben(&mut self) -> IWDG1APBEN_W {
        IWDG1APBEN_W { w: self }
    }
    #[doc = "Bit 16 - BSECEN"]
    #[inline(always)]
    pub fn bsecen(&mut self) -> BSECEN_W {
        BSECEN_W { w: self }
    }
    #[doc = "Bit 20 - STGENEN"]
    #[inline(always)]
    pub fn stgenen(&mut self) -> STGENEN_W {
        STGENEN_W { w: self }
    }
}
