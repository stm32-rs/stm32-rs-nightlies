#[doc = "Register `DDRCTRL_DFILPCFG0` reader"]
pub struct R(crate::R<DDRCTRL_DFILPCFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFILPCFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFILPCFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFILPCFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFILPCFG0` writer"]
pub struct W(crate::W<DDRCTRL_DFILPCFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFILPCFG0_SPEC>;
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
impl From<crate::W<DDRCTRL_DFILPCFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFILPCFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_LP_EN_PD` reader - DFI_LP_EN_PD"]
pub struct DFI_LP_EN_PD_R(crate::FieldReader<bool, bool>);
impl DFI_LP_EN_PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_LP_EN_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_EN_PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_EN_PD` writer - DFI_LP_EN_PD"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DFI_LP_WAKEUP_PD` reader - DFI_LP_WAKEUP_PD"]
pub struct DFI_LP_WAKEUP_PD_R(crate::FieldReader<u8, u8>);
impl DFI_LP_WAKEUP_PD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_LP_WAKEUP_PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_WAKEUP_PD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_WAKEUP_PD` writer - DFI_LP_WAKEUP_PD"]
pub struct DFI_LP_WAKEUP_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_PD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `DFI_LP_EN_SR` reader - DFI_LP_EN_SR"]
pub struct DFI_LP_EN_SR_R(crate::FieldReader<bool, bool>);
impl DFI_LP_EN_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_LP_EN_SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_EN_SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_EN_SR` writer - DFI_LP_EN_SR"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `DFI_LP_WAKEUP_SR` reader - DFI_LP_WAKEUP_SR"]
pub struct DFI_LP_WAKEUP_SR_R(crate::FieldReader<u8, u8>);
impl DFI_LP_WAKEUP_SR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_LP_WAKEUP_SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_WAKEUP_SR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_WAKEUP_SR` writer - DFI_LP_WAKEUP_SR"]
pub struct DFI_LP_WAKEUP_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_SR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `DFI_LP_EN_DPD` reader - DFI_LP_EN_DPD"]
pub struct DFI_LP_EN_DPD_R(crate::FieldReader<bool, bool>);
impl DFI_LP_EN_DPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_LP_EN_DPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_EN_DPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_EN_DPD` writer - DFI_LP_EN_DPD"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `DFI_LP_WAKEUP_DPD` reader - DFI_LP_WAKEUP_DPD"]
pub struct DFI_LP_WAKEUP_DPD_R(crate::FieldReader<u8, u8>);
impl DFI_LP_WAKEUP_DPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_LP_WAKEUP_DPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_LP_WAKEUP_DPD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_LP_WAKEUP_DPD` writer - DFI_LP_WAKEUP_DPD"]
pub struct DFI_LP_WAKEUP_DPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_LP_WAKEUP_DPD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `DFI_TLP_RESP` reader - DFI_TLP_RESP"]
pub struct DFI_TLP_RESP_R(crate::FieldReader<u8, u8>);
impl DFI_TLP_RESP_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_TLP_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_TLP_RESP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_TLP_RESP` writer - DFI_TLP_RESP"]
pub struct DFI_TLP_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_TLP_RESP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL low power configuration register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfilpcfg0](index.html) module"]
pub struct DDRCTRL_DFILPCFG0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFILPCFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfilpcfg0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFILPCFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfilpcfg0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFILPCFG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DFILPCFG0 to value 0x0700_0000"]
impl crate::Resettable for DDRCTRL_DFILPCFG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0700_0000
    }
}
