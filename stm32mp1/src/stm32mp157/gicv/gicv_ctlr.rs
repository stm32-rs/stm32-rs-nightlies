#[doc = "Reader of register GICV_CTLR"]
pub type R = crate::R<u32, super::GICV_CTLR>;
#[doc = "Writer for register GICV_CTLR"]
pub type W = crate::W<u32, super::GICV_CTLR>;
#[doc = "Register GICV_CTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICV_CTLR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ENABLEGRP0`"]
pub type ENABLEGRP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEGRP0`"]
pub struct ENABLEGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEGRP0_W<'a> {
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
#[doc = "Reader of field `ENABLEGRP1`"]
pub type ENABLEGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ENABLEGRP1`"]
pub struct ENABLEGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLEGRP1_W<'a> {
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
#[doc = "Reader of field `ACKCTL`"]
pub type ACKCTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKCTL`"]
pub struct ACKCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKCTL_W<'a> {
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
#[doc = "Reader of field `FIQEN`"]
pub type FIQEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIQEN`"]
pub struct FIQEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQEN_W<'a> {
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
#[doc = "Reader of field `CBPR`"]
pub type CBPR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CBPR`"]
pub struct CBPR_W<'a> {
    w: &'a mut W,
}
impl<'a> CBPR_W<'a> {
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
#[doc = "Reader of field `EOIMODE`"]
pub type EOIMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOIMODE`"]
pub struct EOIMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&self) -> ENABLEGRP0_R {
        ENABLEGRP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&self) -> ENABLEGRP1_R {
        ENABLEGRP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&self) -> ACKCTL_R {
        ACKCTL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&self) -> FIQEN_R {
        FIQEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&self) -> CBPR_R {
        CBPR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    pub fn eoimode(&self) -> EOIMODE_R {
        EOIMODE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLEGRP0"]
    #[inline(always)]
    pub fn enablegrp0(&mut self) -> ENABLEGRP0_W {
        ENABLEGRP0_W { w: self }
    }
    #[doc = "Bit 1 - ENABLEGRP1"]
    #[inline(always)]
    pub fn enablegrp1(&mut self) -> ENABLEGRP1_W {
        ENABLEGRP1_W { w: self }
    }
    #[doc = "Bit 2 - ACKCTL"]
    #[inline(always)]
    pub fn ackctl(&mut self) -> ACKCTL_W {
        ACKCTL_W { w: self }
    }
    #[doc = "Bit 3 - FIQEN"]
    #[inline(always)]
    pub fn fiqen(&mut self) -> FIQEN_W {
        FIQEN_W { w: self }
    }
    #[doc = "Bit 4 - CBPR"]
    #[inline(always)]
    pub fn cbpr(&mut self) -> CBPR_W {
        CBPR_W { w: self }
    }
    #[doc = "Bit 9 - EOIMODE"]
    #[inline(always)]
    pub fn eoimode(&mut self) -> EOIMODE_W {
        EOIMODE_W { w: self }
    }
}
