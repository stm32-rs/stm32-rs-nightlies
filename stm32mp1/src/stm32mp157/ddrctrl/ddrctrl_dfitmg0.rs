#[doc = "Reader of register DDRCTRL_DFITMG0"]
pub type R = crate::R<u32, super::DDRCTRL_DFITMG0>;
#[doc = "Writer for register DDRCTRL_DFITMG0"]
pub type W = crate::W<u32, super::DDRCTRL_DFITMG0>;
#[doc = "Register DDRCTRL_DFITMG0 `reset()`'s with value 0x0702_0002"]
impl crate::ResetValue for super::DDRCTRL_DFITMG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0702_0002
    }
}
#[doc = "Reader of field `DFI_TPHY_WRLAT`"]
pub type DFI_TPHY_WRLAT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_TPHY_WRLAT`"]
pub struct DFI_TPHY_WRLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TPHY_WRLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
#[doc = "Reader of field `DFI_TPHY_WRDATA`"]
pub type DFI_TPHY_WRDATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_TPHY_WRDATA`"]
pub struct DFI_TPHY_WRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TPHY_WRDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_RDDATA_EN`"]
pub type DFI_T_RDDATA_EN_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_RDDATA_EN`"]
pub struct DFI_T_RDDATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_RDDATA_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_CTRL_DELAY`"]
pub type DFI_T_CTRL_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_CTRL_DELAY`"]
pub struct DFI_T_CTRL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | (((value as u32) & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - DFI_TPHY_WRLAT"]
    #[inline(always)]
    pub fn dfi_tphy_wrlat(&self) -> DFI_TPHY_WRLAT_R {
        DFI_TPHY_WRLAT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - DFI_TPHY_WRDATA"]
    #[inline(always)]
    pub fn dfi_tphy_wrdata(&self) -> DFI_TPHY_WRDATA_R {
        DFI_TPHY_WRDATA_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - DFI_T_RDDATA_EN"]
    #[inline(always)]
    pub fn dfi_t_rddata_en(&self) -> DFI_T_RDDATA_EN_R {
        DFI_T_RDDATA_EN_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:28 - DFI_T_CTRL_DELAY"]
    #[inline(always)]
    pub fn dfi_t_ctrl_delay(&self) -> DFI_T_CTRL_DELAY_R {
        DFI_T_CTRL_DELAY_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DFI_TPHY_WRLAT"]
    #[inline(always)]
    pub fn dfi_tphy_wrlat(&mut self) -> DFI_TPHY_WRLAT_W {
        DFI_TPHY_WRLAT_W { w: self }
    }
    #[doc = "Bits 8:13 - DFI_TPHY_WRDATA"]
    #[inline(always)]
    pub fn dfi_tphy_wrdata(&mut self) -> DFI_TPHY_WRDATA_W {
        DFI_TPHY_WRDATA_W { w: self }
    }
    #[doc = "Bits 16:22 - DFI_T_RDDATA_EN"]
    #[inline(always)]
    pub fn dfi_t_rddata_en(&mut self) -> DFI_T_RDDATA_EN_W {
        DFI_T_RDDATA_EN_W { w: self }
    }
    #[doc = "Bits 24:28 - DFI_T_CTRL_DELAY"]
    #[inline(always)]
    pub fn dfi_t_ctrl_delay(&mut self) -> DFI_T_CTRL_DELAY_W {
        DFI_T_CTRL_DELAY_W { w: self }
    }
}
