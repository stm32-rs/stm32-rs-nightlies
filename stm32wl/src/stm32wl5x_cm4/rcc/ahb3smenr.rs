#[doc = "Reader of register AHB3SMENR"]
pub type R = crate::R<u32, super::AHB3SMENR>;
#[doc = "Writer for register AHB3SMENR"]
pub type W = crate::W<u32, super::AHB3SMENR>;
#[doc = "Register AHB3SMENR `reset()`'s with value 0x0387_0000"]
impl crate::ResetValue for super::AHB3SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0387_0000
    }
}
#[doc = "Reader of field `FLASHSMEN`"]
pub type FLASHSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHSMEN`"]
pub struct FLASHSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHSMEN_W<'a> {
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
#[doc = "Reader of field `SRAM2SMEN`"]
pub type SRAM2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2SMEN`"]
pub struct SRAM2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2SMEN_W<'a> {
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
#[doc = "Reader of field `SRAM1SMEN`"]
pub type SRAM1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1SMEN`"]
pub struct SRAM1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1SMEN_W<'a> {
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
#[doc = "Reader of field `RNGSMEN`"]
pub type RNGSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGSMEN`"]
pub struct RNGSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `AESSMEN`"]
pub type AESSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESSMEN`"]
pub struct AESSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AESSMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `PKASMEN`"]
pub type PKASMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKASMEN`"]
pub struct PKASMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PKASMEN_W<'a> {
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
    #[doc = "Bit 25 - Flash interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn flashsmen(&self) -> FLASHSMEN_R {
        FLASHSMEN_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn sram2smen(&self) -> SRAM2SMEN_R {
        SRAM2SMEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn sram1smen(&self) -> SRAM1SMEN_R {
        SRAM1SMEN_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn rngsmen(&self) -> RNGSMEN_R {
        RNGSMEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn aessmen(&self) -> AESSMEN_R {
        AESSMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn pkasmen(&self) -> PKASMEN_R {
        PKASMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 25 - Flash interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn flashsmen(&mut self) -> FLASHSMEN_W {
        FLASHSMEN_W { w: self }
    }
    #[doc = "Bit 24 - SRAM2 memory interface clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn sram2smen(&mut self) -> SRAM2SMEN_W {
        SRAM2SMEN_W { w: self }
    }
    #[doc = "Bit 23 - SRAM1 interface clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn sram1smen(&mut self) -> SRAM1SMEN_W {
        SRAM1SMEN_W { w: self }
    }
    #[doc = "Bit 18 - True RNG clocks enable during CPU1 Csleep and CStop modes"]
    #[inline(always)]
    pub fn rngsmen(&mut self) -> RNGSMEN_W {
        RNGSMEN_W { w: self }
    }
    #[doc = "Bit 17 - AES accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn aessmen(&mut self) -> AESSMEN_W {
        AESSMEN_W { w: self }
    }
    #[doc = "Bit 16 - PKA accelerator clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn pkasmen(&mut self) -> PKASMEN_W {
        PKASMEN_W { w: self }
    }
}
