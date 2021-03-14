#[doc = "Reader of register ICACHE_CR"]
pub type R = crate::R<u32, super::ICACHE_CR>;
#[doc = "Writer for register ICACHE_CR"]
pub type W = crate::W<u32, super::ICACHE_CR>;
#[doc = "Register ICACHE_CR `reset()`'s with value 0x04"]
impl crate::ResetValue for super::ICACHE_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x04
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
#[doc = "Reader of field `CACHEINV`"]
pub type CACHEINV_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CACHEINV`"]
pub struct CACHEINV_W<'a> {
    w: &'a mut W,
}
impl<'a> CACHEINV_W<'a> {
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
#[doc = "Reader of field `WAYSEL`"]
pub type WAYSEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WAYSEL`"]
pub struct WAYSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> WAYSEL_W<'a> {
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
#[doc = "Reader of field `HITMEN`"]
pub type HITMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HITMEN`"]
pub struct HITMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HITMEN_W<'a> {
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
#[doc = "Reader of field `MISSMEN`"]
pub type MISSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISSMEN`"]
pub struct MISSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSMEN_W<'a> {
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
#[doc = "Reader of field `HITMRST`"]
pub type HITMRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HITMRST`"]
pub struct HITMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> HITMRST_W<'a> {
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
#[doc = "Reader of field `MISSMRST`"]
pub type MISSMRST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISSMRST`"]
pub struct MISSMRST_W<'a> {
    w: &'a mut W,
}
impl<'a> MISSMRST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    pub fn cacheinv(&self) -> CACHEINV_R {
        CACHEINV_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    pub fn waysel(&self) -> WAYSEL_R {
        WAYSEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    pub fn hitmen(&self) -> HITMEN_R {
        HITMEN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    pub fn missmen(&self) -> MISSMEN_R {
        MISSMEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    pub fn hitmrst(&self) -> HITMRST_R {
        HITMRST_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - MISSMRST"]
    #[inline(always)]
    pub fn missmrst(&self) -> MISSMRST_R {
        MISSMRST_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W { w: self }
    }
    #[doc = "Bit 1 - CACHEINV"]
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W {
        CACHEINV_W { w: self }
    }
    #[doc = "Bit 2 - WAYSEL"]
    #[inline(always)]
    pub fn waysel(&mut self) -> WAYSEL_W {
        WAYSEL_W { w: self }
    }
    #[doc = "Bit 16 - HITMEN"]
    #[inline(always)]
    pub fn hitmen(&mut self) -> HITMEN_W {
        HITMEN_W { w: self }
    }
    #[doc = "Bit 17 - MISSMEN"]
    #[inline(always)]
    pub fn missmen(&mut self) -> MISSMEN_W {
        MISSMEN_W { w: self }
    }
    #[doc = "Bit 18 - HITMRST"]
    #[inline(always)]
    pub fn hitmrst(&mut self) -> HITMRST_W {
        HITMRST_W { w: self }
    }
    #[doc = "Bit 19 - MISSMRST"]
    #[inline(always)]
    pub fn missmrst(&mut self) -> MISSMRST_W {
        MISSMRST_W { w: self }
    }
}
