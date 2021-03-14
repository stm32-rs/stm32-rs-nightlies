#[doc = "Reader of register CSLOCKR"]
pub type R = crate::R<u32, super::CSLOCKR>;
#[doc = "Writer for register CSLOCKR"]
pub type W = crate::W<u32, super::CSLOCKR>;
#[doc = "Register CSLOCKR `reset()`'s with value 0"]
impl crate::ResetValue for super::CSLOCKR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `LOCKSVTAIRCR`"]
pub type LOCKSVTAIRCR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKSVTAIRCR`"]
pub struct LOCKSVTAIRCR_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSVTAIRCR_W<'a> {
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
#[doc = "Reader of field `LOCKSMPU`"]
pub type LOCKSMPU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKSMPU`"]
pub struct LOCKSMPU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSMPU_W<'a> {
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
#[doc = "Reader of field `LOCKSAU`"]
pub type LOCKSAU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LOCKSAU`"]
pub struct LOCKSAU_W<'a> {
    w: &'a mut W,
}
impl<'a> LOCKSAU_W<'a> {
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
impl R {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&self) -> LOCKSVTAIRCR_R {
        LOCKSVTAIRCR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&self) -> LOCKSMPU_R {
        LOCKSMPU_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&self) -> LOCKSAU_R {
        LOCKSAU_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LOCKSVTAIRCR"]
    #[inline(always)]
    pub fn locksvtaircr(&mut self) -> LOCKSVTAIRCR_W {
        LOCKSVTAIRCR_W { w: self }
    }
    #[doc = "Bit 1 - LOCKSMPU"]
    #[inline(always)]
    pub fn locksmpu(&mut self) -> LOCKSMPU_W {
        LOCKSMPU_W { w: self }
    }
    #[doc = "Bit 2 - LOCKSAU"]
    #[inline(always)]
    pub fn locksau(&mut self) -> LOCKSAU_W {
        LOCKSAU_W { w: self }
    }
}
