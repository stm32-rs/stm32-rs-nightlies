#[doc = "Reader of register FDCAN_ILS"]
pub type R = crate::R<u32, super::FDCAN_ILS>;
#[doc = "Writer for register FDCAN_ILS"]
pub type W = crate::W<u32, super::FDCAN_ILS>;
#[doc = "Register FDCAN_ILS `reset()`'s with value 0"]
impl crate::ResetValue for super::FDCAN_ILS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RxFIFO0`"]
pub type RXFIFO0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxFIFO0`"]
pub struct RXFIFO0_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO0_W<'a> {
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
#[doc = "Reader of field `RxFIFO1`"]
pub type RXFIFO1_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RxFIFO1`"]
pub struct RXFIFO1_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFIFO1_W<'a> {
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
#[doc = "Reader of field `SMSG`"]
pub type SMSG_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMSG`"]
pub struct SMSG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMSG_W<'a> {
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
#[doc = "Reader of field `TFERR`"]
pub type TFERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TFERR`"]
pub struct TFERR_W<'a> {
    w: &'a mut W,
}
impl<'a> TFERR_W<'a> {
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
#[doc = "Reader of field `MISC`"]
pub type MISC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `MISC`"]
pub struct MISC_W<'a> {
    w: &'a mut W,
}
impl<'a> MISC_W<'a> {
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
#[doc = "Reader of field `BERR`"]
pub type BERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BERR`"]
pub struct BERR_W<'a> {
    w: &'a mut W,
}
impl<'a> BERR_W<'a> {
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
#[doc = "Reader of field `PERR`"]
pub type PERR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PERR`"]
pub struct PERR_W<'a> {
    w: &'a mut W,
}
impl<'a> PERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RxFIFO0"]
    #[inline(always)]
    pub fn rx_fifo0(&self) -> RXFIFO0_R {
        RXFIFO0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RxFIFO1"]
    #[inline(always)]
    pub fn rx_fifo1(&self) -> RXFIFO1_R {
        RXFIFO1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SMSG"]
    #[inline(always)]
    pub fn smsg(&self) -> SMSG_R {
        SMSG_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TFERR"]
    #[inline(always)]
    pub fn tferr(&self) -> TFERR_R {
        TFERR_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - MISC"]
    #[inline(always)]
    pub fn misc(&self) -> MISC_R {
        MISC_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - BERR"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - PERR"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RxFIFO0"]
    #[inline(always)]
    pub fn rx_fifo0(&mut self) -> RXFIFO0_W {
        RXFIFO0_W { w: self }
    }
    #[doc = "Bit 1 - RxFIFO1"]
    #[inline(always)]
    pub fn rx_fifo1(&mut self) -> RXFIFO1_W {
        RXFIFO1_W { w: self }
    }
    #[doc = "Bit 2 - SMSG"]
    #[inline(always)]
    pub fn smsg(&mut self) -> SMSG_W {
        SMSG_W { w: self }
    }
    #[doc = "Bit 3 - TFERR"]
    #[inline(always)]
    pub fn tferr(&mut self) -> TFERR_W {
        TFERR_W { w: self }
    }
    #[doc = "Bit 4 - MISC"]
    #[inline(always)]
    pub fn misc(&mut self) -> MISC_W {
        MISC_W { w: self }
    }
    #[doc = "Bit 5 - BERR"]
    #[inline(always)]
    pub fn berr(&mut self) -> BERR_W {
        BERR_W { w: self }
    }
    #[doc = "Bit 6 - PERR"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W { w: self }
    }
}
