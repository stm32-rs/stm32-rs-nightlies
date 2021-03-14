#[doc = "Reader of register SECCFGR"]
pub type R = crate::R<u32, super::SECCFGR>;
#[doc = "Writer for register SECCFGR"]
pub type W = crate::W<u32, super::SECCFGR>;
#[doc = "Register SECCFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::SECCFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APCSEC`"]
pub type APCSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APCSEC`"]
pub struct APCSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> APCSEC_W<'a> {
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
#[doc = "Reader of field `VBSEC`"]
pub type VBSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VBSEC`"]
pub struct VBSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VBSEC_W<'a> {
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
#[doc = "Reader of field `VDMSEC`"]
pub type VDMSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VDMSEC`"]
pub struct VDMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> VDMSEC_W<'a> {
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
#[doc = "Reader of field `LPMSEC`"]
pub type LPMSEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPMSEC`"]
pub struct LPMSEC_W<'a> {
    w: &'a mut W,
}
impl<'a> LPMSEC_W<'a> {
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
#[doc = "Reader of field `WUP5SEC`"]
pub type WUP5SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP5SEC`"]
pub struct WUP5SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP5SEC_W<'a> {
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
#[doc = "Reader of field `WUP4SEC`"]
pub type WUP4SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP4SEC`"]
pub struct WUP4SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP4SEC_W<'a> {
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
#[doc = "Reader of field `WUP3SEC`"]
pub type WUP3SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP3SEC`"]
pub struct WUP3SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP3SEC_W<'a> {
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
#[doc = "Reader of field `WUP2SEC`"]
pub type WUP2SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP2SEC`"]
pub struct WUP2SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP2SEC_W<'a> {
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
#[doc = "Reader of field `WUP1SEC`"]
pub type WUP1SEC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WUP1SEC`"]
pub struct WUP1SEC_W<'a> {
    w: &'a mut W,
}
impl<'a> WUP1SEC_W<'a> {
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
impl R {
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&self) -> APCSEC_R {
        APCSEC_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&self) -> VBSEC_R {
        VBSEC_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&self) -> VDMSEC_R {
        VDMSEC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&self) -> LPMSEC_R {
        LPMSEC_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&self) -> WUP5SEC_R {
        WUP5SEC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&self) -> WUP4SEC_R {
        WUP4SEC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&self) -> WUP3SEC_R {
        WUP3SEC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&self) -> WUP2SEC_R {
        WUP2SEC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&self) -> WUP1SEC_R {
        WUP1SEC_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 11 - APCSEC"]
    #[inline(always)]
    pub fn apcsec(&mut self) -> APCSEC_W {
        APCSEC_W { w: self }
    }
    #[doc = "Bit 10 - VBSEC"]
    #[inline(always)]
    pub fn vbsec(&mut self) -> VBSEC_W {
        VBSEC_W { w: self }
    }
    #[doc = "Bit 9 - VDMSEC"]
    #[inline(always)]
    pub fn vdmsec(&mut self) -> VDMSEC_W {
        VDMSEC_W { w: self }
    }
    #[doc = "Bit 8 - LPMSEC"]
    #[inline(always)]
    pub fn lpmsec(&mut self) -> LPMSEC_W {
        LPMSEC_W { w: self }
    }
    #[doc = "Bit 4 - WKUP5 pin security"]
    #[inline(always)]
    pub fn wup5sec(&mut self) -> WUP5SEC_W {
        WUP5SEC_W { w: self }
    }
    #[doc = "Bit 3 - WKUP4 pin security"]
    #[inline(always)]
    pub fn wup4sec(&mut self) -> WUP4SEC_W {
        WUP4SEC_W { w: self }
    }
    #[doc = "Bit 2 - WKUP3 pin security"]
    #[inline(always)]
    pub fn wup3sec(&mut self) -> WUP3SEC_W {
        WUP3SEC_W { w: self }
    }
    #[doc = "Bit 1 - WKUP2 pin security"]
    #[inline(always)]
    pub fn wup2sec(&mut self) -> WUP2SEC_W {
        WUP2SEC_W { w: self }
    }
    #[doc = "Bit 0 - WKUP1 pin security"]
    #[inline(always)]
    pub fn wup1sec(&mut self) -> WUP1SEC_W {
        WUP1SEC_W { w: self }
    }
}
