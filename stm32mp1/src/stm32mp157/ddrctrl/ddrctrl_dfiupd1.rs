#[doc = "Reader of register DDRCTRL_DFIUPD1"]
pub type R = crate::R<u32, super::DDRCTRL_DFIUPD1>;
#[doc = "Writer for register DDRCTRL_DFIUPD1"]
pub type W = crate::W<u32, super::DDRCTRL_DFIUPD1>;
#[doc = "Register DDRCTRL_DFIUPD1 `reset()`'s with value 0x0001_0001"]
impl crate::ResetValue for super::DDRCTRL_DFIUPD1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0001_0001
    }
}
#[doc = "Reader of field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024`"]
pub type DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_CTRLUPD_INTERVAL_MAX_X1024`"]
pub struct DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024`"]
pub type DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_CTRLUPD_INTERVAL_MIN_X1024`"]
pub struct DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_max_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_min_x1024(&self) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DFI_T_CTRLUPD_INTERVAL_MAX_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_max_x1024(&mut self) -> DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W {
        DFI_T_CTRLUPD_INTERVAL_MAX_X1024_W { w: self }
    }
    #[doc = "Bits 16:23 - DFI_T_CTRLUPD_INTERVAL_MIN_X1024"]
    #[inline(always)]
    pub fn dfi_t_ctrlupd_interval_min_x1024(&mut self) -> DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W {
        DFI_T_CTRLUPD_INTERVAL_MIN_X1024_W { w: self }
    }
}
