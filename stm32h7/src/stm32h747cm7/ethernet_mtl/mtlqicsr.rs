#[doc = "Reader of register MTLQICSR"]
pub type R = crate::R<u32, super::MTLQICSR>;
#[doc = "Writer for register MTLQICSR"]
pub type W = crate::W<u32, super::MTLQICSR>;
#[doc = "Register MTLQICSR `reset()`'s with value 0"]
impl crate::ResetValue for super::MTLQICSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXOIE`"]
pub type RXOIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOIE`"]
pub struct RXOIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "Reader of field `RXOVFIS`"]
pub type RXOVFIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXOVFIS`"]
pub struct RXOVFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> RXOVFIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TXUIE`"]
pub type TXUIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUIE`"]
pub struct TXUIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUIE_W<'a> {
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
#[doc = "Reader of field `TXUNFIS`"]
pub type TXUNFIS_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXUNFIS`"]
pub struct TXUNFIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TXUNFIS_W<'a> {
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
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RXOVFIS_R {
        RXOVFIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txuie(&self) -> TXUIE_R {
        TXUIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    pub fn txunfis(&self) -> TXUNFIS_R {
        TXUNFIS_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - Receive Queue Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W {
        RXOIE_W { w: self }
    }
    #[doc = "Bit 16 - Receive Queue Overflow Interrupt Status"]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RXOVFIS_W {
        RXOVFIS_W { w: self }
    }
    #[doc = "Bit 8 - Transmit Queue Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txuie(&mut self) -> TXUIE_W {
        TXUIE_W { w: self }
    }
    #[doc = "Bit 0 - Transmit Queue Underflow Interrupt Status"]
    #[inline(always)]
    pub fn txunfis(&mut self) -> TXUNFIS_W {
        TXUNFIS_W { w: self }
    }
}
