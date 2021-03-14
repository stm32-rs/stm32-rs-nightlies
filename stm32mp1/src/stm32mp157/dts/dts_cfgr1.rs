#[doc = "Reader of register DTS_CFGR1"]
pub type R = crate::R<u32, super::DTS_CFGR1>;
#[doc = "Writer for register DTS_CFGR1"]
pub type W = crate::W<u32, super::DTS_CFGR1>;
#[doc = "Register DTS_CFGR1 `reset()`'s with value 0"]
impl crate::ResetValue for super::DTS_CFGR1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TS1_EN`"]
pub type TS1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_EN`"]
pub struct TS1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_EN_W<'a> {
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
#[doc = "Reader of field `TS1_START`"]
pub type TS1_START_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TS1_START`"]
pub struct TS1_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_START_W<'a> {
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
#[doc = "Reader of field `TS1_INTRIG_SEL`"]
pub type TS1_INTRIG_SEL_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TS1_INTRIG_SEL`"]
pub struct TS1_INTRIG_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_INTRIG_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TS1_SMP_TIME`"]
pub type TS1_SMP_TIME_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TS1_SMP_TIME`"]
pub struct TS1_SMP_TIME_W<'a> {
    w: &'a mut W,
}
impl<'a> TS1_SMP_TIME_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Reader of field `REFCLK_SEL`"]
pub type REFCLK_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REFCLK_SEL`"]
pub struct REFCLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> REFCLK_SEL_W<'a> {
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
#[doc = "Reader of field `Q_MEAS_opt`"]
pub type Q_MEAS_OPT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `Q_MEAS_opt`"]
pub struct Q_MEAS_OPT_W<'a> {
    w: &'a mut W,
}
impl<'a> Q_MEAS_OPT_W<'a> {
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
#[doc = "Reader of field `HSREF_CLK_DIV`"]
pub type HSREF_CLK_DIV_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `HSREF_CLK_DIV`"]
pub struct HSREF_CLK_DIV_W<'a> {
    w: &'a mut W,
}
impl<'a> HSREF_CLK_DIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&self) -> TS1_EN_R {
        TS1_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&self) -> TS1_START_R {
        TS1_START_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&self) -> TS1_INTRIG_SEL_R {
        TS1_INTRIG_SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&self) -> TS1_SMP_TIME_R {
        TS1_SMP_TIME_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&self) -> REFCLK_SEL_R {
        REFCLK_SEL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&self) -> Q_MEAS_OPT_R {
        Q_MEAS_OPT_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&self) -> HSREF_CLK_DIV_R {
        HSREF_CLK_DIV_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_EN"]
    #[inline(always)]
    pub fn ts1_en(&mut self) -> TS1_EN_W {
        TS1_EN_W { w: self }
    }
    #[doc = "Bit 4 - TS1_START"]
    #[inline(always)]
    pub fn ts1_start(&mut self) -> TS1_START_W {
        TS1_START_W { w: self }
    }
    #[doc = "Bits 8:11 - TS1_INTRIG_SEL"]
    #[inline(always)]
    pub fn ts1_intrig_sel(&mut self) -> TS1_INTRIG_SEL_W {
        TS1_INTRIG_SEL_W { w: self }
    }
    #[doc = "Bits 16:19 - TS1_SMP_TIME"]
    #[inline(always)]
    pub fn ts1_smp_time(&mut self) -> TS1_SMP_TIME_W {
        TS1_SMP_TIME_W { w: self }
    }
    #[doc = "Bit 20 - REFCLK_SEL"]
    #[inline(always)]
    pub fn refclk_sel(&mut self) -> REFCLK_SEL_W {
        REFCLK_SEL_W { w: self }
    }
    #[doc = "Bit 21 - Q_MEAS_opt"]
    #[inline(always)]
    pub fn q_meas_opt(&mut self) -> Q_MEAS_OPT_W {
        Q_MEAS_OPT_W { w: self }
    }
    #[doc = "Bits 24:30 - HSREF_CLK_DIV"]
    #[inline(always)]
    pub fn hsref_clk_div(&mut self) -> HSREF_CLK_DIV_W {
        HSREF_CLK_DIV_W { w: self }
    }
}
