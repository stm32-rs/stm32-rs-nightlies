#[doc = "Reader of register SPI2S_CR1"]
pub type R = crate::R<u32, super::SPI2S_CR1>;
#[doc = "Writer for register SPI2S_CR1"]
pub type W = crate::W<u32, super::SPI2S_CR1>;
#[doc = "Register SPI2S_CR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI2S_CR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SPE`"]
pub type SPE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPE`"]
pub struct SPE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPE_W<'a> {
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
#[doc = "Reader of field `MASRX`"]
pub type MASRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MASRX`"]
pub struct MASRX_W<'a> {
    w: &'a mut W,
}
impl<'a> MASRX_W<'a> {
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
#[doc = "Reader of field `CSTART`"]
pub type CSTART_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSTART`"]
pub struct CSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CSTART_W<'a> {
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
#[doc = "Write proxy for field `CSUSP`"]
pub struct CSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> CSUSP_W<'a> {
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
#[doc = "Reader of field `HDDIR`"]
pub type HDDIR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDDIR`"]
pub struct HDDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> HDDIR_W<'a> {
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
#[doc = "Reader of field `SSI`"]
pub type SSI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SSI`"]
pub struct SSI_W<'a> {
    w: &'a mut W,
}
impl<'a> SSI_W<'a> {
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
#[doc = "Reader of field `CRC33_17`"]
pub type CRC33_17_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRC33_17`"]
pub struct CRC33_17_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC33_17_W<'a> {
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
#[doc = "Reader of field `RCRCINI`"]
pub type RCRCINI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RCRCINI`"]
pub struct RCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> RCRCINI_W<'a> {
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
#[doc = "Reader of field `TCRCINI`"]
pub type TCRCINI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TCRCINI`"]
pub struct TCRCINI_W<'a> {
    w: &'a mut W,
}
impl<'a> TCRCINI_W<'a> {
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
#[doc = "Reader of field `IOLOCK`"]
pub type IOLOCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IOLOCK`"]
pub struct IOLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> IOLOCK_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&self) -> MASRX_R {
        MASRX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&self) -> CSTART_R {
        CSTART_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&self) -> HDDIR_R {
        HDDIR_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&self) -> CRC33_17_R {
        CRC33_17_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&self) -> RCRCINI_R {
        RCRCINI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&self) -> TCRCINI_R {
        TCRCINI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&self) -> IOLOCK_R {
        IOLOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPE"]
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W {
        SPE_W { w: self }
    }
    #[doc = "Bit 8 - MASRX"]
    #[inline(always)]
    pub fn masrx(&mut self) -> MASRX_W {
        MASRX_W { w: self }
    }
    #[doc = "Bit 9 - CSTART"]
    #[inline(always)]
    pub fn cstart(&mut self) -> CSTART_W {
        CSTART_W { w: self }
    }
    #[doc = "Bit 10 - CSUSP"]
    #[inline(always)]
    pub fn csusp(&mut self) -> CSUSP_W {
        CSUSP_W { w: self }
    }
    #[doc = "Bit 11 - HDDIR"]
    #[inline(always)]
    pub fn hddir(&mut self) -> HDDIR_W {
        HDDIR_W { w: self }
    }
    #[doc = "Bit 12 - SSI"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W {
        SSI_W { w: self }
    }
    #[doc = "Bit 13 - CRC33_17"]
    #[inline(always)]
    pub fn crc33_17(&mut self) -> CRC33_17_W {
        CRC33_17_W { w: self }
    }
    #[doc = "Bit 14 - RCRCINI"]
    #[inline(always)]
    pub fn rcrcini(&mut self) -> RCRCINI_W {
        RCRCINI_W { w: self }
    }
    #[doc = "Bit 15 - TCRCINI"]
    #[inline(always)]
    pub fn tcrcini(&mut self) -> TCRCINI_W {
        TCRCINI_W { w: self }
    }
    #[doc = "Bit 16 - IOLOCK"]
    #[inline(always)]
    pub fn iolock(&mut self) -> IOLOCK_W {
        IOLOCK_W { w: self }
    }
}
