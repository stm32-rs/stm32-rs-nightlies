#[doc = "Reader of register OTG_HS_GI2CCTL"]
pub type R = crate::R<u32, super::OTG_HS_GI2CCTL>;
#[doc = "Writer for register OTG_HS_GI2CCTL"]
pub type W = crate::W<u32, super::OTG_HS_GI2CCTL>;
#[doc = "Register OTG_HS_GI2CCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_HS_GI2CCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BSYDNE`"]
pub type BSYDNE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BSYDNE`"]
pub struct BSYDNE_W<'a> {
    w: &'a mut W,
}
impl<'a> BSYDNE_W<'a> {
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
#[doc = "Reader of field `RW`"]
pub type RW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RW`"]
pub struct RW_W<'a> {
    w: &'a mut W,
}
impl<'a> RW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `I2CDATSE0`"]
pub type I2CDATSE0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CDATSE0`"]
pub struct I2CDATSE0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDATSE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `I2CDEVADR`"]
pub type I2CDEVADR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `I2CDEVADR`"]
pub struct I2CDEVADR_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CDEVADR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | (((value as u32) & 0x03) << 26);
        self.w
    }
}
#[doc = "Reader of field `ACK`"]
pub type ACK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACK`"]
pub struct ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> ACK_W<'a> {
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
#[doc = "Reader of field `I2CEN`"]
pub type I2CEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2CEN`"]
pub struct I2CEN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CEN_W<'a> {
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
#[doc = "Reader of field `ADDR`"]
pub type ADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADDR`"]
pub struct ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `REGADDR`"]
pub type REGADDR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `REGADDR`"]
pub struct REGADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> REGADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RWDATA`"]
pub type RWDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RWDATA`"]
pub struct RWDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RWDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&self) -> BSYDNE_R {
        BSYDNE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&self) -> RW_R {
        RW_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&self) -> I2CDATSE0_R {
        I2CDATSE0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&self) -> I2CDEVADR_R {
        I2CDEVADR_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&self) -> I2CEN_R {
        I2CEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&self) -> REGADDR_R {
        REGADDR_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&self) -> RWDATA_R {
        RWDATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - I2C Busy/Done"]
    #[inline(always)]
    pub fn bsydne(&mut self) -> BSYDNE_W {
        BSYDNE_W { w: self }
    }
    #[doc = "Bit 30 - Read/Write Indicator"]
    #[inline(always)]
    pub fn rw(&mut self) -> RW_W {
        RW_W { w: self }
    }
    #[doc = "Bit 28 - I2C DatSe0 USB mode"]
    #[inline(always)]
    pub fn i2cdatse0(&mut self) -> I2CDATSE0_W {
        I2CDATSE0_W { w: self }
    }
    #[doc = "Bits 26:27 - I2C Device Address"]
    #[inline(always)]
    pub fn i2cdevadr(&mut self) -> I2CDEVADR_W {
        I2CDEVADR_W { w: self }
    }
    #[doc = "Bit 24 - I2C ACK"]
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W {
        ACK_W { w: self }
    }
    #[doc = "Bit 23 - I2C Enable"]
    #[inline(always)]
    pub fn i2cen(&mut self) -> I2CEN_W {
        I2CEN_W { w: self }
    }
    #[doc = "Bits 16:22 - I2C Address"]
    #[inline(always)]
    pub fn addr(&mut self) -> ADDR_W {
        ADDR_W { w: self }
    }
    #[doc = "Bits 8:15 - I2C Register Address"]
    #[inline(always)]
    pub fn regaddr(&mut self) -> REGADDR_W {
        REGADDR_W { w: self }
    }
    #[doc = "Bits 0:7 - I2C Read/Write Data"]
    #[inline(always)]
    pub fn rwdata(&mut self) -> RWDATA_W {
        RWDATA_W { w: self }
    }
}
