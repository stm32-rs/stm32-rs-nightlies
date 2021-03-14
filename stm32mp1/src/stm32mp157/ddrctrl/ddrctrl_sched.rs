#[doc = "Reader of register DDRCTRL_SCHED"]
pub type R = crate::R<u32, super::DDRCTRL_SCHED>;
#[doc = "Writer for register DDRCTRL_SCHED"]
pub type W = crate::W<u32, super::DDRCTRL_SCHED>;
#[doc = "Register DDRCTRL_SCHED `reset()`'s with value 0x0805"]
impl crate::ResetValue for super::DDRCTRL_SCHED {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0805
    }
}
#[doc = "Reader of field `FORCE_LOW_PRI_N`"]
pub type FORCE_LOW_PRI_N_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FORCE_LOW_PRI_N`"]
pub struct FORCE_LOW_PRI_N_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_LOW_PRI_N_W<'a> {
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
#[doc = "Reader of field `PREFER_WRITE`"]
pub type PREFER_WRITE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PREFER_WRITE`"]
pub struct PREFER_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> PREFER_WRITE_W<'a> {
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
#[doc = "Reader of field `PAGECLOSE`"]
pub type PAGECLOSE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PAGECLOSE`"]
pub struct PAGECLOSE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGECLOSE_W<'a> {
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
#[doc = "Reader of field `LPR_NUM_ENTRIES`"]
pub type LPR_NUM_ENTRIES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `LPR_NUM_ENTRIES`"]
pub struct LPR_NUM_ENTRIES_W<'a> {
    w: &'a mut W,
}
impl<'a> LPR_NUM_ENTRIES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `GO2CRITICAL_HYSTERESIS`"]
pub type GO2CRITICAL_HYSTERESIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GO2CRITICAL_HYSTERESIS`"]
pub struct GO2CRITICAL_HYSTERESIS_W<'a> {
    w: &'a mut W,
}
impl<'a> GO2CRITICAL_HYSTERESIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `RDWR_IDLE_GAP`"]
pub type RDWR_IDLE_GAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RDWR_IDLE_GAP`"]
pub struct RDWR_IDLE_GAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RDWR_IDLE_GAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    pub fn force_low_pri_n(&self) -> FORCE_LOW_PRI_N_R {
        FORCE_LOW_PRI_N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    pub fn prefer_write(&self) -> PREFER_WRITE_R {
        PREFER_WRITE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    pub fn pageclose(&self) -> PAGECLOSE_R {
        PAGECLOSE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    pub fn lpr_num_entries(&self) -> LPR_NUM_ENTRIES_R {
        LPR_NUM_ENTRIES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    pub fn go2critical_hysteresis(&self) -> GO2CRITICAL_HYSTERESIS_R {
        GO2CRITICAL_HYSTERESIS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    pub fn rdwr_idle_gap(&self) -> RDWR_IDLE_GAP_R {
        RDWR_IDLE_GAP_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FORCE_LOW_PRI_N"]
    #[inline(always)]
    pub fn force_low_pri_n(&mut self) -> FORCE_LOW_PRI_N_W {
        FORCE_LOW_PRI_N_W { w: self }
    }
    #[doc = "Bit 1 - PREFER_WRITE"]
    #[inline(always)]
    pub fn prefer_write(&mut self) -> PREFER_WRITE_W {
        PREFER_WRITE_W { w: self }
    }
    #[doc = "Bit 2 - PAGECLOSE"]
    #[inline(always)]
    pub fn pageclose(&mut self) -> PAGECLOSE_W {
        PAGECLOSE_W { w: self }
    }
    #[doc = "Bits 8:11 - LPR_NUM_ENTRIES"]
    #[inline(always)]
    pub fn lpr_num_entries(&mut self) -> LPR_NUM_ENTRIES_W {
        LPR_NUM_ENTRIES_W { w: self }
    }
    #[doc = "Bits 16:23 - GO2CRITICAL_HYSTERESIS"]
    #[inline(always)]
    pub fn go2critical_hysteresis(&mut self) -> GO2CRITICAL_HYSTERESIS_W {
        GO2CRITICAL_HYSTERESIS_W { w: self }
    }
    #[doc = "Bits 24:30 - RDWR_IDLE_GAP"]
    #[inline(always)]
    pub fn rdwr_idle_gap(&mut self) -> RDWR_IDLE_GAP_W {
        RDWR_IDLE_GAP_W { w: self }
    }
}
