#[doc = "Reader of register OTG_GAHBCFG"]
pub type R = crate::R<u32, super::OTG_GAHBCFG>;
#[doc = "Writer for register OTG_GAHBCFG"]
pub type W = crate::W<u32, super::OTG_GAHBCFG>;
#[doc = "Register OTG_GAHBCFG `reset()`'s with value 0"]
impl crate::ResetValue for super::OTG_GAHBCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GINTMSK`"]
pub type GINTMSK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GINTMSK`"]
pub struct GINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> GINTMSK_W<'a> {
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
#[doc = "Reader of field `HBSTLEN`"]
pub type HBSTLEN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HBSTLEN`"]
pub struct HBSTLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HBSTLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Reader of field `DMAEN`"]
pub type DMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAEN`"]
pub struct DMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAEN_W<'a> {
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
#[doc = "Reader of field `TXFELVL`"]
pub type TXFELVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXFELVL`"]
pub struct TXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFELVL_W<'a> {
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
#[doc = "Reader of field `PTXFELVL`"]
pub type PTXFELVL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PTXFELVL`"]
pub struct PTXFELVL_W<'a> {
    w: &'a mut W,
}
impl<'a> PTXFELVL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&self) -> GINTMSK_R {
        GINTMSK_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&self) -> HBSTLEN_R {
        HBSTLEN_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&self) -> TXFELVL_R {
        TXFELVL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&self) -> PTXFELVL_R {
        PTXFELVL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GINTMSK"]
    #[inline(always)]
    pub fn gintmsk(&mut self) -> GINTMSK_W {
        GINTMSK_W { w: self }
    }
    #[doc = "Bits 1:4 - HBSTLEN"]
    #[inline(always)]
    pub fn hbstlen(&mut self) -> HBSTLEN_W {
        HBSTLEN_W { w: self }
    }
    #[doc = "Bit 5 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DMAEN_W {
        DMAEN_W { w: self }
    }
    #[doc = "Bit 7 - TXFELVL"]
    #[inline(always)]
    pub fn txfelvl(&mut self) -> TXFELVL_W {
        TXFELVL_W { w: self }
    }
    #[doc = "Bit 8 - PTXFELVL"]
    #[inline(always)]
    pub fn ptxfelvl(&mut self) -> PTXFELVL_W {
        PTXFELVL_W { w: self }
    }
}
