#[doc = "Reader of register GICH_HCR"]
pub type R = crate::R<u32, super::GICH_HCR>;
#[doc = "Writer for register GICH_HCR"]
pub type W = crate::W<u32, super::GICH_HCR>;
#[doc = "Register GICH_HCR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICH_HCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EN`"]
pub type EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN`"]
pub struct EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_W<'a> {
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
#[doc = "Reader of field `UIE`"]
pub type UIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UIE`"]
pub struct UIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UIE_W<'a> {
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
#[doc = "Reader of field `LRENPIE`"]
pub type LRENPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LRENPIE`"]
pub struct LRENPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LRENPIE_W<'a> {
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
#[doc = "Reader of field `NPIE`"]
pub type NPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `NPIE`"]
pub struct NPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> NPIE_W<'a> {
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
#[doc = "Reader of field `VGRP0EIE`"]
pub type VGRP0EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VGRP0EIE`"]
pub struct VGRP0EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP0EIE_W<'a> {
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
#[doc = "Reader of field `VGRP0DIE`"]
pub type VGRP0DIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VGRP0DIE`"]
pub struct VGRP0DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP0DIE_W<'a> {
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
#[doc = "Reader of field `VGRP1EIE`"]
pub type VGRP1EIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VGRP1EIE`"]
pub struct VGRP1EIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP1EIE_W<'a> {
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
#[doc = "Reader of field `VGRP1DIE`"]
pub type VGRP1DIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VGRP1DIE`"]
pub struct VGRP1DIE_W<'a> {
    w: &'a mut W,
}
impl<'a> VGRP1DIE_W<'a> {
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
#[doc = "Reader of field `EOICOUNT`"]
pub type EOICOUNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EOICOUNT`"]
pub struct EOICOUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EOICOUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 27)) | (((value as u32) & 0x1f) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    pub fn lrenpie(&self) -> LRENPIE_R {
        LRENPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    pub fn npie(&self) -> NPIE_R {
        NPIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    pub fn vgrp0eie(&self) -> VGRP0EIE_R {
        VGRP0EIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    pub fn vgrp0die(&self) -> VGRP0DIE_R {
        VGRP0DIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    pub fn vgrp1eie(&self) -> VGRP1EIE_R {
        VGRP1EIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    pub fn vgrp1die(&self) -> VGRP1DIE_R {
        VGRP1DIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    pub fn eoicount(&self) -> EOICOUNT_R {
        EOICOUNT_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - UIE"]
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W {
        UIE_W { w: self }
    }
    #[doc = "Bit 2 - LRENPIE"]
    #[inline(always)]
    pub fn lrenpie(&mut self) -> LRENPIE_W {
        LRENPIE_W { w: self }
    }
    #[doc = "Bit 3 - NPIE"]
    #[inline(always)]
    pub fn npie(&mut self) -> NPIE_W {
        NPIE_W { w: self }
    }
    #[doc = "Bit 4 - VGRP0EIE"]
    #[inline(always)]
    pub fn vgrp0eie(&mut self) -> VGRP0EIE_W {
        VGRP0EIE_W { w: self }
    }
    #[doc = "Bit 5 - VGRP0DIE"]
    #[inline(always)]
    pub fn vgrp0die(&mut self) -> VGRP0DIE_W {
        VGRP0DIE_W { w: self }
    }
    #[doc = "Bit 6 - VGRP1EIE"]
    #[inline(always)]
    pub fn vgrp1eie(&mut self) -> VGRP1EIE_W {
        VGRP1EIE_W { w: self }
    }
    #[doc = "Bit 7 - VGRP1DIE"]
    #[inline(always)]
    pub fn vgrp1die(&mut self) -> VGRP1DIE_W {
        VGRP1DIE_W { w: self }
    }
    #[doc = "Bits 27:31 - EOICOUNT"]
    #[inline(always)]
    pub fn eoicount(&mut self) -> EOICOUNT_W {
        EOICOUNT_W { w: self }
    }
}
