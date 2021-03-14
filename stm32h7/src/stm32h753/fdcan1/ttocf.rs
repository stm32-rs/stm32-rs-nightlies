#[doc = "Reader of register TTOCF"]
pub type R = crate::R<u32, super::TTOCF>;
#[doc = "Writer for register TTOCF"]
pub type W = crate::W<u32, super::TTOCF>;
#[doc = "Register TTOCF `reset()`'s with value 0"]
impl crate::ResetValue for super::TTOCF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `OM`"]
pub type OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `OM`"]
pub struct OM_W<'a> {
    w: &'a mut W,
}
impl<'a> OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Reader of field `GEN`"]
pub type GEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN`"]
pub struct GEN_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_W<'a> {
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
#[doc = "Reader of field `TM`"]
pub type TM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TM`"]
pub struct TM_W<'a> {
    w: &'a mut W,
}
impl<'a> TM_W<'a> {
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
#[doc = "Reader of field `LDSDL`"]
pub type LDSDL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LDSDL`"]
pub struct LDSDL_W<'a> {
    w: &'a mut W,
}
impl<'a> LDSDL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | (((value as u32) & 0x07) << 5);
        self.w
    }
}
#[doc = "Reader of field `IRTO`"]
pub type IRTO_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `IRTO`"]
pub struct IRTO_W<'a> {
    w: &'a mut W,
}
impl<'a> IRTO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 8)) | (((value as u32) & 0x7f) << 8);
        self.w
    }
}
#[doc = "Reader of field `EECS`"]
pub type EECS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EECS`"]
pub struct EECS_W<'a> {
    w: &'a mut W,
}
impl<'a> EECS_W<'a> {
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
#[doc = "Reader of field `AWL`"]
pub type AWL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AWL`"]
pub struct AWL_W<'a> {
    w: &'a mut W,
}
impl<'a> AWL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `EGTF`"]
pub type EGTF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EGTF`"]
pub struct EGTF_W<'a> {
    w: &'a mut W,
}
impl<'a> EGTF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `ECC`"]
pub type ECC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ECC`"]
pub struct ECC_W<'a> {
    w: &'a mut W,
}
impl<'a> ECC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | (((value as u32) & 0x01) << 25);
        self.w
    }
}
#[doc = "Reader of field `EVTP`"]
pub type EVTP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EVTP`"]
pub struct EVTP_W<'a> {
    w: &'a mut W,
}
impl<'a> EVTP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&self) -> OM_R {
        OM_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen(&self) -> GEN_R {
        GEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TM_R {
        TM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LDSDL_R {
        LDSDL_R::new(((self.bits >> 5) & 0x07) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&self) -> IRTO_R {
        IRTO_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&self) -> EECS_R {
        EECS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&self) -> AWL_R {
        AWL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&self) -> EGTF_R {
        EGTF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&self) -> ECC_R {
        ECC_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&self) -> EVTP_R {
        EVTP_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&mut self) -> OM_W {
        OM_W { w: self }
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen(&mut self) -> GEN_W {
        GEN_W { w: self }
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&mut self) -> TM_W {
        TM_W { w: self }
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&mut self) -> LDSDL_W {
        LDSDL_W { w: self }
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&mut self) -> IRTO_W {
        IRTO_W { w: self }
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&mut self) -> EECS_W {
        EECS_W { w: self }
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&mut self) -> AWL_W {
        AWL_W { w: self }
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&mut self) -> EGTF_W {
        EGTF_W { w: self }
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&mut self) -> ECC_W {
        ECC_W { w: self }
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&mut self) -> EVTP_W {
        EVTP_W { w: self }
    }
}
