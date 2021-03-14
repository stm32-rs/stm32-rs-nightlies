#[doc = "Reader of register MACIER"]
pub type R = crate::R<u32, super::MACIER>;
#[doc = "Writer for register MACIER"]
pub type W = crate::W<u32, super::MACIER>;
#[doc = "Register MACIER `reset()`'s with value 0"]
impl crate::ResetValue for super::MACIER {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PHYIE`"]
pub type PHYIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PHYIE`"]
pub struct PHYIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHYIE_W<'a> {
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
#[doc = "Reader of field `PMTIE`"]
pub type PMTIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PMTIE`"]
pub struct PMTIE_W<'a> {
    w: &'a mut W,
}
impl<'a> PMTIE_W<'a> {
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
#[doc = "Reader of field `LPIIE`"]
pub type LPIIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPIIE`"]
pub struct LPIIE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIIE_W<'a> {
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
#[doc = "Reader of field `TSIE`"]
pub type TSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TSIE`"]
pub struct TSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TSIE_W<'a> {
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
#[doc = "Reader of field `TXSTSIE`"]
pub type TXSTSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSTSIE`"]
pub struct TXSTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSTSIE_W<'a> {
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
#[doc = "Reader of field `RXSTSIE`"]
pub type RXSTSIE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXSTSIE`"]
pub struct RXSTSIE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXSTSIE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    pub fn phyie(&self) -> PHYIE_R {
        PHYIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    pub fn pmtie(&self) -> PMTIE_R {
        PMTIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    pub fn lpiie(&self) -> LPIIE_R {
        LPIIE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub fn txstsie(&self) -> TXSTSIE_R {
        TXSTSIE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    pub fn rxstsie(&self) -> RXSTSIE_R {
        RXSTSIE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - PHY Interrupt Enable"]
    #[inline(always)]
    pub fn phyie(&mut self) -> PHYIE_W {
        PHYIE_W { w: self }
    }
    #[doc = "Bit 4 - PMT Interrupt Enable"]
    #[inline(always)]
    pub fn pmtie(&mut self) -> PMTIE_W {
        PMTIE_W { w: self }
    }
    #[doc = "Bit 5 - LPI Interrupt Enable"]
    #[inline(always)]
    pub fn lpiie(&mut self) -> LPIIE_W {
        LPIIE_W { w: self }
    }
    #[doc = "Bit 12 - Timestamp Interrupt Enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W {
        TSIE_W { w: self }
    }
    #[doc = "Bit 13 - Transmit Status Interrupt Enable"]
    #[inline(always)]
    pub fn txstsie(&mut self) -> TXSTSIE_W {
        TXSTSIE_W { w: self }
    }
    #[doc = "Bit 14 - Receive Status Interrupt Enable"]
    #[inline(always)]
    pub fn rxstsie(&mut self) -> RXSTSIE_W {
        RXSTSIE_W { w: self }
    }
}
