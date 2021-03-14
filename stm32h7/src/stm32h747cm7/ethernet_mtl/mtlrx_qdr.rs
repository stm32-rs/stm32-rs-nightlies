#[doc = "Reader of register MTLRxQDR"]
pub type R = crate::R<u32, super::MTLRXQDR>;
#[doc = "Writer for register MTLRxQDR"]
pub type W = crate::W<u32, super::MTLRXQDR>;
#[doc = "Register MTLRxQDR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTLRXQDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRXQ`"]
pub type PRXQ_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRXQ`"]
pub struct PRXQ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRXQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 16)) | (((value as u32) & 0x3fff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RXQSTS`"]
pub type RXQSTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RXQSTS`"]
pub struct RXQSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXQSTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "Reader of field `RRCSTS`"]
pub type RRCSTS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RRCSTS`"]
pub struct RRCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RRCSTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `RWCSTS`"]
pub type RWCSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RWCSTS`"]
pub struct RWCSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> RWCSTS_W<'a> {
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
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&mut self) -> PRXQ_W {
        PRXQ_W { w: self }
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&mut self) -> RXQSTS_W {
        RXQSTS_W { w: self }
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&mut self) -> RRCSTS_W {
        RRCSTS_W { w: self }
    }
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&mut self) -> RWCSTS_W {
        RWCSTS_W { w: self }
    }
}
