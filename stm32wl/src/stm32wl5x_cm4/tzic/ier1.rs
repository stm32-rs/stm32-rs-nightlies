#[doc = "Reader of register IER1"]
pub type R = crate::R<u32, super::IER1>;
#[doc = "Writer for register IER1"]
pub type W = crate::W<u32, super::IER1>;
#[doc = "Register IER1 `reset()`'s with value 0xffff_ffff"]
impl crate::ResetValue for super::IER1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0xffff_ffff
    }
}
#[doc = "Reader of field `TZICIE`"]
pub type TZICIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZICIE`"]
pub struct TZICIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICIE_W<'a> {
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
#[doc = "Reader of field `TZSCIE`"]
pub type TZSCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZSCIE`"]
pub struct TZSCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCIE_W<'a> {
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
#[doc = "Reader of field `AESIE`"]
pub type AESIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESIE`"]
pub struct AESIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AESIE_W<'a> {
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
#[doc = "Reader of field `RNGIE`"]
pub type RNGIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGIE`"]
pub struct RNGIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGIE_W<'a> {
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
#[doc = "Reader of field `SUBGHZSPIIE`"]
pub type SUBGHZSPIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHZSPIIE`"]
pub struct SUBGHZSPIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPIIE_W<'a> {
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
#[doc = "Reader of field `PWRIE`"]
pub type PWRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRIE`"]
pub struct PWRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRIE_W<'a> {
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
#[doc = "Reader of field `FLASHIFIE`"]
pub type FLASHIFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHIFIE`"]
pub struct FLASHIFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIFIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `DMA1IE`"]
pub type DMA1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1IE`"]
pub struct DMA1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1IE_W<'a> {
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
#[doc = "Reader of field `DMA2IE`"]
pub type DMA2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2IE`"]
pub struct DMA2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2IE_W<'a> {
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
#[doc = "Reader of field `DMAMUX1IE`"]
pub type DMAMUX1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1IE`"]
pub struct DMAMUX1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1IE_W<'a> {
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
#[doc = "Reader of field `FLASHIE`"]
pub type FLASHIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHIE`"]
pub struct FLASHIE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIE_W<'a> {
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
#[doc = "Reader of field `SRAM1IE`"]
pub type SRAM1IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1IE`"]
pub struct SRAM1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1IE_W<'a> {
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
#[doc = "Reader of field `SRAM2IE`"]
pub type SRAM2IE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2IE`"]
pub struct SRAM2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2IE_W<'a> {
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
#[doc = "Reader of field `PKAIE`"]
pub type PKAIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKAIE`"]
pub struct PKAIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PKAIE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&self) -> TZICIE_R {
        TZICIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&self) -> TZSCIE_R {
        TZSCIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&self) -> AESIE_R {
        AESIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&self) -> RNGIE_R {
        RNGIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&self) -> SUBGHZSPIIE_R {
        SUBGHZSPIIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&self) -> PWRIE_R {
        PWRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&self) -> FLASHIFIE_R {
        FLASHIFIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&self) -> DMA1IE_R {
        DMA1IE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&self) -> DMA2IE_R {
        DMA2IE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&self) -> DMAMUX1IE_R {
        DMAMUX1IE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&self) -> FLASHIE_R {
        FLASHIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&self) -> SRAM1IE_R {
        SRAM1IE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&self) -> SRAM2IE_R {
        SRAM2IE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&self) -> PKAIE_R {
        PKAIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICIE"]
    #[inline(always)]
    pub fn tzicie(&mut self) -> TZICIE_W {
        TZICIE_W { w: self }
    }
    #[doc = "Bit 1 - TZSCIE"]
    #[inline(always)]
    pub fn tzscie(&mut self) -> TZSCIE_W {
        TZSCIE_W { w: self }
    }
    #[doc = "Bit 2 - AESIE"]
    #[inline(always)]
    pub fn aesie(&mut self) -> AESIE_W {
        AESIE_W { w: self }
    }
    #[doc = "Bit 3 - RNGIE"]
    #[inline(always)]
    pub fn rngie(&mut self) -> RNGIE_W {
        RNGIE_W { w: self }
    }
    #[doc = "Bit 4 - SUBGHZSPIIE"]
    #[inline(always)]
    pub fn subghzspiie(&mut self) -> SUBGHZSPIIE_W {
        SUBGHZSPIIE_W { w: self }
    }
    #[doc = "Bit 5 - PWRIE"]
    #[inline(always)]
    pub fn pwrie(&mut self) -> PWRIE_W {
        PWRIE_W { w: self }
    }
    #[doc = "Bit 6 - FLASHIFIE"]
    #[inline(always)]
    pub fn flashifie(&mut self) -> FLASHIFIE_W {
        FLASHIFIE_W { w: self }
    }
    #[doc = "Bit 7 - DMA1IE"]
    #[inline(always)]
    pub fn dma1ie(&mut self) -> DMA1IE_W {
        DMA1IE_W { w: self }
    }
    #[doc = "Bit 8 - DMA2IE"]
    #[inline(always)]
    pub fn dma2ie(&mut self) -> DMA2IE_W {
        DMA2IE_W { w: self }
    }
    #[doc = "Bit 9 - DMAMUX1IE"]
    #[inline(always)]
    pub fn dmamux1ie(&mut self) -> DMAMUX1IE_W {
        DMAMUX1IE_W { w: self }
    }
    #[doc = "Bit 10 - FLASHIE"]
    #[inline(always)]
    pub fn flashie(&mut self) -> FLASHIE_W {
        FLASHIE_W { w: self }
    }
    #[doc = "Bit 11 - SRAM1IE"]
    #[inline(always)]
    pub fn sram1ie(&mut self) -> SRAM1IE_W {
        SRAM1IE_W { w: self }
    }
    #[doc = "Bit 12 - SRAM2IE"]
    #[inline(always)]
    pub fn sram2ie(&mut self) -> SRAM2IE_W {
        SRAM2IE_W { w: self }
    }
    #[doc = "Bit 13 - PKAIE"]
    #[inline(always)]
    pub fn pkaie(&mut self) -> PKAIE_W {
        PKAIE_W { w: self }
    }
}
