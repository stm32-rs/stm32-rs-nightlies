#[doc = "Reader of register DDRCTRL_HWLPCTL"]
pub type R = crate::R<u32, super::DDRCTRL_HWLPCTL>;
#[doc = "Writer for register DDRCTRL_HWLPCTL"]
pub type W = crate::W<u32, super::DDRCTRL_HWLPCTL>;
#[doc = "Register DDRCTRL_HWLPCTL `reset()`'s with value 0x03"]
impl crate::ResetValue for super::DDRCTRL_HWLPCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x03
    }
}
#[doc = "Reader of field `HW_LP_EN`"]
pub type HW_LP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_LP_EN`"]
pub struct HW_LP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_EN_W<'a> {
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
#[doc = "Reader of field `HW_LP_EXIT_IDLE_EN`"]
pub type HW_LP_EXIT_IDLE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HW_LP_EXIT_IDLE_EN`"]
pub struct HW_LP_EXIT_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_EXIT_IDLE_EN_W<'a> {
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
#[doc = "Reader of field `HW_LP_IDLE_X32`"]
pub type HW_LP_IDLE_X32_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `HW_LP_IDLE_X32`"]
pub struct HW_LP_IDLE_X32_W<'a> {
    w: &'a mut W,
}
impl<'a> HW_LP_IDLE_X32_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | (((value as u32) & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&self) -> HW_LP_EN_R {
        HW_LP_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&self) -> HW_LP_EXIT_IDLE_EN_R {
        HW_LP_EXIT_IDLE_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&self) -> HW_LP_IDLE_X32_R {
        HW_LP_IDLE_X32_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - HW_LP_EN"]
    #[inline(always)]
    pub fn hw_lp_en(&mut self) -> HW_LP_EN_W {
        HW_LP_EN_W { w: self }
    }
    #[doc = "Bit 1 - HW_LP_EXIT_IDLE_EN"]
    #[inline(always)]
    pub fn hw_lp_exit_idle_en(&mut self) -> HW_LP_EXIT_IDLE_EN_W {
        HW_LP_EXIT_IDLE_EN_W { w: self }
    }
    #[doc = "Bits 16:27 - HW_LP_IDLE_X32"]
    #[inline(always)]
    pub fn hw_lp_idle_x32(&mut self) -> HW_LP_IDLE_X32_W {
        HW_LP_IDLE_X32_W { w: self }
    }
}
