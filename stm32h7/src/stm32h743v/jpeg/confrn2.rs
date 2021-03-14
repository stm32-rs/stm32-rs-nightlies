#[doc = "Reader of register CONFRN2"]
pub type R = crate::R<u32, super::CONFRN2>;
#[doc = "Writer for register CONFRN2"]
pub type W = crate::W<u32, super::CONFRN2>;
#[doc = "Register CONFRN2 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONFRN2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `HD`"]
pub type HD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HD`"]
pub struct HD_W<'a> {
    w: &'a mut W,
}
impl<'a> HD_W<'a> {
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
#[doc = "Reader of field `HA`"]
pub type HA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HA`"]
pub struct HA_W<'a> {
    w: &'a mut W,
}
impl<'a> HA_W<'a> {
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
#[doc = "Reader of field `QT`"]
pub type QT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `QT`"]
pub struct QT_W<'a> {
    w: &'a mut W,
}
impl<'a> QT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | (((value as u32) & 0x03) << 2);
        self.w
    }
}
#[doc = "Reader of field `NB`"]
pub type NB_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `NB`"]
pub struct NB_W<'a> {
    w: &'a mut W,
}
impl<'a> NB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `VSF`"]
pub type VSF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `VSF`"]
pub struct VSF_W<'a> {
    w: &'a mut W,
}
impl<'a> VSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `HSF`"]
pub type HSF_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSF`"]
pub struct HSF_W<'a> {
    w: &'a mut W,
}
impl<'a> HSF_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    pub fn hd(&self) -> HD_R {
        HD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    pub fn ha(&self) -> HA_R {
        HA_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    pub fn qt(&self) -> QT_R {
        QT_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    pub fn nb(&self) -> NB_R {
        NB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    pub fn vsf(&self) -> VSF_R {
        VSF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    pub fn hsf(&self) -> HSF_R {
        HSF_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Huffman DC Selects the Huffman table for encoding the DC coefficients."]
    #[inline(always)]
    pub fn hd(&mut self) -> HD_W {
        HD_W { w: self }
    }
    #[doc = "Bit 1 - Huffman AC Selects the Huffman table for encoding the AC coefficients."]
    #[inline(always)]
    pub fn ha(&mut self) -> HA_W {
        HA_W { w: self }
    }
    #[doc = "Bits 2:3 - Quantization Table Selects quantization table associated with a color component."]
    #[inline(always)]
    pub fn qt(&mut self) -> QT_W {
        QT_W { w: self }
    }
    #[doc = "Bits 4:7 - Number of Block Number of data units minus 1 that belong to a particular color in the MCU."]
    #[inline(always)]
    pub fn nb(&mut self) -> NB_W {
        NB_W { w: self }
    }
    #[doc = "Bits 8:11 - Vertical Sampling Factor Vertical sampling factor for component i."]
    #[inline(always)]
    pub fn vsf(&mut self) -> VSF_W {
        VSF_W { w: self }
    }
    #[doc = "Bits 12:15 - Horizontal Sampling Factor Horizontal sampling factor for component i."]
    #[inline(always)]
    pub fn hsf(&mut self) -> HSF_W {
        HSF_W { w: self }
    }
}
