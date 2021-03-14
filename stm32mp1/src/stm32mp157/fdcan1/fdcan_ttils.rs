#[doc = "Reader of register FDCAN_TTILS"]
pub type R = crate::R<u32, super::FDCAN_TTILS>;
#[doc = "Writer for register FDCAN_TTILS"]
pub type W = crate::W<u32, super::FDCAN_TTILS>;
#[doc = "Register FDCAN_TTILS `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTILS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBCL`"]
pub type SBCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBCL`"]
pub struct SBCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SBCL_W<'a> {
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
#[doc = "Reader of field `SMCL`"]
pub type SMCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMCL`"]
pub struct SMCL_W<'a> {
    w: &'a mut W,
}
impl<'a> SMCL_W<'a> {
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
#[doc = "Reader of field `CSML`"]
pub type CSML_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSML`"]
pub struct CSML_W<'a> {
    w: &'a mut W,
}
impl<'a> CSML_W<'a> {
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
#[doc = "Reader of field `SOGL`"]
pub type SOGL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOGL`"]
pub struct SOGL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOGL_W<'a> {
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
#[doc = "Reader of field `RTMIL`"]
pub type RTMIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMIL`"]
pub struct RTMIL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMIL_W<'a> {
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
#[doc = "Reader of field `TTMIL`"]
pub type TTMIL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTMIL`"]
pub struct TTMIL_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMIL_W<'a> {
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
#[doc = "Reader of field `SWEL`"]
pub type SWEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWEL`"]
pub struct SWEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWEL_W<'a> {
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
#[doc = "Reader of field `GTWL`"]
pub type GTWL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTWL`"]
pub struct GTWL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTWL_W<'a> {
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
#[doc = "Reader of field `GTDL`"]
pub type GTDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTDL`"]
pub struct GTDL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTDL_W<'a> {
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
#[doc = "Reader of field `GTEL`"]
pub type GTEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTEL`"]
pub struct GTEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GTEL_W<'a> {
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
#[doc = "Reader of field `TXUL`"]
pub type TXUL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUL`"]
pub struct TXUL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUL_W<'a> {
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
#[doc = "Reader of field `TXOL`"]
pub type TXOL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXOL`"]
pub struct TXOL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXOL_W<'a> {
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
#[doc = "Reader of field `SE1L`"]
pub type SE1L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE1L`"]
pub struct SE1L_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1L_W<'a> {
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
#[doc = "Reader of field `SE2L`"]
pub type SE2L_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE2L`"]
pub struct SE2L_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2L_W<'a> {
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
#[doc = "Reader of field `ELCL`"]
pub type ELCL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELCL`"]
pub struct ELCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ELCL_W<'a> {
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
#[doc = "Reader of field `IWTL`"]
pub type IWTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWTL`"]
pub struct IWTL_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTL_W<'a> {
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
#[doc = "Reader of field `WTL`"]
pub type WTL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WTL`"]
pub struct WTL_W<'a> {
    w: &'a mut W,
}
impl<'a> WTL_W<'a> {
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
#[doc = "Reader of field `AWL`"]
pub type AWL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AWL`"]
pub struct AWL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWL_W<'a> {
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
#[doc = "Reader of field `CERL`"]
pub type CERL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CERL`"]
pub struct CERL_W<'a> {
    w: &'a mut W,
}
impl<'a> CERL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    pub fn sbcl(&self) -> SBCL_R {
        SBCL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    pub fn smcl(&self) -> SMCL_R {
        SMCL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    pub fn csml(&self) -> CSML_R {
        CSML_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    pub fn sogl(&self) -> SOGL_R {
        SOGL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    pub fn rtmil(&self) -> RTMIL_R {
        RTMIL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    pub fn ttmil(&self) -> TTMIL_R {
        TTMIL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    pub fn swel(&self) -> SWEL_R {
        SWEL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    pub fn gtwl(&self) -> GTWL_R {
        GTWL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    pub fn gtdl(&self) -> GTDL_R {
        GTDL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    pub fn gtel(&self) -> GTEL_R {
        GTEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    pub fn txul(&self) -> TXUL_R {
        TXUL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    pub fn txol(&self) -> TXOL_R {
        TXOL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    pub fn se1l(&self) -> SE1L_R {
        SE1L_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    pub fn se2l(&self) -> SE2L_R {
        SE2L_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    pub fn elcl(&self) -> ELCL_R {
        ELCL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    pub fn iwtl(&self) -> IWTL_R {
        IWTL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    pub fn wtl(&self) -> WTL_R {
        WTL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    pub fn cerl(&self) -> CERL_R {
        CERL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBCL"]
    #[inline(always)]
    pub fn sbcl(&mut self) -> SBCL_W {
        SBCL_W { w: self }
    }
    #[doc = "Bit 1 - SMCL"]
    #[inline(always)]
    pub fn smcl(&mut self) -> SMCL_W {
        SMCL_W { w: self }
    }
    #[doc = "Bit 2 - CSML"]
    #[inline(always)]
    pub fn csml(&mut self) -> CSML_W {
        CSML_W { w: self }
    }
    #[doc = "Bit 3 - SOGL"]
    #[inline(always)]
    pub fn sogl(&mut self) -> SOGL_W {
        SOGL_W { w: self }
    }
    #[doc = "Bit 4 - RTMIL"]
    #[inline(always)]
    pub fn rtmil(&mut self) -> RTMIL_W {
        RTMIL_W { w: self }
    }
    #[doc = "Bit 5 - TTMIL"]
    #[inline(always)]
    pub fn ttmil(&mut self) -> TTMIL_W {
        TTMIL_W { w: self }
    }
    #[doc = "Bit 6 - SWEL"]
    #[inline(always)]
    pub fn swel(&mut self) -> SWEL_W {
        SWEL_W { w: self }
    }
    #[doc = "Bit 7 - GTWL"]
    #[inline(always)]
    pub fn gtwl(&mut self) -> GTWL_W {
        GTWL_W { w: self }
    }
    #[doc = "Bit 8 - GTDL"]
    #[inline(always)]
    pub fn gtdl(&mut self) -> GTDL_W {
        GTDL_W { w: self }
    }
    #[doc = "Bit 9 - GTEL"]
    #[inline(always)]
    pub fn gtel(&mut self) -> GTEL_W {
        GTEL_W { w: self }
    }
    #[doc = "Bit 10 - TXUL"]
    #[inline(always)]
    pub fn txul(&mut self) -> TXUL_W {
        TXUL_W { w: self }
    }
    #[doc = "Bit 11 - TXOL"]
    #[inline(always)]
    pub fn txol(&mut self) -> TXOL_W {
        TXOL_W { w: self }
    }
    #[doc = "Bit 12 - SE1L"]
    #[inline(always)]
    pub fn se1l(&mut self) -> SE1L_W {
        SE1L_W { w: self }
    }
    #[doc = "Bit 13 - SE2L"]
    #[inline(always)]
    pub fn se2l(&mut self) -> SE2L_W {
        SE2L_W { w: self }
    }
    #[doc = "Bit 14 - ELCL"]
    #[inline(always)]
    pub fn elcl(&mut self) -> ELCL_W {
        ELCL_W { w: self }
    }
    #[doc = "Bit 15 - IWTL"]
    #[inline(always)]
    pub fn iwtl(&mut self) -> IWTL_W {
        IWTL_W { w: self }
    }
    #[doc = "Bit 16 - WTL"]
    #[inline(always)]
    pub fn wtl(&mut self) -> WTL_W {
        WTL_W { w: self }
    }
    #[doc = "Bit 17 - AWL"]
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W {
        AWL_W { w: self }
    }
    #[doc = "Bit 18 - CERL"]
    #[inline(always)]
    pub fn cerl(&mut self) -> CERL_W {
        CERL_W { w: self }
    }
}
