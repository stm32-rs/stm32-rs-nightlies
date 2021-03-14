#[doc = "Reader of register DMADSR"]
pub type R = crate::R<u32, super::DMADSR>;
#[doc = "Writer for register DMADSR"]
pub type W = crate::W<u32, super::DMADSR>;
#[doc = "Register DMADSR `reset()`'s with value 0"]
impl crate::ResetValue for super::DMADSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TPS0`"]
pub type TPS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TPS0`"]
pub struct TPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> TPS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RPS0`"]
pub type RPS0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RPS0`"]
pub struct RPS0_W<'a> {
    w: &'a mut W,
}
impl<'a> RPS0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `AXWHSTS`"]
pub type AXWHSTS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AXWHSTS`"]
pub struct AXWHSTS_W<'a> {
    w: &'a mut W,
}
impl<'a> AXWHSTS_W<'a> {
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
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    pub fn tps0(&self) -> TPS0_R {
        TPS0_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    pub fn rps0(&self) -> RPS0_R {
        RPS0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AXWHSTS_R {
        AXWHSTS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    pub fn tps0(&mut self) -> TPS0_W {
        TPS0_W { w: self }
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    pub fn rps0(&mut self) -> RPS0_W {
        RPS0_W { w: self }
    }
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&mut self) -> AXWHSTS_W {
        AXWHSTS_W { w: self }
    }
}
