#[doc = "Reader of register FDCAN_PSR"]
pub type R = crate::R<u32, super::FDCAN_PSR>;
#[doc = "Writer for register FDCAN_PSR"]
pub type W = crate::W<u32, super::FDCAN_PSR>;
#[doc = "Register FDCAN_PSR `reset()`'s with value 0x0707"]
impl crate::ResetValue for super::FDCAN_PSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0707
    }
}
#[doc = "Reader of field `LEC`"]
pub type LEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `ACT`"]
pub type ACT_R = crate::R<u8, u8>;
#[doc = "Reader of field `EP`"]
pub type EP_R = crate::R<bool, bool>;
#[doc = "Reader of field `EW`"]
pub type EW_R = crate::R<bool, bool>;
#[doc = "Reader of field `BO`"]
pub type BO_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLEC`"]
pub type DLEC_R = crate::R<u8, u8>;
#[doc = "Reader of field `RESI`"]
pub type RESI_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RESI`"]
pub struct RESI_W<'a> {
    w: &'a mut W,
}
impl<'a> RESI_W<'a> {
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
#[doc = "Reader of field `RBRS`"]
pub type RBRS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RBRS`"]
pub struct RBRS_W<'a> {
    w: &'a mut W,
}
impl<'a> RBRS_W<'a> {
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
#[doc = "Reader of field `REDL`"]
pub type REDL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REDL`"]
pub struct REDL_W<'a> {
    w: &'a mut W,
}
impl<'a> REDL_W<'a> {
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
#[doc = "Reader of field `PXE`"]
pub type PXE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PXE`"]
pub struct PXE_W<'a> {
    w: &'a mut W,
}
impl<'a> PXE_W<'a> {
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
#[doc = "Reader of field `TDCV`"]
pub type TDCV_R = crate::R<u8, u8>;
impl R {
    #[doc = "Bits 0:2 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:4 - ACT"]
    #[inline(always)]
    pub fn act(&self) -> ACT_R {
        ACT_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - EP"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - EW"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - BO"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - DLEC"]
    #[inline(always)]
    pub fn dlec(&self) -> DLEC_R {
        DLEC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&self) -> RESI_R {
        RESI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&self) -> RBRS_R {
        RBRS_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&self) -> REDL_R {
        REDL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&self) -> PXE_R {
        PXE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 16:22 - TDCV"]
    #[inline(always)]
    pub fn tdcv(&self) -> TDCV_R {
        TDCV_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - RESI"]
    #[inline(always)]
    pub fn resi(&mut self) -> RESI_W {
        RESI_W { w: self }
    }
    #[doc = "Bit 12 - RBRS"]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RBRS_W {
        RBRS_W { w: self }
    }
    #[doc = "Bit 13 - REDL"]
    #[inline(always)]
    pub fn redl(&mut self) -> REDL_W {
        REDL_W { w: self }
    }
    #[doc = "Bit 14 - PXE"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PXE_W {
        PXE_W { w: self }
    }
}
