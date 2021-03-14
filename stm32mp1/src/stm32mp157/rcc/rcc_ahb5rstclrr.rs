#[doc = "Reader of register RCC_AHB5RSTCLRR"]
pub type R = crate::R<u32, super::RCC_AHB5RSTCLRR>;
#[doc = "Writer for register RCC_AHB5RSTCLRR"]
pub type W = crate::W<u32, super::RCC_AHB5RSTCLRR>;
#[doc = "Register RCC_AHB5RSTCLRR `reset()`'s with value 0"]
impl crate::ResetValue for super::RCC_AHB5RSTCLRR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIOZRST`"]
pub type GPIOZRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIOZRST`"]
pub struct GPIOZRST_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZRST_W<'a> {
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
#[doc = "Reader of field `CRYP1RST`"]
pub type CRYP1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYP1RST`"]
pub struct CRYP1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1RST_W<'a> {
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
#[doc = "Reader of field `HASH1RST`"]
pub type HASH1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASH1RST`"]
pub struct HASH1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1RST_W<'a> {
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
#[doc = "Reader of field `RNG1RST`"]
pub type RNG1RST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG1RST`"]
pub struct RNG1RST_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1RST_W<'a> {
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
#[doc = "Reader of field `AXIMCRST`"]
pub type AXIMCRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXIMCRST`"]
pub struct AXIMCRST_W<'a> {
    w: &'a mut W,
}
impl<'a> AXIMCRST_W<'a> {
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
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&self) -> GPIOZRST_R {
        GPIOZRST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&self) -> CRYP1RST_R {
        CRYP1RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&self) -> HASH1RST_R {
        HASH1RST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&self) -> RNG1RST_R {
        RNG1RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&self) -> AXIMCRST_R {
        AXIMCRST_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZRST"]
    #[inline(always)]
    pub fn gpiozrst(&mut self) -> GPIOZRST_W {
        GPIOZRST_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1RST"]
    #[inline(always)]
    pub fn cryp1rst(&mut self) -> CRYP1RST_W {
        CRYP1RST_W { w: self }
    }
    #[doc = "Bit 5 - HASH1RST"]
    #[inline(always)]
    pub fn hash1rst(&mut self) -> HASH1RST_W {
        HASH1RST_W { w: self }
    }
    #[doc = "Bit 6 - RNG1RST"]
    #[inline(always)]
    pub fn rng1rst(&mut self) -> RNG1RST_W {
        RNG1RST_W { w: self }
    }
    #[doc = "Bit 16 - AXIMCRST"]
    #[inline(always)]
    pub fn aximcrst(&mut self) -> AXIMCRST_W {
        AXIMCRST_W { w: self }
    }
}
