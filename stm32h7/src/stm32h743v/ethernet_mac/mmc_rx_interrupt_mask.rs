#[doc = "Reader of register MMC_RX_INTERRUPT_MASK"]
pub type R = crate::R<u32, super::MMC_RX_INTERRUPT_MASK>;
#[doc = "Writer for register MMC_RX_INTERRUPT_MASK"]
pub type W = crate::W<u32, super::MMC_RX_INTERRUPT_MASK>;
#[doc = "Register MMC_RX_INTERRUPT_MASK `reset()`'s with value 0"]
impl crate::ResetValue for super::MMC_RX_INTERRUPT_MASK {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RXCRCERPIM`"]
pub type RXCRCERPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXCRCERPIM`"]
pub struct RXCRCERPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXCRCERPIM_W<'a> {
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
#[doc = "Reader of field `RXALGNERPIM`"]
pub type RXALGNERPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXALGNERPIM`"]
pub struct RXALGNERPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXALGNERPIM_W<'a> {
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
#[doc = "Reader of field `RXUCGPIM`"]
pub type RXUCGPIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXUCGPIM`"]
pub struct RXUCGPIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXUCGPIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `RXLPIUSCIM`"]
pub type RXLPIUSCIM_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RXLPIUSCIM`"]
pub struct RXLPIUSCIM_W<'a> {
    w: &'a mut W,
}
impl<'a> RXLPIUSCIM_W<'a> {
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
#[doc = "Reader of field `RXLPITRCIM`"]
pub type RXLPITRCIM_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RXCRCERPIM_R {
        RXCRCERPIM_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RXALGNERPIM_R {
        RXALGNERPIM_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgpim(&self) -> RXUCGPIM_R {
        RXUCGPIM_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RXLPIUSCIM_R {
        RXLPIUSCIM_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - MMC Receive LPI transition counter interrupt Mask"]
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RXLPITRCIM_R {
        RXLPITRCIM_R::new(((self.bits >> 27) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - MMC Receive CRC Error Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxcrcerpim(&mut self) -> RXCRCERPIM_W {
        RXCRCERPIM_W { w: self }
    }
    #[doc = "Bit 6 - MMC Receive Alignment Error Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxalgnerpim(&mut self) -> RXALGNERPIM_W {
        RXALGNERPIM_W { w: self }
    }
    #[doc = "Bit 17 - MMC Receive Unicast Good Packet Counter Interrupt Mask"]
    #[inline(always)]
    pub fn rxucgpim(&mut self) -> RXUCGPIM_W {
        RXUCGPIM_W { w: self }
    }
    #[doc = "Bit 26 - MMC Receive LPI microsecond counter interrupt Mask"]
    #[inline(always)]
    pub fn rxlpiuscim(&mut self) -> RXLPIUSCIM_W {
        RXLPIUSCIM_W { w: self }
    }
}
