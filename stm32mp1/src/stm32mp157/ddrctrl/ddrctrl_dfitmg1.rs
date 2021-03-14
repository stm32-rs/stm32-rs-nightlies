#[doc = "Reader of register DDRCTRL_DFITMG1"]
pub type R = crate::R<u32, super::DDRCTRL_DFITMG1>;
#[doc = "Writer for register DDRCTRL_DFITMG1"]
pub type W = crate::W<u32, super::DDRCTRL_DFITMG1>;
#[doc = "Register DDRCTRL_DFITMG1 `reset()`'s with value 0x0404"]
impl crate::ResetValue for super::DDRCTRL_DFITMG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0404
    }
}
#[doc = "Reader of field `DFI_T_DRAM_CLK_ENABLE`"]
pub type DFI_T_DRAM_CLK_ENABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_DRAM_CLK_ENABLE`"]
pub struct DFI_T_DRAM_CLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_DRAM_CLK_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_DRAM_CLK_DISABLE`"]
pub type DFI_T_DRAM_CLK_DISABLE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_DRAM_CLK_DISABLE`"]
pub struct DFI_T_DRAM_CLK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_DRAM_CLK_DISABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | (((value as u32) & 0x1f) << 8);
        self.w
    }
}
#[doc = "Reader of field `DFI_T_WRDATA_DELAY`"]
pub type DFI_T_WRDATA_DELAY_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DFI_T_WRDATA_DELAY`"]
pub struct DFI_T_WRDATA_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_WRDATA_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | (((value as u32) & 0x1f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - DFI_T_DRAM_CLK_ENABLE"]
    #[inline(always)]
    pub fn dfi_t_dram_clk_enable(&self) -> DFI_T_DRAM_CLK_ENABLE_R {
        DFI_T_DRAM_CLK_ENABLE_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - DFI_T_DRAM_CLK_DISABLE"]
    #[inline(always)]
    pub fn dfi_t_dram_clk_disable(&self) -> DFI_T_DRAM_CLK_DISABLE_R {
        DFI_T_DRAM_CLK_DISABLE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - DFI_T_WRDATA_DELAY"]
    #[inline(always)]
    pub fn dfi_t_wrdata_delay(&self) -> DFI_T_WRDATA_DELAY_R {
        DFI_T_WRDATA_DELAY_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DFI_T_DRAM_CLK_ENABLE"]
    #[inline(always)]
    pub fn dfi_t_dram_clk_enable(&mut self) -> DFI_T_DRAM_CLK_ENABLE_W {
        DFI_T_DRAM_CLK_ENABLE_W { w: self }
    }
    #[doc = "Bits 8:12 - DFI_T_DRAM_CLK_DISABLE"]
    #[inline(always)]
    pub fn dfi_t_dram_clk_disable(&mut self) -> DFI_T_DRAM_CLK_DISABLE_W {
        DFI_T_DRAM_CLK_DISABLE_W { w: self }
    }
    #[doc = "Bits 16:20 - DFI_T_WRDATA_DELAY"]
    #[inline(always)]
    pub fn dfi_t_wrdata_delay(&mut self) -> DFI_T_WRDATA_DELAY_W {
        DFI_T_WRDATA_DELAY_W { w: self }
    }
}
