#[doc = "Register `DDRCTRL_DFIMISC` reader"]
pub struct R(crate::R<DDRCTRL_DFIMISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIMISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIMISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIMISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFIMISC` writer"]
pub struct W(crate::W<DDRCTRL_DFIMISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIMISC_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIMISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIMISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_INIT_COMPLETE_EN` reader - DFI_INIT_COMPLETE_EN"]
pub struct DFI_INIT_COMPLETE_EN_R(crate::FieldReader<bool, bool>);
impl DFI_INIT_COMPLETE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_INIT_COMPLETE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_INIT_COMPLETE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_INIT_COMPLETE_EN` writer - DFI_INIT_COMPLETE_EN"]
pub struct DFI_INIT_COMPLETE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_INIT_COMPLETE_EN_W<'a> {
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
#[doc = "Field `CTL_IDLE_EN` reader - CTL_IDLE_EN"]
pub struct CTL_IDLE_EN_R(crate::FieldReader<bool, bool>);
impl CTL_IDLE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTL_IDLE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTL_IDLE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTL_IDLE_EN` writer - CTL_IDLE_EN"]
pub struct CTL_IDLE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTL_IDLE_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `DFI_INIT_START` reader - DFI_INIT_START"]
pub struct DFI_INIT_START_R(crate::FieldReader<bool, bool>);
impl DFI_INIT_START_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFI_INIT_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_INIT_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_INIT_START` writer - DFI_INIT_START"]
pub struct DFI_INIT_START_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_INIT_START_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `DFI_FREQUENCY` reader - DFI_FREQUENCY"]
pub struct DFI_FREQUENCY_R(crate::FieldReader<u8, u8>);
impl DFI_FREQUENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DFI_FREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_FREQUENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_FREQUENCY` writer - DFI_FREQUENCY"]
pub struct DFI_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 8)) | ((value as u32 & 0x1f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    pub fn dfi_init_complete_en(&self) -> DFI_INIT_COMPLETE_EN_R {
        DFI_INIT_COMPLETE_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    pub fn ctl_idle_en(&self) -> CTL_IDLE_EN_R {
        CTL_IDLE_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    pub fn dfi_init_start(&self) -> DFI_INIT_START_R {
        DFI_INIT_START_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    pub fn dfi_frequency(&self) -> DFI_FREQUENCY_R {
        DFI_FREQUENCY_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DFI_INIT_COMPLETE_EN"]
    #[inline(always)]
    pub fn dfi_init_complete_en(&mut self) -> DFI_INIT_COMPLETE_EN_W {
        DFI_INIT_COMPLETE_EN_W { w: self }
    }
    #[doc = "Bit 4 - CTL_IDLE_EN"]
    #[inline(always)]
    pub fn ctl_idle_en(&mut self) -> CTL_IDLE_EN_W {
        CTL_IDLE_EN_W { w: self }
    }
    #[doc = "Bit 5 - DFI_INIT_START"]
    #[inline(always)]
    pub fn dfi_init_start(&mut self) -> DFI_INIT_START_W {
        DFI_INIT_START_W { w: self }
    }
    #[doc = "Bits 8:12 - DFI_FREQUENCY"]
    #[inline(always)]
    pub fn dfi_frequency(&mut self) -> DFI_FREQUENCY_W {
        DFI_FREQUENCY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI miscellaneous control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfimisc](index.html) module"]
pub struct DDRCTRL_DFIMISC_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfimisc::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIMISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfimisc::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIMISC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DFIMISC to value 0x01"]
impl crate::Resettable for DDRCTRL_DFIMISC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
