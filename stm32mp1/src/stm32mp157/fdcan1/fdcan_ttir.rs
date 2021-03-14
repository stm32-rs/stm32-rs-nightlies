#[doc = "Reader of register FDCAN_TTIR"]
pub type R = crate::R<u32, super::FDCAN_TTIR>;
#[doc = "Writer for register FDCAN_TTIR"]
pub type W = crate::W<u32, super::FDCAN_TTIR>;
#[doc = "Register FDCAN_TTIR `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_TTIR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SBC`"]
pub type SBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SBC`"]
pub struct SBC_W<'a> {
    w: &'a mut W,
}
impl<'a> SBC_W<'a> {
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
#[doc = "Reader of field `SMC`"]
pub type SMC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMC`"]
pub struct SMC_W<'a> {
    w: &'a mut W,
}
impl<'a> SMC_W<'a> {
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
#[doc = "Reader of field `CSM`"]
pub type CSM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CSM`"]
pub struct CSM_W<'a> {
    w: &'a mut W,
}
impl<'a> CSM_W<'a> {
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
#[doc = "Reader of field `SOG`"]
pub type SOG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SOG`"]
pub struct SOG_W<'a> {
    w: &'a mut W,
}
impl<'a> SOG_W<'a> {
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
#[doc = "Reader of field `RTMI`"]
pub type RTMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RTMI`"]
pub struct RTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> RTMI_W<'a> {
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
#[doc = "Reader of field `TTMI`"]
pub type TTMI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TTMI`"]
pub struct TTMI_W<'a> {
    w: &'a mut W,
}
impl<'a> TTMI_W<'a> {
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
#[doc = "Reader of field `SWE`"]
pub type SWE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWE`"]
pub struct SWE_W<'a> {
    w: &'a mut W,
}
impl<'a> SWE_W<'a> {
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
#[doc = "Reader of field `GTW`"]
pub type GTW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTW`"]
pub struct GTW_W<'a> {
    w: &'a mut W,
}
impl<'a> GTW_W<'a> {
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
#[doc = "Reader of field `GTD`"]
pub type GTD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTD`"]
pub struct GTD_W<'a> {
    w: &'a mut W,
}
impl<'a> GTD_W<'a> {
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
#[doc = "Reader of field `GTE`"]
pub type GTE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GTE`"]
pub struct GTE_W<'a> {
    w: &'a mut W,
}
impl<'a> GTE_W<'a> {
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
#[doc = "Reader of field `TXU`"]
pub type TXU_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXU`"]
pub struct TXU_W<'a> {
    w: &'a mut W,
}
impl<'a> TXU_W<'a> {
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
#[doc = "Reader of field `TXO`"]
pub type TXO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXO`"]
pub struct TXO_W<'a> {
    w: &'a mut W,
}
impl<'a> TXO_W<'a> {
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
#[doc = "Reader of field `SE1`"]
pub type SE1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE1`"]
pub struct SE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1_W<'a> {
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
#[doc = "Reader of field `SE2`"]
pub type SE2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SE2`"]
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
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
#[doc = "Reader of field `ELC`"]
pub type ELC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ELC`"]
pub struct ELC_W<'a> {
    w: &'a mut W,
}
impl<'a> ELC_W<'a> {
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
#[doc = "Reader of field `IWTG`"]
pub type IWTG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IWTG`"]
pub struct IWTG_W<'a> {
    w: &'a mut W,
}
impl<'a> IWTG_W<'a> {
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
#[doc = "Reader of field `WT`"]
pub type WT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WT`"]
pub struct WT_W<'a> {
    w: &'a mut W,
}
impl<'a> WT_W<'a> {
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
#[doc = "Reader of field `AW`"]
pub type AW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AW`"]
pub struct AW_W<'a> {
    w: &'a mut W,
}
impl<'a> AW_W<'a> {
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
#[doc = "Reader of field `CER`"]
pub type CER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CER`"]
pub struct CER_W<'a> {
    w: &'a mut W,
}
impl<'a> CER_W<'a> {
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
    #[doc = "Bit 0 - SBC"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SMC"]
    #[inline(always)]
    pub fn smc(&self) -> SMC_R {
        SMC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CSM"]
    #[inline(always)]
    pub fn csm(&self) -> CSM_R {
        CSM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SOG"]
    #[inline(always)]
    pub fn sog(&self) -> SOG_R {
        SOG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTMI"]
    #[inline(always)]
    pub fn rtmi(&self) -> RTMI_R {
        RTMI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TTMI"]
    #[inline(always)]
    pub fn ttmi(&self) -> TTMI_R {
        TTMI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SWE"]
    #[inline(always)]
    pub fn swe(&self) -> SWE_R {
        SWE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - GTW"]
    #[inline(always)]
    pub fn gtw(&self) -> GTW_R {
        GTW_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - GTD"]
    #[inline(always)]
    pub fn gtd(&self) -> GTD_R {
        GTD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - GTE"]
    #[inline(always)]
    pub fn gte(&self) -> GTE_R {
        GTE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TXU"]
    #[inline(always)]
    pub fn txu(&self) -> TXU_R {
        TXU_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - TXO"]
    #[inline(always)]
    pub fn txo(&self) -> TXO_R {
        TXO_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SE1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - SE2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - ELC"]
    #[inline(always)]
    pub fn elc(&self) -> ELC_R {
        ELC_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - IWTG"]
    #[inline(always)]
    pub fn iwtg(&self) -> IWTG_R {
        IWTG_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - AW"]
    #[inline(always)]
    pub fn aw(&self) -> AW_R {
        AW_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CER"]
    #[inline(always)]
    pub fn cer(&self) -> CER_R {
        CER_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SBC"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W {
        SBC_W { w: self }
    }
    #[doc = "Bit 1 - SMC"]
    #[inline(always)]
    pub fn smc(&mut self) -> SMC_W {
        SMC_W { w: self }
    }
    #[doc = "Bit 2 - CSM"]
    #[inline(always)]
    pub fn csm(&mut self) -> CSM_W {
        CSM_W { w: self }
    }
    #[doc = "Bit 3 - SOG"]
    #[inline(always)]
    pub fn sog(&mut self) -> SOG_W {
        SOG_W { w: self }
    }
    #[doc = "Bit 4 - RTMI"]
    #[inline(always)]
    pub fn rtmi(&mut self) -> RTMI_W {
        RTMI_W { w: self }
    }
    #[doc = "Bit 5 - TTMI"]
    #[inline(always)]
    pub fn ttmi(&mut self) -> TTMI_W {
        TTMI_W { w: self }
    }
    #[doc = "Bit 6 - SWE"]
    #[inline(always)]
    pub fn swe(&mut self) -> SWE_W {
        SWE_W { w: self }
    }
    #[doc = "Bit 7 - GTW"]
    #[inline(always)]
    pub fn gtw(&mut self) -> GTW_W {
        GTW_W { w: self }
    }
    #[doc = "Bit 8 - GTD"]
    #[inline(always)]
    pub fn gtd(&mut self) -> GTD_W {
        GTD_W { w: self }
    }
    #[doc = "Bit 9 - GTE"]
    #[inline(always)]
    pub fn gte(&mut self) -> GTE_W {
        GTE_W { w: self }
    }
    #[doc = "Bit 10 - TXU"]
    #[inline(always)]
    pub fn txu(&mut self) -> TXU_W {
        TXU_W { w: self }
    }
    #[doc = "Bit 11 - TXO"]
    #[inline(always)]
    pub fn txo(&mut self) -> TXO_W {
        TXO_W { w: self }
    }
    #[doc = "Bit 12 - SE1"]
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W {
        SE1_W { w: self }
    }
    #[doc = "Bit 13 - SE2"]
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    #[doc = "Bit 14 - ELC"]
    #[inline(always)]
    pub fn elc(&mut self) -> ELC_W {
        ELC_W { w: self }
    }
    #[doc = "Bit 15 - IWTG"]
    #[inline(always)]
    pub fn iwtg(&mut self) -> IWTG_W {
        IWTG_W { w: self }
    }
    #[doc = "Bit 16 - WT"]
    #[inline(always)]
    pub fn wt(&mut self) -> WT_W {
        WT_W { w: self }
    }
    #[doc = "Bit 17 - AW"]
    #[inline(always)]
    pub fn aw(&mut self) -> AW_W {
        AW_W { w: self }
    }
    #[doc = "Bit 18 - CER"]
    #[inline(always)]
    pub fn cer(&mut self) -> CER_W {
        CER_W { w: self }
    }
}
