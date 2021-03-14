#[doc = "Reader of register MMC_TX_INTERRUPT_MASK"]
pub type R = crate::R<u32, super::MMC_TX_INTERRUPT_MASK>;
#[doc = "Writer for register MMC_TX_INTERRUPT_MASK"]
pub type W = crate::W<u32, super::MMC_TX_INTERRUPT_MASK>;
#[doc = "Register MMC_TX_INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_TX_INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TXSCOLGPIM`"]
pub type TXSCOLGPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXSCOLGPIM`"]
pub struct TXSCOLGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXSCOLGPIM_W<'a> {
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
#[doc = "Reader of field `TXMCOLGPIM`"]
pub type TXMCOLGPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXMCOLGPIM`"]
pub struct TXMCOLGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXMCOLGPIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `TXGPKTIM`"]
pub type TXGPKTIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXGPKTIM`"]
pub struct TXGPKTIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXGPKTIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `TXLPIUSCIM`"]
pub type TXLPIUSCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TXLPIUSCIM`"]
pub struct TXLPIUSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TXLPIUSCIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `TXLPITRCIM`"]
pub type TXLPITRCIM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TXSCOLGPIM_R {
        TXSCOLGPIM_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TXMCOLGPIM_R {
        TXMCOLGPIM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TXGPKTIM_R {
        TXGPKTIM_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TXLPIUSCIM_R {
        TXLPIUSCIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Transmit LPI transition counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TXLPITRCIM_R {
        TXLPITRCIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - MMC Transmit Single Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txscolgpim(&mut self) -> TXSCOLGPIM_W {
        TXSCOLGPIM_W { w: self }
    }
    #[doc = "Bit 15 - MMC Transmit Multiple Collision Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txmcolgpim(&mut self) -> TXMCOLGPIM_W {
        TXMCOLGPIM_W { w: self }
    }
    #[doc = "Bit 21 - MMC Transmit Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn txgpktim(&mut self) -> TXGPKTIM_W {
        TXGPKTIM_W { w: self }
    }
    #[doc = "Bit 26 - MMC Transmit LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn txlpiuscim(&mut self) -> TXLPIUSCIM_W {
        TXLPIUSCIM_W { w: self }
    }
}
