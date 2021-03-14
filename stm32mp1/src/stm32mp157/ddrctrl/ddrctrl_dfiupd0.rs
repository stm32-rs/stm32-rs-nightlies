#[doc = "Reader of register DDRCTRL_DFIUPD0"]
pub type R = crate::R<u32, super::DDRCTRL_DFIUPD0>;
#[doc = "Writer for register DDRCTRL_DFIUPD0"]
pub type W = crate::W<u32, super::DDRCTRL_DFIUPD0>;
#[doc = "Register DDRCTRL_DFIUPD0 `reset()`'s with value 0x0040_0003"]
impl crate::ResetValue for super::DDRCTRL_DFIUPD0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0040_0003
    }
}
#[doc = "Reader of field `DFI_T_CTRLUP_MIN`"]
pub type DFI_T_CTRLUP_MIN_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DFI_T_CTRLUP_MIN`"]
pub struct DFI_T_CTRLUP_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUP_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_CTRLUP_MAX`"]
pub type DFI_T_CTRLUP_MAX_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DFI_T_CTRLUP_MAX`"]
pub struct DFI_T_CTRLUP_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUP_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `CTRLUPD_PRE_SRX`"]
pub type CTRLUPD_PRE_SRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CTRLUPD_PRE_SRX`"]
pub struct CTRLUPD_PRE_SRX_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRLUPD_PRE_SRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `DIS_AUTO_CTRLUPD_SRX`"]
pub type DIS_AUTO_CTRLUPD_SRX_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_AUTO_CTRLUPD_SRX`"]
pub struct DIS_AUTO_CTRLUPD_SRX_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_CTRLUPD_SRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DIS_AUTO_CTRLUPD`"]
pub type DIS_AUTO_CTRLUPD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DIS_AUTO_CTRLUPD`"]
pub struct DIS_AUTO_CTRLUPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DIS_AUTO_CTRLUPD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - DFI_T_CTRLUP_MIN"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&self) -> DFI_T_CTRLUP_MIN_R {
        DFI_T_CTRLUP_MIN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DFI_T_CTRLUP_MAX"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&self) -> DFI_T_CTRLUP_MAX_R {
        DFI_T_CTRLUP_MAX_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 29 - CTRLUPD_PRE_SRX"]
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&self) -> CTRLUPD_PRE_SRX_R {
        CTRLUPD_PRE_SRX_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - DIS_AUTO_CTRLUPD_SRX"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&self) -> DIS_AUTO_CTRLUPD_SRX_R {
        DIS_AUTO_CTRLUPD_SRX_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DIS_AUTO_CTRLUPD"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&self) -> DIS_AUTO_CTRLUPD_R {
        DIS_AUTO_CTRLUPD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - DFI_T_CTRLUP_MIN"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_min(&mut self) -> DFI_T_CTRLUP_MIN_W {
        DFI_T_CTRLUP_MIN_W { w: self }
    }
    #[doc = "Bits 16:25 - DFI_T_CTRLUP_MAX"]
    #[inline(always)]
    pub fn dfi_t_ctrlup_max(&mut self) -> DFI_T_CTRLUP_MAX_W {
        DFI_T_CTRLUP_MAX_W { w: self }
    }
    #[doc = "Bit 29 - CTRLUPD_PRE_SRX"]
    #[inline(always)]
    pub fn ctrlupd_pre_srx(&mut self) -> CTRLUPD_PRE_SRX_W {
        CTRLUPD_PRE_SRX_W { w: self }
    }
    #[doc = "Bit 30 - DIS_AUTO_CTRLUPD_SRX"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd_srx(&mut self) -> DIS_AUTO_CTRLUPD_SRX_W {
        DIS_AUTO_CTRLUPD_SRX_W { w: self }
    }
    #[doc = "Bit 31 - DIS_AUTO_CTRLUPD"]
    #[inline(always)]
    pub fn dis_auto_ctrlupd(&mut self) -> DIS_AUTO_CTRLUPD_W {
        DIS_AUTO_CTRLUPD_W { w: self }
    }
}
