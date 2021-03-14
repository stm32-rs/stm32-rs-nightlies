#[doc = "Reader of register JPEG_CR"]
pub type R = crate::R<u32, super::JPEG_CR>;
#[doc = "Writer for register JPEG_CR"]
pub type W = crate::W<u32, super::JPEG_CR>;
#[doc = "Register JPEG_CR `reset()`'s with value 0"]
impl crate::ResetValue for super::JPEG_CR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `JCEN`"]
pub type JCEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `JCEN`"]
pub struct JCEN_W<'a> {
    w: &'a mut W,
}
impl<'a> JCEN_W<'a> {
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
#[doc = "Reader of field `IFTIE`"]
pub type IFTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFTIE`"]
pub struct IFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFTIE_W<'a> {
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
#[doc = "Reader of field `IFNFIE`"]
pub type IFNFIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IFNFIE`"]
pub struct IFNFIE_W<'a> {
    w: &'a mut W,
}
impl<'a> IFNFIE_W<'a> {
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
#[doc = "Reader of field `OFTIE`"]
pub type OFTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFTIE`"]
pub struct OFTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFTIE_W<'a> {
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
#[doc = "Reader of field `OFNEIE`"]
pub type OFNEIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `OFNEIE`"]
pub struct OFNEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OFNEIE_W<'a> {
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
#[doc = "Reader of field `EOCIE`"]
pub type EOCIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EOCIE`"]
pub struct EOCIE_W<'a> {
    w: &'a mut W,
}
impl<'a> EOCIE_W<'a> {
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
#[doc = "Reader of field `HPDIE`"]
pub type HPDIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HPDIE`"]
pub struct HPDIE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPDIE_W<'a> {
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
#[doc = "Reader of field `IDMAEN`"]
pub type IDMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `IDMAEN`"]
pub struct IDMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IDMAEN_W<'a> {
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
#[doc = "Reader of field `ODMAEN`"]
pub type ODMAEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ODMAEN`"]
pub struct ODMAEN_W<'a> {
    w: &'a mut W,
}
impl<'a> ODMAEN_W<'a> {
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
#[doc = "Reader of field `IFF`"]
pub type IFF_R = crate::R<bool, bool>;
#[doc = "Reader of field `OFF`"]
pub type OFF_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 0 - JPEG Core Enable"]
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn iftie(&self) -> IFTIE_R {
        IFTIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable"]
    #[inline(always)]
    pub fn ifnfie(&self) -> IFNFIE_R {
        IFNFIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn oftie(&self) -> OFTIE_R {
        OFTIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ofneie(&self) -> OFNEIE_R {
        OFNEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable"]
    #[inline(always)]
    pub fn hpdie(&self) -> HPDIE_R {
        HPDIE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Input DMA Enable"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output DMA Enable"]
    #[inline(always)]
    pub fn odmaen(&self) -> ODMAEN_R {
        ODMAEN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Input FIFO Flush"]
    #[inline(always)]
    pub fn iff(&self) -> IFF_R {
        IFF_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Output FIFO Flush"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JPEG Core Enable"]
    #[inline(always)]
    pub fn jcen(&mut self) -> JCEN_W {
        JCEN_W { w: self }
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn iftie(&mut self) -> IFTIE_W {
        IFTIE_W { w: self }
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable"]
    #[inline(always)]
    pub fn ifnfie(&mut self) -> IFNFIE_W {
        IFNFIE_W { w: self }
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn oftie(&mut self) -> OFTIE_W {
        OFTIE_W { w: self }
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ofneie(&mut self) -> OFNEIE_W {
        OFNEIE_W { w: self }
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EOCIE_W {
        EOCIE_W { w: self }
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable"]
    #[inline(always)]
    pub fn hpdie(&mut self) -> HPDIE_W {
        HPDIE_W { w: self }
    }
    #[doc = "Bit 11 - Input DMA Enable"]
    #[inline(always)]
    pub fn idmaen(&mut self) -> IDMAEN_W {
        IDMAEN_W { w: self }
    }
    #[doc = "Bit 12 - Output DMA Enable"]
    #[inline(always)]
    pub fn odmaen(&mut self) -> ODMAEN_W {
        ODMAEN_W { w: self }
    }
}
