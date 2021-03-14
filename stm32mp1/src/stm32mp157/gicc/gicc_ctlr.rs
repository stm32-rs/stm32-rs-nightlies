#[doc = "Reader of register GICC_CTLR"]
pub type R = crate::R<u32, super::GICC_CTLR>;
#[doc = "Writer for register GICC_CTLR"]
pub type W = crate::W<u32, super::GICC_CTLR>;
#[doc = "Register GICC_CTLR `reset()`'s with value 0"]
impl crate::ResetValue for super::GICC_CTLR {
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
#[doc = "Reader of field `FIQBYPDISGRP0`"]
pub type FIQBYPDISGRP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIQBYPDISGRP0`"]
pub struct FIQBYPDISGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQBYPDISGRP0_W<'a> {
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
#[doc = "Reader of field `IRQBYPDISGRP0`"]
pub type IRQBYPDISGRP0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQBYPDISGRP0`"]
pub struct IRQBYPDISGRP0_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQBYPDISGRP0_W<'a> {
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
#[doc = "Reader of field `FIQBYPDISGRP1`"]
pub type FIQBYPDISGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FIQBYPDISGRP1`"]
pub struct FIQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> FIQBYPDISGRP1_W<'a> {
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
#[doc = "Reader of field `IRQBYPDISGRP1`"]
pub type IRQBYPDISGRP1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IRQBYPDISGRP1`"]
pub struct IRQBYPDISGRP1_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQBYPDISGRP1_W<'a> {
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
#[doc = "Reader of field `EOIMODES`"]
pub type EOIMODES_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOIMODES`"]
pub struct EOIMODES_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODES_W<'a> {
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
#[doc = "Reader of field `EOIMODENS`"]
pub type EOIMODENS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOIMODENS`"]
pub struct EOIMODENS_W<'a> {
    w: &'a mut W,
}
impl<'a> EOIMODENS_W<'a> {
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
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&self) -> FIQBYPDISGRP0_R {
        FIQBYPDISGRP0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&self) -> IRQBYPDISGRP0_R {
        IRQBYPDISGRP0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&self) -> FIQBYPDISGRP1_R {
        FIQBYPDISGRP1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&self) -> IRQBYPDISGRP1_R {
        IRQBYPDISGRP1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    pub fn eoimodes(&self) -> EOIMODES_R {
        EOIMODES_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    pub fn eoimodens(&self) -> EOIMODENS_R {
        EOIMODENS_R::new(((self.bits >> 10) & 0x01) != 0)
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
    #[doc = "Bit 5 - FIQBYPDISGRP0"]
    #[inline(always)]
    pub fn fiqbypdisgrp0(&mut self) -> FIQBYPDISGRP0_W {
        FIQBYPDISGRP0_W { w: self }
    }
    #[doc = "Bit 6 - IRQBYPDISGRP0"]
    #[inline(always)]
    pub fn irqbypdisgrp0(&mut self) -> IRQBYPDISGRP0_W {
        IRQBYPDISGRP0_W { w: self }
    }
    #[doc = "Bit 7 - FIQBYPDISGRP1"]
    #[inline(always)]
    pub fn fiqbypdisgrp1(&mut self) -> FIQBYPDISGRP1_W {
        FIQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 8 - IRQBYPDISGRP1"]
    #[inline(always)]
    pub fn irqbypdisgrp1(&mut self) -> IRQBYPDISGRP1_W {
        IRQBYPDISGRP1_W { w: self }
    }
    #[doc = "Bit 9 - EOIMODES"]
    #[inline(always)]
    pub fn eoimodes(&mut self) -> EOIMODES_W {
        EOIMODES_W { w: self }
    }
    #[doc = "Bit 10 - EOIMODENS"]
    #[inline(always)]
    pub fn eoimodens(&mut self) -> EOIMODENS_W {
        EOIMODENS_W { w: self }
    }
}
