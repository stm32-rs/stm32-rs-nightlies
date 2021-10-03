#[doc = "Register `DDRCTRL_DFITMG1` reader"]
pub struct R(crate::R<DDRCTRL_DFITMG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFITMG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFITMG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFITMG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFITMG1` writer"]
pub struct W(crate::W<DDRCTRL_DFITMG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFITMG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DDRCTRL_DFITMG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFITMG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_T_DRAM_CLK_ENABLE` reader - DFI_T_DRAM_CLK_ENABLE"]
pub struct DFI_T_DRAM_CLK_ENABLE_R(crate::FieldReader<u8, u8>);
impl DFI_T_DRAM_CLK_ENABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_T_DRAM_CLK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_DRAM_CLK_ENABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_DRAM_CLK_ENABLE` writer - DFI_T_DRAM_CLK_ENABLE"]
pub struct DFI_T_DRAM_CLK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_DRAM_CLK_ENABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `DFI_T_DRAM_CLK_DISABLE` reader - DFI_T_DRAM_CLK_DISABLE"]
pub struct DFI_T_DRAM_CLK_DISABLE_R(crate::FieldReader<u8, u8>);
impl DFI_T_DRAM_CLK_DISABLE_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_T_DRAM_CLK_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_DRAM_CLK_DISABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_DRAM_CLK_DISABLE` writer - DFI_T_DRAM_CLK_DISABLE"]
pub struct DFI_T_DRAM_CLK_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_DRAM_CLK_DISABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
#[doc = "Field `DFI_T_WRDATA_DELAY` reader - DFI_T_WRDATA_DELAY"]
pub struct DFI_T_WRDATA_DELAY_R(crate::FieldReader<u8, u8>);
impl DFI_T_WRDATA_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_T_WRDATA_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_WRDATA_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_WRDATA_DELAY` writer - DFI_T_WRDATA_DELAY"]
pub struct DFI_T_WRDATA_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_WRDATA_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI timing register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfitmg1](index.html) module"]
pub struct DDRCTRL_DFITMG1_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFITMG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfitmg1::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFITMG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfitmg1::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFITMG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DFITMG1 to value 0x0404"]
impl crate::Resettable for DDRCTRL_DFITMG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0404
    }
}
