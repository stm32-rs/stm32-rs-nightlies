#[doc = "Register `DDRCTRL_DFITMG0` reader"]
pub struct R(crate::R<DDRCTRL_DFITMG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFITMG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFITMG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFITMG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFITMG0` writer"]
pub struct W(crate::W<DDRCTRL_DFITMG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFITMG0_SPEC>;
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
impl From<crate::W<DDRCTRL_DFITMG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFITMG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_TPHY_WRLAT` reader - DFI_TPHY_WRLAT"]
pub struct DFI_TPHY_WRLAT_R(crate::FieldReader<u8, u8>);
impl DFI_TPHY_WRLAT_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_TPHY_WRLAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_TPHY_WRLAT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_TPHY_WRLAT` writer - DFI_TPHY_WRLAT"]
pub struct DFI_TPHY_WRLAT_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TPHY_WRLAT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `DFI_TPHY_WRDATA` reader - DFI_TPHY_WRDATA"]
pub struct DFI_TPHY_WRDATA_R(crate::FieldReader<u8, u8>);
impl DFI_TPHY_WRDATA_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_TPHY_WRDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_TPHY_WRDATA_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_TPHY_WRDATA` writer - DFI_TPHY_WRDATA"]
pub struct DFI_TPHY_WRDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TPHY_WRDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `DFI_T_RDDATA_EN` reader - DFI_T_RDDATA_EN"]
pub struct DFI_T_RDDATA_EN_R(crate::FieldReader<u8, u8>);
impl DFI_T_RDDATA_EN_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_T_RDDATA_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_RDDATA_EN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_RDDATA_EN` writer - DFI_T_RDDATA_EN"]
pub struct DFI_T_RDDATA_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_RDDATA_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | ((value as u32 & 0x7f) << 16);
        self.w
    }
}
#[doc = "Field `DFI_T_CTRL_DELAY` reader - DFI_T_CTRL_DELAY"]
pub struct DFI_T_CTRL_DELAY_R(crate::FieldReader<u8, u8>);
impl DFI_T_CTRL_DELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_T_CTRL_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_CTRL_DELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_CTRL_DELAY` writer - DFI_T_CTRL_DELAY"]
pub struct DFI_T_CTRL_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRL_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI timing register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfitmg0](index.html) module"]
pub struct DDRCTRL_DFITMG0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFITMG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfitmg0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFITMG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfitmg0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFITMG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DFITMG0 to value 0x0702_0002"]
impl crate::Resettable for DDRCTRL_DFITMG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0702_0002
    }
}
