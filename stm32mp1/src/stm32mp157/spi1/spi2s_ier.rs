#[doc = "Reader of register SPI2S_IER"]
pub type R = crate::R<u32, super::SPI2S_IER>;
#[doc = "Writer for register SPI2S_IER"]
pub type W = crate::W<u32, super::SPI2S_IER>;
#[doc = "Register SPI2S_IER `reset()`'s with value 0"]
impl crate::ResetValue for super::SPI2S_IER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXPIE`"]
pub type RXPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXPIE`"]
pub struct RXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXPIE_W<'a> {
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
#[doc = "Reader of field `TXPIE`"]
pub type TXPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXPIE`"]
pub struct TXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXPIE_W<'a> {
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
#[doc = "Reader of field `DXPIE`"]
pub type DXPIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DXPIE`"]
pub struct DXPIE_W<'a> {
    w: &'a mut W,
}
impl<'a> DXPIE_W<'a> {
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
#[doc = "Reader of field `EOTIE`"]
pub type EOTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOTIE`"]
pub struct EOTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOTIE_W<'a> {
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
#[doc = "Reader of field `TXTFIE`"]
pub type TXTFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXTFIE`"]
pub struct TXTFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXTFIE_W<'a> {
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
#[doc = "Reader of field `UDRIE`"]
pub type UDRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `UDRIE`"]
pub struct UDRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> UDRIE_W<'a> {
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
#[doc = "Reader of field `OVRIE`"]
pub type OVRIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OVRIE`"]
pub struct OVRIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVRIE_W<'a> {
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
#[doc = "Reader of field `CRCEIE`"]
pub type CRCEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CRCEIE`"]
pub struct CRCEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRCEIE_W<'a> {
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
#[doc = "Reader of field `TIFREIE`"]
pub type TIFREIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TIFREIE`"]
pub struct TIFREIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIFREIE_W<'a> {
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
#[doc = "Reader of field `MODFIE`"]
pub type MODFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MODFIE`"]
pub struct MODFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODFIE_W<'a> {
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
#[doc = "Reader of field `TSERFIE`"]
pub type TSERFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSERFIE`"]
pub struct TSERFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSERFIE_W<'a> {
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
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&self) -> RXPIE_R {
        RXPIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&self) -> TXPIE_R {
        TXPIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&self) -> DXPIE_R {
        DXPIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&self) -> EOTIE_R {
        EOTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&self) -> TXTFIE_R {
        TXTFIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&self) -> UDRIE_R {
        UDRIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&self) -> TIFREIE_R {
        TIFREIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&self) -> MODFIE_R {
        MODFIE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&self) -> TSERFIE_R {
        TSERFIE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXPIE"]
    #[inline(always)]
    pub fn rxpie(&mut self) -> RXPIE_W {
        RXPIE_W { w: self }
    }
    #[doc = "Bit 1 - TXPIE"]
    #[inline(always)]
    pub fn txpie(&mut self) -> TXPIE_W {
        TXPIE_W { w: self }
    }
    #[doc = "Bit 2 - DXPIE"]
    #[inline(always)]
    pub fn dxpie(&mut self) -> DXPIE_W {
        DXPIE_W { w: self }
    }
    #[doc = "Bit 3 - EOTIE"]
    #[inline(always)]
    pub fn eotie(&mut self) -> EOTIE_W {
        EOTIE_W { w: self }
    }
    #[doc = "Bit 4 - TXTFIE"]
    #[inline(always)]
    pub fn txtfie(&mut self) -> TXTFIE_W {
        TXTFIE_W { w: self }
    }
    #[doc = "Bit 5 - UDRIE"]
    #[inline(always)]
    pub fn udrie(&mut self) -> UDRIE_W {
        UDRIE_W { w: self }
    }
    #[doc = "Bit 6 - OVRIE"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OVRIE_W {
        OVRIE_W { w: self }
    }
    #[doc = "Bit 7 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W {
        CRCEIE_W { w: self }
    }
    #[doc = "Bit 8 - TIFREIE"]
    #[inline(always)]
    pub fn tifreie(&mut self) -> TIFREIE_W {
        TIFREIE_W { w: self }
    }
    #[doc = "Bit 9 - MODFIE"]
    #[inline(always)]
    pub fn modfie(&mut self) -> MODFIE_W {
        MODFIE_W { w: self }
    }
    #[doc = "Bit 10 - TSERFIE"]
    #[inline(always)]
    pub fn tserfie(&mut self) -> TSERFIE_W {
        TSERFIE_W { w: self }
    }
}
