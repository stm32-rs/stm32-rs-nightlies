#[doc = "Reader of register DSI_WPCR0"]
pub type R = crate::R<u32, super::DSI_WPCR0>;
#[doc = "Writer for register DSI_WPCR0"]
pub type W = crate::W<u32, super::DSI_WPCR0>;
#[doc = "Register DSI_WPCR0 `reset()`'s with value 0"]
impl crate::ResetValue for super::DSI_WPCR0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UIX4`"]
pub type UIX4_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `UIX4`"]
pub struct UIX4_W<'a> {
    w: &'a mut W,
}
impl<'a> UIX4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `SWCL`"]
pub type SWCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWCL`"]
pub struct SWCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWCL_W<'a> {
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
#[doc = "Reader of field `SWDL0`"]
pub type SWDL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWDL0`"]
pub struct SWDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL0_W<'a> {
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
#[doc = "Reader of field `SWDL1`"]
pub type SWDL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWDL1`"]
pub struct SWDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> SWDL1_W<'a> {
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
#[doc = "Reader of field `HSICL`"]
pub type HSICL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSICL`"]
pub struct HSICL_W<'a> {
    w: &'a mut W,
}
impl<'a> HSICL_W<'a> {
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
#[doc = "Reader of field `HSIDL0`"]
pub type HSIDL0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIDL0`"]
pub struct HSIDL0_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL0_W<'a> {
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
#[doc = "Reader of field `HSIDL1`"]
pub type HSIDL1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HSIDL1`"]
pub struct HSIDL1_W<'a> {
    w: &'a mut W,
}
impl<'a> HSIDL1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `FTXSMCL`"]
pub type FTXSMCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTXSMCL`"]
pub struct FTXSMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMCL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `FTXSMDL`"]
pub type FTXSMDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FTXSMDL`"]
pub struct FTXSMDL_W<'a> {
    w: &'a mut W,
}
impl<'a> FTXSMDL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `CDOFFDL`"]
pub type CDOFFDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CDOFFDL`"]
pub struct CDOFFDL_W<'a> {
    w: &'a mut W,
}
impl<'a> CDOFFDL_W<'a> {
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
#[doc = "Reader of field `TDDL`"]
pub type TDDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TDDL`"]
pub struct TDDL_W<'a> {
    w: &'a mut W,
}
impl<'a> TDDL_W<'a> {
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
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> UIX4_R {
        UIX4_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SWCL_R {
        SWCL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> SWDL0_R {
        SWDL0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> SWDL1_R {
        SWDL1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HSICL_R {
        HSICL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> HSIDL0_R {
        HSIDL0_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> HSIDL1_R {
        HSIDL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FTXSMCL_R {
        FTXSMCL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FTXSMDL_R {
        FTXSMDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CDOFFDL_R {
        CDOFFDL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TDDL_R {
        TDDL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&mut self) -> UIX4_W {
        UIX4_W { w: self }
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&mut self) -> SWCL_W {
        SWCL_W { w: self }
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&mut self) -> SWDL0_W {
        SWDL0_W { w: self }
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&mut self) -> SWDL1_W {
        SWDL1_W { w: self }
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&mut self) -> HSICL_W {
        HSICL_W { w: self }
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&mut self) -> HSIDL0_W {
        HSIDL0_W { w: self }
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&mut self) -> HSIDL1_W {
        HSIDL1_W { w: self }
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FTXSMCL_W {
        FTXSMCL_W { w: self }
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FTXSMDL_W {
        FTXSMDL_W { w: self }
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CDOFFDL_W {
        CDOFFDL_W { w: self }
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&mut self) -> TDDL_W {
        TDDL_W { w: self }
    }
}
