#[doc = "Reader of register PMC"]
pub type R = crate::R<u32, super::PMC>;
#[doc = "Writer for register PMC"]
pub type W = crate::W<u32, super::PMC>;
#[doc = "Register PMC `reset()`'s with value 0"]
impl crate::ResetValue for super::PMC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PB7_FMP`"]
pub type PB7_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB7_FMP`"]
pub struct PB7_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB7_FMP_W<'a> {
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
#[doc = "Reader of field `PB8_FMP`"]
pub type PB8_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB8_FMP`"]
pub struct PB8_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB8_FMP_W<'a> {
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
#[doc = "Reader of field `PB9_FMP`"]
pub type PB9_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB9_FMP`"]
pub struct PB9_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB9_FMP_W<'a> {
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
#[doc = "Reader of field `ADCDC2`"]
pub type ADCDC2_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADCDC2`"]
pub struct ADCDC2_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCDC2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `PB6_FMP`"]
pub type PB6_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PB6_FMP`"]
pub struct PB6_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PB6_FMP_W<'a> {
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
#[doc = "Reader of field `I2C3_FMP`"]
pub type I2C3_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C3_FMP`"]
pub struct I2C3_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3_FMP_W<'a> {
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
#[doc = "Reader of field `I2C2_FMP`"]
pub type I2C2_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C2_FMP`"]
pub struct I2C2_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2_FMP_W<'a> {
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
#[doc = "Reader of field `I2C1_FMP`"]
pub type I2C1_FMP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `I2C1_FMP`"]
pub struct I2C1_FMP_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1_FMP_W<'a> {
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
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&self) -> PB7_FMP_R {
        PB7_FMP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&self) -> PB8_FMP_R {
        PB8_FMP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&self) -> PB9_FMP_R {
        PB9_FMP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 16:18 - ADC3DC2"]
    #[inline(always)]
    pub fn adcdc2(&self) -> ADCDC2_R {
        ADCDC2_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    pub fn pb6_fmp(&self) -> PB6_FMP_R {
        PB6_FMP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&self) -> I2C3_FMP_R {
        I2C3_FMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&self) -> I2C2_FMP_R {
        I2C2_FMP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&self) -> I2C1_FMP_R {
        I2C1_FMP_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - PB7_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb7_fmp(&mut self) -> PB7_FMP_W {
        PB7_FMP_W { w: self }
    }
    #[doc = "Bit 6 - PB8_FMP Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb8_fmp(&mut self) -> PB8_FMP_W {
        PB8_FMP_W { w: self }
    }
    #[doc = "Bit 7 - Fast Mode + Enable"]
    #[inline(always)]
    pub fn pb9_fmp(&mut self) -> PB9_FMP_W {
        PB9_FMP_W { w: self }
    }
    #[doc = "Bits 16:18 - ADC3DC2"]
    #[inline(always)]
    pub fn adcdc2(&mut self) -> ADCDC2_W {
        ADCDC2_W { w: self }
    }
    #[doc = "Bit 4 - PB6_FMP Fast Mode"]
    #[inline(always)]
    pub fn pb6_fmp(&mut self) -> PB6_FMP_W {
        PB6_FMP_W { w: self }
    }
    #[doc = "Bit 2 - I2C3_FMP I2C3 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c3_fmp(&mut self) -> I2C3_FMP_W {
        I2C3_FMP_W { w: self }
    }
    #[doc = "Bit 1 - I2C2_FMP I2C2 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c2_fmp(&mut self) -> I2C2_FMP_W {
        I2C2_FMP_W { w: self }
    }
    #[doc = "Bit 0 - I2C1_FMP I2C1 Fast Mode + Enable"]
    #[inline(always)]
    pub fn i2c1_fmp(&mut self) -> I2C1_FMP_W {
        I2C1_FMP_W { w: self }
    }
}
