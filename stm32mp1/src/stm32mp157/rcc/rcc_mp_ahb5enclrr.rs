#[doc = "Reader of register RCC_MP_AHB5ENCLRR"]
pub type R = crate::R<u32, super::RCC_MP_AHB5ENCLRR>;
#[doc = "Writer for register RCC_MP_AHB5ENCLRR"]
pub type W = crate::W<u32, super::RCC_MP_AHB5ENCLRR>;
#[doc = "Register RCC_MP_AHB5ENCLRR `reset()`'s with value 0x0001_0000"]
impl crate::ResetValue for super::RCC_MP_AHB5ENCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0000
    }
}
#[doc = "Reader of field `GPIOZEN`"]
pub type GPIOZEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIOZEN`"]
pub struct GPIOZEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZEN_W<'a> {
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
#[doc = "Reader of field `CRYP1EN`"]
pub type CRYP1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYP1EN`"]
pub struct CRYP1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1EN_W<'a> {
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
#[doc = "Reader of field `HASH1EN`"]
pub type HASH1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASH1EN`"]
pub struct HASH1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1EN_W<'a> {
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
#[doc = "Reader of field `RNG1EN`"]
pub type RNG1EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG1EN`"]
pub struct RNG1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1EN_W<'a> {
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
#[doc = "Reader of field `BKPSRAMEN`"]
pub type BKPSRAMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPSRAMEN`"]
pub struct BKPSRAMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMEN_W<'a> {
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
#[doc = "Reader of field `AXIMCEN`"]
pub type AXIMCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIMCEN`"]
pub struct AXIMCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMCEN_W<'a> {
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
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&self) -> GPIOZEN_R {
        GPIOZEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&self) -> CRYP1EN_R {
        CRYP1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&self) -> HASH1EN_R {
        HASH1EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&self) -> RNG1EN_R {
        RNG1EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&self) -> BKPSRAMEN_R {
        BKPSRAMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    pub fn aximcen(&self) -> AXIMCEN_R {
        AXIMCEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZEN"]
    #[inline(always)]
    pub fn gpiozen(&mut self) -> GPIOZEN_W {
        GPIOZEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1EN"]
    #[inline(always)]
    pub fn cryp1en(&mut self) -> CRYP1EN_W {
        CRYP1EN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1EN"]
    #[inline(always)]
    pub fn hash1en(&mut self) -> HASH1EN_W {
        HASH1EN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1EN"]
    #[inline(always)]
    pub fn rng1en(&mut self) -> RNG1EN_W {
        RNG1EN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMEN"]
    #[inline(always)]
    pub fn bkpsramen(&mut self) -> BKPSRAMEN_W {
        BKPSRAMEN_W { w: self }
    }
    #[doc = "Bit 16 - AXIMCEN"]
    #[inline(always)]
    pub fn aximcen(&mut self) -> AXIMCEN_W {
        AXIMCEN_W { w: self }
    }
}
