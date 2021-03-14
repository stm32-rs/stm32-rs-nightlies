#[doc = "Reader of register PRIVCFGR2"]
pub type R = crate::R<u32, super::PRIVCFGR2>;
#[doc = "Writer for register PRIVCFGR2"]
pub type W = crate::W<u32, super::PRIVCFGR2>;
#[doc = "Register PRIVCFGR2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRIVCFGR2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRIV32`"]
pub type PRIV32_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV32`"]
pub struct PRIV32_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV32_W<'a> {
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
#[doc = "Reader of field `PRIV33`"]
pub type PRIV33_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV33`"]
pub struct PRIV33_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV33_W<'a> {
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
#[doc = "Reader of field `PRIV34`"]
pub type PRIV34_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV34`"]
pub struct PRIV34_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV34_W<'a> {
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
#[doc = "Reader of field `PRIV35`"]
pub type PRIV35_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV35`"]
pub struct PRIV35_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV35_W<'a> {
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
#[doc = "Reader of field `PRIV36`"]
pub type PRIV36_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV36`"]
pub struct PRIV36_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV36_W<'a> {
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
#[doc = "Reader of field `PRIV37`"]
pub type PRIV37_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV37`"]
pub struct PRIV37_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV37_W<'a> {
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
#[doc = "Reader of field `PRIV38`"]
pub type PRIV38_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV38`"]
pub struct PRIV38_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV38_W<'a> {
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
#[doc = "Reader of field `PRIV39`"]
pub type PRIV39_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV39`"]
pub struct PRIV39_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV39_W<'a> {
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
#[doc = "Reader of field `PRIV40`"]
pub type PRIV40_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV40`"]
pub struct PRIV40_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV40_W<'a> {
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
#[doc = "Reader of field `PRIV41`"]
pub type PRIV41_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV41`"]
pub struct PRIV41_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV41_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `PRIV42`"]
pub type PRIV42_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRIV42`"]
pub struct PRIV42_W<'a> {
    w: &'a mut W,
}
impl<'a> PRIV42_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    pub fn priv32(&self) -> PRIV32_R {
        PRIV32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    pub fn priv33(&self) -> PRIV33_R {
        PRIV33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    pub fn priv34(&self) -> PRIV34_R {
        PRIV34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    pub fn priv35(&self) -> PRIV35_R {
        PRIV35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    pub fn priv36(&self) -> PRIV36_R {
        PRIV36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    pub fn priv37(&self) -> PRIV37_R {
        PRIV37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    pub fn priv38(&self) -> PRIV38_R {
        PRIV38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    pub fn priv39(&self) -> PRIV39_R {
        PRIV39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    pub fn priv40(&self) -> PRIV40_R {
        PRIV40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    pub fn priv41(&self) -> PRIV41_R {
        PRIV41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    pub fn priv42(&self) -> PRIV42_R {
        PRIV42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PRIV32"]
    #[inline(always)]
    pub fn priv32(&mut self) -> PRIV32_W {
        PRIV32_W { w: self }
    }
    #[doc = "Bit 1 - PRIV33"]
    #[inline(always)]
    pub fn priv33(&mut self) -> PRIV33_W {
        PRIV33_W { w: self }
    }
    #[doc = "Bit 2 - PRIV34"]
    #[inline(always)]
    pub fn priv34(&mut self) -> PRIV34_W {
        PRIV34_W { w: self }
    }
    #[doc = "Bit 3 - PRIV35"]
    #[inline(always)]
    pub fn priv35(&mut self) -> PRIV35_W {
        PRIV35_W { w: self }
    }
    #[doc = "Bit 4 - PRIV36"]
    #[inline(always)]
    pub fn priv36(&mut self) -> PRIV36_W {
        PRIV36_W { w: self }
    }
    #[doc = "Bit 5 - PRIV37"]
    #[inline(always)]
    pub fn priv37(&mut self) -> PRIV37_W {
        PRIV37_W { w: self }
    }
    #[doc = "Bit 6 - PRIV38"]
    #[inline(always)]
    pub fn priv38(&mut self) -> PRIV38_W {
        PRIV38_W { w: self }
    }
    #[doc = "Bit 7 - PRIV39"]
    #[inline(always)]
    pub fn priv39(&mut self) -> PRIV39_W {
        PRIV39_W { w: self }
    }
    #[doc = "Bit 8 - PRIV40"]
    #[inline(always)]
    pub fn priv40(&mut self) -> PRIV40_W {
        PRIV40_W { w: self }
    }
    #[doc = "Bit 9 - PRIV41"]
    #[inline(always)]
    pub fn priv41(&mut self) -> PRIV41_W {
        PRIV41_W { w: self }
    }
    #[doc = "Bit 10 - PRIV42"]
    #[inline(always)]
    pub fn priv42(&mut self) -> PRIV42_W {
        PRIV42_W { w: self }
    }
}
