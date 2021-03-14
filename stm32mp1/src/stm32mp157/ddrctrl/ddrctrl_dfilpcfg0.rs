#[doc = "Reader of register DDRCTRL_DFILPCFG0"]
pub type R = crate::R<u32, super::DDRCTRL_DFILPCFG0>;
#[doc = "Writer for register DDRCTRL_DFILPCFG0"]
pub type W = crate::W<u32, super::DDRCTRL_DFILPCFG0>;
#[doc = "Register DDRCTRL_DFILPCFG0 `reset()`'s with value 0x0700_0000"]
impl crate::ResetValue for super::DDRCTRL_DFILPCFG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0700_0000
    }
}
#[doc = "Reader of field `DFI_LP_EN_PD`"]
pub type DFI_LP_EN_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_LP_EN_PD`"]
pub struct DFI_LP_EN_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_EN_PD_W<'a> {
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
#[doc = "Reader of field `DFI_LP_WAKEUP_PD`"]
pub type DFI_LP_WAKEUP_PD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_LP_WAKEUP_PD`"]
pub struct DFI_LP_WAKEUP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Reader of field `DFI_LP_EN_SR`"]
pub type DFI_LP_EN_SR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_LP_EN_SR`"]
pub struct DFI_LP_EN_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_EN_SR_W<'a> {
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
#[doc = "Reader of field `DFI_LP_WAKEUP_SR`"]
pub type DFI_LP_WAKEUP_SR_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_LP_WAKEUP_SR`"]
pub struct DFI_LP_WAKEUP_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Reader of field `DFI_LP_EN_DPD`"]
pub type DFI_LP_EN_DPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DFI_LP_EN_DPD`"]
pub struct DFI_LP_EN_DPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_EN_DPD_W<'a> {
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
#[doc = "Reader of field `DFI_LP_WAKEUP_DPD`"]
pub type DFI_LP_WAKEUP_DPD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_LP_WAKEUP_DPD`"]
pub struct DFI_LP_WAKEUP_DPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_DPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Reader of field `DFI_TLP_RESP`"]
pub type DFI_TLP_RESP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_TLP_RESP`"]
pub struct DFI_TLP_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TLP_RESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFI_LP_EN_PD"]
    #[inline(always)]
    pub fn dfi_lp_en_pd(&self) -> DFI_LP_EN_PD_R {
        DFI_LP_EN_PD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - DFI_LP_WAKEUP_PD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_pd(&self) -> DFI_LP_WAKEUP_PD_R {
        DFI_LP_WAKEUP_PD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - DFI_LP_EN_SR"]
    #[inline(always)]
    pub fn dfi_lp_en_sr(&self) -> DFI_LP_EN_SR_R {
        DFI_LP_EN_SR_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:15 - DFI_LP_WAKEUP_SR"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_sr(&self) -> DFI_LP_WAKEUP_SR_R {
        DFI_LP_WAKEUP_SR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - DFI_LP_EN_DPD"]
    #[inline(always)]
    pub fn dfi_lp_en_dpd(&self) -> DFI_LP_EN_DPD_R {
        DFI_LP_EN_DPD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - DFI_LP_WAKEUP_DPD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_dpd(&self) -> DFI_LP_WAKEUP_DPD_R {
        DFI_LP_WAKEUP_DPD_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - DFI_TLP_RESP"]
    #[inline(always)]
    pub fn dfi_tlp_resp(&self) -> DFI_TLP_RESP_R {
        DFI_TLP_RESP_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_LP_EN_PD"]
    #[inline(always)]
    pub fn dfi_lp_en_pd(&mut self) -> DFI_LP_EN_PD_W {
        DFI_LP_EN_PD_W { w: self }
    }
    #[doc = "Bits 4:7 - DFI_LP_WAKEUP_PD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_pd(&mut self) -> DFI_LP_WAKEUP_PD_W {
        DFI_LP_WAKEUP_PD_W { w: self }
    }
    #[doc = "Bit 8 - DFI_LP_EN_SR"]
    #[inline(always)]
    pub fn dfi_lp_en_sr(&mut self) -> DFI_LP_EN_SR_W {
        DFI_LP_EN_SR_W { w: self }
    }
    #[doc = "Bits 12:15 - DFI_LP_WAKEUP_SR"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_sr(&mut self) -> DFI_LP_WAKEUP_SR_W {
        DFI_LP_WAKEUP_SR_W { w: self }
    }
    #[doc = "Bit 16 - DFI_LP_EN_DPD"]
    #[inline(always)]
    pub fn dfi_lp_en_dpd(&mut self) -> DFI_LP_EN_DPD_W {
        DFI_LP_EN_DPD_W { w: self }
    }
    #[doc = "Bits 20:23 - DFI_LP_WAKEUP_DPD"]
    #[inline(always)]
    pub fn dfi_lp_wakeup_dpd(&mut self) -> DFI_LP_WAKEUP_DPD_W {
        DFI_LP_WAKEUP_DPD_W { w: self }
    }
    #[doc = "Bits 24:28 - DFI_TLP_RESP"]
    #[inline(always)]
    pub fn dfi_tlp_resp(&mut self) -> DFI_TLP_RESP_W {
        DFI_TLP_RESP_W { w: self }
    }
}
