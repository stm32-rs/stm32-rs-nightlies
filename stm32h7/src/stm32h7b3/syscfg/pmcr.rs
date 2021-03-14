#[doc = "Reader of register PMCR"]
pub type R = crate::R<u32, super::PMCR>;
#[doc = "Writer for register PMCR"]
pub type W = crate::W<u32, super::PMCR>;
#[doc = "Register PMCR `reset()`'s with value 0"]
impl crate::ResetValue for super::PMCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2C1FMP`"]
pub type I2C1FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1FMP`"]
pub struct I2C1FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1FMP_W<'a> {
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
#[doc = "Reader of field `I2C2FMP`"]
pub type I2C2FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2FMP`"]
pub struct I2C2FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2FMP_W<'a> {
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
#[doc = "Reader of field `I2C3FMP`"]
pub type I2C3FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3FMP`"]
pub struct I2C3FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3FMP_W<'a> {
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
#[doc = "Reader of field `I2C4FMP`"]
pub type I2C4FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C4FMP`"]
pub struct I2C4FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4FMP_W<'a> {
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
#[doc = "Reader of field `PB6FMP`"]
pub type PB6FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB6FMP`"]
pub struct PB6FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6FMP_W<'a> {
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
#[doc = "Reader of field `PB7FMP`"]
pub type PB7FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB7FMP`"]
pub struct PB7FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7FMP_W<'a> {
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
#[doc = "Reader of field `PB8FMP`"]
pub type PB8FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB8FMP`"]
pub struct PB8FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8FMP_W<'a> {
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
#[doc = "Reader of field `PB9FMP`"]
pub type PB9FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB9FMP`"]
pub struct PB9FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9FMP_W<'a> {
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
#[doc = "Reader of field `BOOSTE`"]
pub type BOOSTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BOOSTE`"]
pub struct BOOSTE_W<'a> {
    w: &'a mut W,
}
impl<'a> BOOSTE_W<'a> {
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
#[doc = "Reader of field `EPIS`"]
pub type EPIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EPIS`"]
pub struct EPIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EPIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | (((value as u32) & 0x07) << 21);
        self.w
    }
}
#[doc = "Reader of field `PA0SO`"]
pub type PA0SO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA0SO`"]
pub struct PA0SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PA0SO_W<'a> {
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
#[doc = "Reader of field `PA1SO`"]
pub type PA1SO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PA1SO`"]
pub struct PA1SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PA1SO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `PC2SO`"]
pub type PC2SO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC2SO`"]
pub struct PC2SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PC2SO_W<'a> {
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
#[doc = "Reader of field `PC3SO`"]
pub type PC3SO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PC3SO`"]
pub struct PC3SO_W<'a> {
    w: &'a mut W,
}
impl<'a> PC3SO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | (((value as u32) & 0x01) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2C1FMP_R {
        I2C1FMP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2C2FMP_R {
        I2C2FMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2C3FMP_R {
        I2C3FMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2C4FMP_R {
        I2C4FMP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&self) -> PB6FMP_R {
        PB6FMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&self) -> PB7FMP_R {
        PB7FMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&self) -> PB8FMP_R {
        PB8FMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&self) -> PB9FMP_R {
        PB9FMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&self) -> BOOSTE_R {
        BOOSTE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&self) -> EPIS_R {
        EPIS_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&self) -> PA0SO_R {
        PA0SO_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&self) -> PA1SO_R {
        PA1SO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&self) -> PC2SO_R {
        PC2SO_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&self) -> PC3SO_R {
        PC3SO_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&mut self) -> I2C1FMP_W {
        I2C1FMP_W { w: self }
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&mut self) -> I2C2FMP_W {
        I2C2FMP_W { w: self }
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&mut self) -> I2C3FMP_W {
        I2C3FMP_W { w: self }
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&mut self) -> I2C4FMP_W {
        I2C4FMP_W { w: self }
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&mut self) -> PB6FMP_W {
        PB6FMP_W { w: self }
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&mut self) -> PB7FMP_W {
        PB7FMP_W { w: self }
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&mut self) -> PB8FMP_W {
        PB8FMP_W { w: self }
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&mut self) -> PB9FMP_W {
        PB9FMP_W { w: self }
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&mut self) -> BOOSTE_W {
        BOOSTE_W { w: self }
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&mut self) -> EPIS_W {
        EPIS_W { w: self }
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&mut self) -> PA0SO_W {
        PA0SO_W { w: self }
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&mut self) -> PA1SO_W {
        PA1SO_W { w: self }
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&mut self) -> PC2SO_W {
        PC2SO_W { w: self }
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&mut self) -> PC3SO_W {
        PC3SO_W { w: self }
    }
}
