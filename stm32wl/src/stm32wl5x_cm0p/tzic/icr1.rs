#[doc = "Reader of register ICR1"]
pub type R = crate::R<u32, super::ICR1>;
#[doc = "Writer for register ICR1"]
pub type W = crate::W<u32, super::ICR1>;
#[doc = "Register ICR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ICR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TZICCF`"]
pub type TZICCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZICCF`"]
pub struct TZICCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZICCF_W<'a> {
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
#[doc = "Reader of field `TZSCCF`"]
pub type TZSCCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TZSCCF`"]
pub struct TZSCCF_W<'a> {
    w: &'a mut W,
}
impl<'a> TZSCCF_W<'a> {
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
#[doc = "Reader of field `AESCF`"]
pub type AESCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AESCF`"]
pub struct AESCF_W<'a> {
    w: &'a mut W,
}
impl<'a> AESCF_W<'a> {
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
#[doc = "Reader of field `RNGCF`"]
pub type RNGCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RNGCF`"]
pub struct RNGCF_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGCF_W<'a> {
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
#[doc = "Reader of field `SUBGHZSPICF`"]
pub type SUBGHZSPICF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SUBGHZSPICF`"]
pub struct SUBGHZSPICF_W<'a> {
    w: &'a mut W,
}
impl<'a> SUBGHZSPICF_W<'a> {
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
#[doc = "Reader of field `PWRCF`"]
pub type PWRCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PWRCF`"]
pub struct PWRCF_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRCF_W<'a> {
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
#[doc = "Reader of field `FLASHIFCF`"]
pub type FLASHIFCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHIFCF`"]
pub struct FLASHIFCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHIFCF_W<'a> {
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
#[doc = "Reader of field `DMA1CF`"]
pub type DMA1CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA1CF`"]
pub struct DMA1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA1CF_W<'a> {
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
#[doc = "Reader of field `DMA2CF`"]
pub type DMA2CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMA2CF`"]
pub struct DMA2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA2CF_W<'a> {
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
#[doc = "Reader of field `DMAMUX1CF`"]
pub type DMAMUX1CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DMAMUX1CF`"]
pub struct DMAMUX1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAMUX1CF_W<'a> {
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
#[doc = "Reader of field `FLASHCF`"]
pub type FLASHCF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FLASHCF`"]
pub struct FLASHCF_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASHCF_W<'a> {
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
#[doc = "Reader of field `SRAM1CF`"]
pub type SRAM1CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM1CF`"]
pub struct SRAM1CF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM1CF_W<'a> {
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
#[doc = "Reader of field `SRAM2CF`"]
pub type SRAM2CF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SRAM2CF`"]
pub struct SRAM2CF_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM2CF_W<'a> {
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
#[doc = "Reader of field `PKACF`"]
pub type PKACF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PKACF`"]
pub struct PKACF_W<'a> {
    w: &'a mut W,
}
impl<'a> PKACF_W<'a> {
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
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&self) -> TZICCF_R {
        TZICCF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&self) -> TZSCCF_R {
        TZSCCF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&self) -> AESCF_R {
        AESCF_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&self) -> RNGCF_R {
        RNGCF_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&self) -> SUBGHZSPICF_R {
        SUBGHZSPICF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&self) -> PWRCF_R {
        PWRCF_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&self) -> FLASHIFCF_R {
        FLASHIFCF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&self) -> DMA1CF_R {
        DMA1CF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&self) -> DMA2CF_R {
        DMA2CF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&self) -> DMAMUX1CF_R {
        DMAMUX1CF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&self) -> FLASHCF_R {
        FLASHCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&self) -> SRAM1CF_R {
        SRAM1CF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&self) -> SRAM2CF_R {
        SRAM2CF_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&self) -> PKACF_R {
        PKACF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZICCF"]
    #[inline(always)]
    pub fn tziccf(&mut self) -> TZICCF_W {
        TZICCF_W { w: self }
    }
    #[doc = "Bit 1 - TZSCCF"]
    #[inline(always)]
    pub fn tzsccf(&mut self) -> TZSCCF_W {
        TZSCCF_W { w: self }
    }
    #[doc = "Bit 2 - AESCF"]
    #[inline(always)]
    pub fn aescf(&mut self) -> AESCF_W {
        AESCF_W { w: self }
    }
    #[doc = "Bit 3 - RNGCF"]
    #[inline(always)]
    pub fn rngcf(&mut self) -> RNGCF_W {
        RNGCF_W { w: self }
    }
    #[doc = "Bit 4 - SUBGHZSPICF"]
    #[inline(always)]
    pub fn subghzspicf(&mut self) -> SUBGHZSPICF_W {
        SUBGHZSPICF_W { w: self }
    }
    #[doc = "Bit 5 - PWRCF"]
    #[inline(always)]
    pub fn pwrcf(&mut self) -> PWRCF_W {
        PWRCF_W { w: self }
    }
    #[doc = "Bit 6 - FLASHIFCF"]
    #[inline(always)]
    pub fn flashifcf(&mut self) -> FLASHIFCF_W {
        FLASHIFCF_W { w: self }
    }
    #[doc = "Bit 7 - DMA1CF"]
    #[inline(always)]
    pub fn dma1cf(&mut self) -> DMA1CF_W {
        DMA1CF_W { w: self }
    }
    #[doc = "Bit 8 - DMA2CF"]
    #[inline(always)]
    pub fn dma2cf(&mut self) -> DMA2CF_W {
        DMA2CF_W { w: self }
    }
    #[doc = "Bit 9 - DMAMUX1CF"]
    #[inline(always)]
    pub fn dmamux1cf(&mut self) -> DMAMUX1CF_W {
        DMAMUX1CF_W { w: self }
    }
    #[doc = "Bit 10 - FLASHCF"]
    #[inline(always)]
    pub fn flashcf(&mut self) -> FLASHCF_W {
        FLASHCF_W { w: self }
    }
    #[doc = "Bit 11 - SRAM1CF"]
    #[inline(always)]
    pub fn sram1cf(&mut self) -> SRAM1CF_W {
        SRAM1CF_W { w: self }
    }
    #[doc = "Bit 12 - SRAM2CF"]
    #[inline(always)]
    pub fn sram2cf(&mut self) -> SRAM2CF_W {
        SRAM2CF_W { w: self }
    }
    #[doc = "Bit 13 - PKACF"]
    #[inline(always)]
    pub fn pkacf(&mut self) -> PKACF_W {
        PKACF_W { w: self }
    }
}
