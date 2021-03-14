#[doc = "Reader of register FTSR2"]
pub type R = crate::R<u32, super::FTSR2>;
#[doc = "Writer for register FTSR2"]
pub type W = crate::W<u32, super::FTSR2>;
#[doc = "Register FTSR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::FTSR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FT35`"]
pub type FT35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT35`"]
pub struct FT35_W<'a> {
    w: &'a mut W,
}
impl<'a> FT35_W<'a> {
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
#[doc = "Reader of field `FT36`"]
pub type FT36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT36`"]
pub struct FT36_W<'a> {
    w: &'a mut W,
}
impl<'a> FT36_W<'a> {
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
#[doc = "Reader of field `FT37`"]
pub type FT37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT37`"]
pub struct FT37_W<'a> {
    w: &'a mut W,
}
impl<'a> FT37_W<'a> {
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
#[doc = "Reader of field `FT38`"]
pub type FT38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FT38`"]
pub struct FT38_W<'a> {
    w: &'a mut W,
}
impl<'a> FT38_W<'a> {
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
impl R {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    pub fn ft35(&self) -> FT35_R {
        FT35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    pub fn ft36(&self) -> FT36_R {
        FT36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    pub fn ft37(&self) -> FT37_R {
        FT37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    pub fn ft38(&self) -> FT38_R {
        FT38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FT35"]
    #[inline(always)]
    pub fn ft35(&mut self) -> FT35_W {
        FT35_W { w: self }
    }
    #[doc = "Bit 4 - FT36"]
    #[inline(always)]
    pub fn ft36(&mut self) -> FT36_W {
        FT36_W { w: self }
    }
    #[doc = "Bit 5 - FT37"]
    #[inline(always)]
    pub fn ft37(&mut self) -> FT37_W {
        FT37_W { w: self }
    }
    #[doc = "Bit 6 - FT38"]
    #[inline(always)]
    pub fn ft38(&mut self) -> FT38_W {
        FT38_W { w: self }
    }
}
