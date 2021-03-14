#[doc = "Reader of register MACECR"]
pub type R = crate::R<u32, super::MACECR>;
#[doc = "Writer for register MACECR"]
pub type W = crate::W<u32, super::MACECR>;
#[doc = "Register MACECR `reset()`'s with value 0"]
impl crate::ResetValue for super::MACECR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPSL`"]
pub type GPSL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPSL`"]
pub struct GPSL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff) | ((value as u32) & 0x3fff);
        self.w
    }
}
#[doc = "Reader of field `DCRCC`"]
pub type DCRCC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DCRCC`"]
pub struct DCRCC_W<'a> {
    w: &'a mut W,
}
impl<'a> DCRCC_W<'a> {
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
#[doc = "Reader of field `SPEN`"]
pub type SPEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SPEN`"]
pub struct SPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEN_W<'a> {
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
#[doc = "Reader of field `USP`"]
pub type USP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `USP`"]
pub struct USP_W<'a> {
    w: &'a mut W,
}
impl<'a> USP_W<'a> {
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
#[doc = "Reader of field `EIPGEN`"]
pub type EIPGEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EIPGEN`"]
pub struct EIPGEN_W<'a> {
    w: &'a mut W,
}
impl<'a> EIPGEN_W<'a> {
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
#[doc = "Reader of field `EIPG`"]
pub type EIPG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EIPG`"]
pub struct EIPG_W<'a> {
    w: &'a mut W,
}
impl<'a> EIPG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | (((value as u32) & 0x1f) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    pub fn gpsl(&self) -> GPSL_R {
        GPSL_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub fn dcrcc(&self) -> DCRCC_R {
        DCRCC_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    pub fn spen(&self) -> SPEN_R {
        SPEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub fn eipgen(&self) -> EIPGEN_R {
        EIPGEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    pub fn eipg(&self) -> EIPG_R {
        EIPG_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - Giant Packet Size Limit"]
    #[inline(always)]
    pub fn gpsl(&mut self) -> GPSL_W {
        GPSL_W { w: self }
    }
    #[doc = "Bit 16 - Disable CRC Checking for Received Packets"]
    #[inline(always)]
    pub fn dcrcc(&mut self) -> DCRCC_W {
        DCRCC_W { w: self }
    }
    #[doc = "Bit 17 - Slow Protocol Detection Enable"]
    #[inline(always)]
    pub fn spen(&mut self) -> SPEN_W {
        SPEN_W { w: self }
    }
    #[doc = "Bit 18 - Unicast Slow Protocol Packet Detect"]
    #[inline(always)]
    pub fn usp(&mut self) -> USP_W {
        USP_W { w: self }
    }
    #[doc = "Bit 24 - Extended Inter-Packet Gap Enable"]
    #[inline(always)]
    pub fn eipgen(&mut self) -> EIPGEN_W {
        EIPGEN_W { w: self }
    }
    #[doc = "Bits 25:29 - Extended Inter-Packet Gap"]
    #[inline(always)]
    pub fn eipg(&mut self) -> EIPG_W {
        EIPG_W { w: self }
    }
}
