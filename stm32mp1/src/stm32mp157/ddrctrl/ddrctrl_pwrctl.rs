#[doc = "Reader of register DDRCTRL_PWRCTL"]
pub type R = crate::R<u32, super::DDRCTRL_PWRCTL>;
#[doc = "Writer for register DDRCTRL_PWRCTL"]
pub type W = crate::W<u32, super::DDRCTRL_PWRCTL>;
#[doc = "Register DDRCTRL_PWRCTL `reset()`'s with value 0"]
impl crate::ResetValue for super::DDRCTRL_PWRCTL {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SELFREF_EN`"]
pub type SELFREF_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELFREF_EN`"]
pub struct SELFREF_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREF_EN_W<'a> {
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
#[doc = "Reader of field `POWERDOWN_EN`"]
pub type POWERDOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `POWERDOWN_EN`"]
pub struct POWERDOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> POWERDOWN_EN_W<'a> {
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
#[doc = "Reader of field `DEEPPOWERDOWN_EN`"]
pub type DEEPPOWERDOWN_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DEEPPOWERDOWN_EN`"]
pub struct DEEPPOWERDOWN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPPOWERDOWN_EN_W<'a> {
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
#[doc = "Reader of field `EN_DFI_DRAM_CLK_DISABLE`"]
pub type EN_DFI_DRAM_CLK_DISABLE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `EN_DFI_DRAM_CLK_DISABLE`"]
pub struct EN_DFI_DRAM_CLK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_DFI_DRAM_CLK_DISABLE_W<'a> {
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
#[doc = "Reader of field `SELFREF_SW`"]
pub type SELFREF_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SELFREF_SW`"]
pub struct SELFREF_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SELFREF_SW_W<'a> {
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
#[doc = "Reader of field `DIS_CAM_DRAIN_SELFREF`"]
pub type DIS_CAM_DRAIN_SELFREF_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_CAM_DRAIN_SELFREF`"]
pub struct DIS_CAM_DRAIN_SELFREF_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_CAM_DRAIN_SELFREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SELFREF_EN"]
    #[inline(always)]
    pub fn selfref_en(&self) -> SELFREF_EN_R {
        SELFREF_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - POWERDOWN_EN"]
    #[inline(always)]
    pub fn powerdown_en(&self) -> POWERDOWN_EN_R {
        POWERDOWN_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DEEPPOWERDOWN_EN"]
    #[inline(always)]
    pub fn deeppowerdown_en(&self) -> DEEPPOWERDOWN_EN_R {
        DEEPPOWERDOWN_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - EN_DFI_DRAM_CLK_DISABLE"]
    #[inline(always)]
    pub fn en_dfi_dram_clk_disable(&self) -> EN_DFI_DRAM_CLK_DISABLE_R {
        EN_DFI_DRAM_CLK_DISABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SELFREF_SW"]
    #[inline(always)]
    pub fn selfref_sw(&self) -> SELFREF_SW_R {
        SELFREF_SW_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 7 - DIS_CAM_DRAIN_SELFREF"]
    #[inline(always)]
    pub fn dis_cam_drain_selfref(&self) -> DIS_CAM_DRAIN_SELFREF_R {
        DIS_CAM_DRAIN_SELFREF_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SELFREF_EN"]
    #[inline(always)]
    pub fn selfref_en(&mut self) -> SELFREF_EN_W {
        SELFREF_EN_W { w: self }
    }
    #[doc = "Bit 1 - POWERDOWN_EN"]
    #[inline(always)]
    pub fn powerdown_en(&mut self) -> POWERDOWN_EN_W {
        POWERDOWN_EN_W { w: self }
    }
    #[doc = "Bit 2 - DEEPPOWERDOWN_EN"]
    #[inline(always)]
    pub fn deeppowerdown_en(&mut self) -> DEEPPOWERDOWN_EN_W {
        DEEPPOWERDOWN_EN_W { w: self }
    }
    #[doc = "Bit 3 - EN_DFI_DRAM_CLK_DISABLE"]
    #[inline(always)]
    pub fn en_dfi_dram_clk_disable(&mut self) -> EN_DFI_DRAM_CLK_DISABLE_W {
        EN_DFI_DRAM_CLK_DISABLE_W { w: self }
    }
    #[doc = "Bit 5 - SELFREF_SW"]
    #[inline(always)]
    pub fn selfref_sw(&mut self) -> SELFREF_SW_W {
        SELFREF_SW_W { w: self }
    }
    #[doc = "Bit 7 - DIS_CAM_DRAIN_SELFREF"]
    #[inline(always)]
    pub fn dis_cam_drain_selfref(&mut self) -> DIS_CAM_DRAIN_SELFREF_W {
        DIS_CAM_DRAIN_SELFREF_W { w: self }
    }
}
