#[doc = "Reader of register CR"]
pub type R = crate::R<u32, super::CR>;
#[doc = "Writer for register CR"]
pub type W = crate::W<u32, super::CR>;
#[doc = "Register CR `reset()`'s with value 0"]
impl crate::ResetValue for super::CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXDMA`"]
pub type RXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXDMA`"]
pub struct RXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> RXDMA_W<'a> {
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
#[doc = "Reader of field `TXDMA`"]
pub type TXDMA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXDMA`"]
pub struct TXDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXDMA_W<'a> {
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
#[doc = "Reader of field `RXMODE`"]
pub type RXMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXMODE`"]
pub struct RXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXMODE_W<'a> {
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
#[doc = "Reader of field `TXMODE`"]
pub type TXMODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMODE`"]
pub struct TXMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMODE_W<'a> {
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
#[doc = "Reader of field `LPBK`"]
pub type LPBK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPBK`"]
pub struct LPBK_W<'a> {
    w: &'a mut W,
}
impl<'a> LPBK_W<'a> {
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
#[doc = "Reader of field `SWPME`"]
pub type SWPME_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SWPME`"]
pub struct SWPME_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPME_W<'a> {
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
#[doc = "Reader of field `DEACT`"]
pub type DEACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEACT`"]
pub struct DEACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DEACT_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    pub fn swpme(&self) -> SWPME_R {
        SWPME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W {
        RXDMA_W { w: self }
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W {
        TXDMA_W { w: self }
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W {
        RXMODE_W { w: self }
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W {
        TXMODE_W { w: self }
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LPBK_W {
        LPBK_W { w: self }
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    pub fn swpme(&mut self) -> SWPME_W {
        SWPME_W { w: self }
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&mut self) -> DEACT_W {
        DEACT_W { w: self }
    }
}
