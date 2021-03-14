#[doc = "Reader of register RCC_MC_AHB5LPENSETR"]
pub type R = crate::R<u32, super::RCC_MC_AHB5LPENSETR>;
#[doc = "Writer for register RCC_MC_AHB5LPENSETR"]
pub type W = crate::W<u32, super::RCC_MC_AHB5LPENSETR>;
#[doc = "Register RCC_MC_AHB5LPENSETR `reset()`'s with value 0x0171"]
impl crate::ResetValue for super::RCC_MC_AHB5LPENSETR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0171
    }
}
#[doc = "Reader of field `GPIOZLPEN`"]
pub type GPIOZLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIOZLPEN`"]
pub struct GPIOZLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIOZLPEN_W<'a> {
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
#[doc = "Reader of field `CRYP1LPEN`"]
pub type CRYP1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRYP1LPEN`"]
pub struct CRYP1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYP1LPEN_W<'a> {
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
#[doc = "Reader of field `HASH1LPEN`"]
pub type HASH1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HASH1LPEN`"]
pub struct HASH1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HASH1LPEN_W<'a> {
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
#[doc = "Reader of field `RNG1LPEN`"]
pub type RNG1LPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNG1LPEN`"]
pub struct RNG1LPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNG1LPEN_W<'a> {
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
#[doc = "Reader of field `BKPSRAMLPEN`"]
pub type BKPSRAMLPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BKPSRAMLPEN`"]
pub struct BKPSRAMLPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BKPSRAMLPEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&self) -> GPIOZLPEN_R {
        GPIOZLPEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&self) -> CRYP1LPEN_R {
        CRYP1LPEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&self) -> HASH1LPEN_R {
        HASH1LPEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&self) -> RNG1LPEN_R {
        RNG1LPEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&self) -> BKPSRAMLPEN_R {
        BKPSRAMLPEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOZLPEN"]
    #[inline(always)]
    pub fn gpiozlpen(&mut self) -> GPIOZLPEN_W {
        GPIOZLPEN_W { w: self }
    }
    #[doc = "Bit 4 - CRYP1LPEN"]
    #[inline(always)]
    pub fn cryp1lpen(&mut self) -> CRYP1LPEN_W {
        CRYP1LPEN_W { w: self }
    }
    #[doc = "Bit 5 - HASH1LPEN"]
    #[inline(always)]
    pub fn hash1lpen(&mut self) -> HASH1LPEN_W {
        HASH1LPEN_W { w: self }
    }
    #[doc = "Bit 6 - RNG1LPEN"]
    #[inline(always)]
    pub fn rng1lpen(&mut self) -> RNG1LPEN_W {
        RNG1LPEN_W { w: self }
    }
    #[doc = "Bit 8 - BKPSRAMLPEN"]
    #[inline(always)]
    pub fn bkpsramlpen(&mut self) -> BKPSRAMLPEN_W {
        BKPSRAMLPEN_W { w: self }
    }
}
