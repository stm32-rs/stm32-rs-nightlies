#[doc = "Reader of register DDRCTRL_POISONCFG"]
pub type R = crate::R<u32, super::DDRCTRL_POISONCFG>;
#[doc = "Writer for register DDRCTRL_POISONCFG"]
pub type W = crate::W<u32, super::DDRCTRL_POISONCFG>;
#[doc = "Register DDRCTRL_POISONCFG `reset()`'s with value 0x0011_0011"]
impl crate::ResetValue for super::DDRCTRL_POISONCFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0011_0011
    }
}
#[doc = "Reader of field `WR_POISON_SLVERR_EN`"]
pub type WR_POISON_SLVERR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_POISON_SLVERR_EN`"]
pub struct WR_POISON_SLVERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_SLVERR_EN_W<'a> {
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
#[doc = "Reader of field `WR_POISON_INTR_EN`"]
pub type WR_POISON_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_POISON_INTR_EN`"]
pub struct WR_POISON_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_INTR_EN_W<'a> {
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
#[doc = "Reader of field `WR_POISON_INTR_CLR`"]
pub type WR_POISON_INTR_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `WR_POISON_INTR_CLR`"]
pub struct WR_POISON_INTR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> WR_POISON_INTR_CLR_W<'a> {
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
#[doc = "Reader of field `RD_POISON_SLVERR_EN`"]
pub type RD_POISON_SLVERR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_POISON_SLVERR_EN`"]
pub struct RD_POISON_SLVERR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_SLVERR_EN_W<'a> {
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
#[doc = "Reader of field `RD_POISON_INTR_EN`"]
pub type RD_POISON_INTR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_POISON_INTR_EN`"]
pub struct RD_POISON_INTR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_INTR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `RD_POISON_INTR_CLR`"]
pub type RD_POISON_INTR_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RD_POISON_INTR_CLR`"]
pub struct RD_POISON_INTR_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> RD_POISON_INTR_CLR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&self) -> WR_POISON_SLVERR_EN_R {
        WR_POISON_SLVERR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&self) -> WR_POISON_INTR_EN_R {
        WR_POISON_INTR_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&self) -> WR_POISON_INTR_CLR_R {
        WR_POISON_INTR_CLR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&self) -> RD_POISON_SLVERR_EN_R {
        RD_POISON_SLVERR_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&self) -> RD_POISON_INTR_EN_R {
        RD_POISON_INTR_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&self) -> RD_POISON_INTR_CLR_R {
        RD_POISON_INTR_CLR_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - WR_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn wr_poison_slverr_en(&mut self) -> WR_POISON_SLVERR_EN_W {
        WR_POISON_SLVERR_EN_W { w: self }
    }
    #[doc = "Bit 4 - WR_POISON_INTR_EN"]
    #[inline(always)]
    pub fn wr_poison_intr_en(&mut self) -> WR_POISON_INTR_EN_W {
        WR_POISON_INTR_EN_W { w: self }
    }
    #[doc = "Bit 8 - WR_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn wr_poison_intr_clr(&mut self) -> WR_POISON_INTR_CLR_W {
        WR_POISON_INTR_CLR_W { w: self }
    }
    #[doc = "Bit 16 - RD_POISON_SLVERR_EN"]
    #[inline(always)]
    pub fn rd_poison_slverr_en(&mut self) -> RD_POISON_SLVERR_EN_W {
        RD_POISON_SLVERR_EN_W { w: self }
    }
    #[doc = "Bit 20 - RD_POISON_INTR_EN"]
    #[inline(always)]
    pub fn rd_poison_intr_en(&mut self) -> RD_POISON_INTR_EN_W {
        RD_POISON_INTR_EN_W { w: self }
    }
    #[doc = "Bit 24 - RD_POISON_INTR_CLR"]
    #[inline(always)]
    pub fn rd_poison_intr_clr(&mut self) -> RD_POISON_INTR_CLR_W {
        RD_POISON_INTR_CLR_W { w: self }
    }
}
