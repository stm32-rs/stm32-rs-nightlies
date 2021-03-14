#[doc = "Reader of register MPCBB2_CR"]
pub type R = crate::R<u32, super::MPCBB2_CR>;
#[doc = "Writer for register MPCBB2_CR"]
pub type W = crate::W<u32, super::MPCBB2_CR>;
#[doc = "Register MPCBB2_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::MPCBB2_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LCK`"]
pub type LCK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LCK`"]
pub struct LCK_W<'a> {
    w: &'a mut W,
}
impl<'a> LCK_W<'a> {
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
#[doc = "Reader of field `INVSECSTATE`"]
pub type INVSECSTATE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `INVSECSTATE`"]
pub struct INVSECSTATE_W<'a> {
    w: &'a mut W,
}
impl<'a> INVSECSTATE_W<'a> {
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
#[doc = "Reader of field `SRWILADIS`"]
pub type SRWILADIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRWILADIS`"]
pub struct SRWILADIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SRWILADIS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    pub fn invsecstate(&self) -> INVSECSTATE_R {
        INVSECSTATE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    pub fn srwiladis(&self) -> SRWILADIS_R {
        SRWILADIS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCK"]
    #[inline(always)]
    pub fn lck(&mut self) -> LCK_W {
        LCK_W { w: self }
    }
    #[doc = "Bit 30 - INVSECSTATE"]
    #[inline(always)]
    pub fn invsecstate(&mut self) -> INVSECSTATE_W {
        INVSECSTATE_W { w: self }
    }
    #[doc = "Bit 31 - SRWILADIS"]
    #[inline(always)]
    pub fn srwiladis(&mut self) -> SRWILADIS_W {
        SRWILADIS_W { w: self }
    }
}
