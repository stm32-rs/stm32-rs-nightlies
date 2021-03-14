#[doc = "Reader of register MMCTIR"]
pub type R = crate::R<u32, super::MMCTIR>;
#[doc = "Writer for register MMCTIR"]
pub type W = crate::W<u32, super::MMCTIR>;
#[doc = "Register MMCTIR `reset()`'s with value 0"]
impl crate::ResetValue for super::MMCTIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TGFSCS`"]
pub type TGFSCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGFSCS`"]
pub struct TGFSCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFSCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `TGFMSCS`"]
pub type TGFMSCS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGFMSCS`"]
pub struct TGFMSCS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFMSCS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TGFS`"]
pub type TGFS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TGFS`"]
pub struct TGFS_W<'a> {
    w: &'a mut W,
}
impl<'a> TGFS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&self) -> TGFSCS_R {
        TGFSCS_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&self) -> TGFMSCS_R {
        TGFMSCS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&self) -> TGFS_R {
        TGFS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Transmitted good frames single collision status"]
    #[inline(always)]
    pub fn tgfscs(&mut self) -> TGFSCS_W {
        TGFSCS_W { w: self }
    }
    #[doc = "Bit 15 - Transmitted good frames more single collision status"]
    #[inline(always)]
    pub fn tgfmscs(&mut self) -> TGFMSCS_W {
        TGFMSCS_W { w: self }
    }
    #[doc = "Bit 21 - Transmitted good frames status"]
    #[inline(always)]
    pub fn tgfs(&mut self) -> TGFS_W {
        TGFS_W { w: self }
    }
}
