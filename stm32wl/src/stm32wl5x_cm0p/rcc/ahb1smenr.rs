#[doc = "Reader of register AHB1SMENR"]
pub type R = crate::R<u32, super::AHB1SMENR>;
#[doc = "Writer for register AHB1SMENR"]
pub type W = crate::W<u32, super::AHB1SMENR>;
#[doc = "Register AHB1SMENR `reset()`'s with value 0x1007"]
impl crate::ResetValue for super::AHB1SMENR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x1007
    }
}
#[doc = "Reader of field `CRCSMEN`"]
pub type CRCSMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCSMEN`"]
pub struct CRCSMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCSMEN_W<'a> {
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
#[doc = "Reader of field `DMAMUX1SMEN`"]
pub type DMAMUX1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1SMEN`"]
pub struct DMAMUX1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1SMEN_W<'a> {
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
#[doc = "Reader of field `DMA2SMEN`"]
pub type DMA2SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2SMEN`"]
pub struct DMA2SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2SMEN_W<'a> {
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
#[doc = "Reader of field `DMA1SMEN`"]
pub type DMA1SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1SMEN`"]
pub struct DMA1SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1SMEN_W<'a> {
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
    #[doc = "Bit 12 - CRC clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&self) -> CRCSMEN_R {
        CRCSMEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&self) -> DMAMUX1SMEN_R {
        DMAMUX1SMEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn dma2smen(&self) -> DMA2SMEN_R {
        DMA2SMEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - DMA1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&self) -> DMA1SMEN_R {
        DMA1SMEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - CRC clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn crcsmen(&mut self) -> CRCSMEN_W {
        CRCSMEN_W { w: self }
    }
    #[doc = "Bit 2 - DMAMUX1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn dmamux1smen(&mut self) -> DMAMUX1SMEN_W {
        DMAMUX1SMEN_W { w: self }
    }
    #[doc = "Bit 1 - DMA2 clock enable during CPU1 CSleep mode"]
    #[inline(always)]
    pub fn dma2smen(&mut self) -> DMA2SMEN_W {
        DMA2SMEN_W { w: self }
    }
    #[doc = "Bit 0 - DMA1 clock enable during CPU1 CSleep mode."]
    #[inline(always)]
    pub fn dma1smen(&mut self) -> DMA1SMEN_W {
        DMA1SMEN_W { w: self }
    }
}
