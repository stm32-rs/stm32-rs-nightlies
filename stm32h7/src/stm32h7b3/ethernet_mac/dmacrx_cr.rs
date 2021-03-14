#[doc = "Reader of register DMACRxCR"]
pub type R = crate::R<u32, super::DMACRXCR>;
#[doc = "Writer for register DMACRxCR"]
pub type W = crate::W<u32, super::DMACRXCR>;
#[doc = "Register DMACRxCR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMACRXCR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SR`"]
pub type SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SR`"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
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
#[doc = "Reader of field `RBSZ`"]
pub type RBSZ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RBSZ`"]
pub struct RBSZ_W<'a> {
    w: &'a mut W,
}
impl<'a> RBSZ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 1)) | (((value as u32) & 0x3fff) << 1);
        self.w
    }
}
#[doc = "Reader of field `RXPBL`"]
pub type RXPBL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXPBL`"]
pub struct RXPBL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPBL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | (((value as u32) & 0x3f) << 16);
        self.w
    }
}
#[doc = "Reader of field `RPF`"]
pub type RPF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RPF`"]
pub struct RPF_W<'a> {
    w: &'a mut W,
}
impl<'a> RPF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start or Stop Receive Command"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bits 1:14 - Receive Buffer size"]
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W {
        RBSZ_W { w: self }
    }
    #[doc = "Bits 16:21 - RXPBL"]
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W {
        RXPBL_W { w: self }
    }
    #[doc = "Bit 31 - DMA Rx Channel Packet Flush"]
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W {
        RPF_W { w: self }
    }
}
