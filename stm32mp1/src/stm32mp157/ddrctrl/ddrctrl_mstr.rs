#[doc = "Reader of register DDRCTRL_MSTR"]
pub type R = crate::R<u32, super::DDRCTRL_MSTR>;
#[doc = "Writer for register DDRCTRL_MSTR"]
pub type W = crate::W<u32, super::DDRCTRL_MSTR>;
#[doc = "Register DDRCTRL_MSTR `reset()`'s with value 0x0004_0001"]
impl crate::ResetValue for super::DDRCTRL_MSTR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0004_0001
    }
}
#[doc = "Reader of field `DDR3`"]
pub type DDR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DDR3`"]
pub struct DDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DDR3_W<'a> {
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
#[doc = "Reader of field `LPDDR2`"]
pub type LPDDR2_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDDR2`"]
pub struct LPDDR2_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR2_W<'a> {
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
#[doc = "Reader of field `LPDDR3`"]
pub type LPDDR3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `LPDDR3`"]
pub struct LPDDR3_W<'a> {
    w: &'a mut W,
}
impl<'a> LPDDR3_W<'a> {
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
#[doc = "Reader of field `BURSTCHOP`"]
pub type BURSTCHOP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BURSTCHOP`"]
pub struct BURSTCHOP_W<'a> {
    w: &'a mut W,
}
impl<'a> BURSTCHOP_W<'a> {
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
#[doc = "Reader of field `EN_2T_TIMING_MODE`"]
pub type EN_2T_TIMING_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_2T_TIMING_MODE`"]
pub struct EN_2T_TIMING_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_2T_TIMING_MODE_W<'a> {
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
#[doc = "Reader of field `DATA_BUS_WIDTH`"]
pub type DATA_BUS_WIDTH_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DATA_BUS_WIDTH`"]
pub struct DATA_BUS_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_BUS_WIDTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
        self.w
    }
}
#[doc = "Reader of field `DLL_OFF_MODE`"]
pub type DLL_OFF_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DLL_OFF_MODE`"]
pub struct DLL_OFF_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DLL_OFF_MODE_W<'a> {
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
#[doc = "Reader of field `BURST_RDWR`"]
pub type BURST_RDWR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `BURST_RDWR`"]
pub struct BURST_RDWR_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_RDWR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&self) -> DDR3_R {
        DDR3_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&self) -> LPDDR2_R {
        LPDDR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&self) -> LPDDR3_R {
        LPDDR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&self) -> BURSTCHOP_R {
        BURSTCHOP_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&self) -> EN_2T_TIMING_MODE_R {
        EN_2T_TIMING_MODE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&self) -> DATA_BUS_WIDTH_R {
        DATA_BUS_WIDTH_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&self) -> DLL_OFF_MODE_R {
        DLL_OFF_MODE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&self) -> BURST_RDWR_R {
        BURST_RDWR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DDR3"]
    #[inline(always)]
    pub fn ddr3(&mut self) -> DDR3_W {
        DDR3_W { w: self }
    }
    #[doc = "Bit 2 - LPDDR2"]
    #[inline(always)]
    pub fn lpddr2(&mut self) -> LPDDR2_W {
        LPDDR2_W { w: self }
    }
    #[doc = "Bit 3 - LPDDR3"]
    #[inline(always)]
    pub fn lpddr3(&mut self) -> LPDDR3_W {
        LPDDR3_W { w: self }
    }
    #[doc = "Bit 9 - BURSTCHOP"]
    #[inline(always)]
    pub fn burstchop(&mut self) -> BURSTCHOP_W {
        BURSTCHOP_W { w: self }
    }
    #[doc = "Bit 10 - EN_2T_TIMING_MODE"]
    #[inline(always)]
    pub fn en_2t_timing_mode(&mut self) -> EN_2T_TIMING_MODE_W {
        EN_2T_TIMING_MODE_W { w: self }
    }
    #[doc = "Bits 12:13 - DATA_BUS_WIDTH"]
    #[inline(always)]
    pub fn data_bus_width(&mut self) -> DATA_BUS_WIDTH_W {
        DATA_BUS_WIDTH_W { w: self }
    }
    #[doc = "Bit 15 - DLL_OFF_MODE"]
    #[inline(always)]
    pub fn dll_off_mode(&mut self) -> DLL_OFF_MODE_W {
        DLL_OFF_MODE_W { w: self }
    }
    #[doc = "Bits 16:19 - BURST_RDWR"]
    #[inline(always)]
    pub fn burst_rdwr(&mut self) -> BURST_RDWR_W {
        BURST_RDWR_W { w: self }
    }
}
