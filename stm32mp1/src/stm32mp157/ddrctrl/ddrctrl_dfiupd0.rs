#[doc = "Register `DDRCTRL_DFIUPD0` reader"]
pub struct R(crate::R<DDRCTRL_DFIUPD0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRCTRL_DFIUPD0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRCTRL_DFIUPD0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRCTRL_DFIUPD0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRCTRL_DFIUPD0` writer"]
pub struct W(crate::W<DDRCTRL_DFIUPD0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRCTRL_DFIUPD0_SPEC>;
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
impl From<crate::W<DDRCTRL_DFIUPD0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRCTRL_DFIUPD0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFI_T_CTRLUP_MIN` reader - DFI_T_CTRLUP_MIN"]
pub struct DFI_T_CTRLUP_MIN_R(crate::FieldReader<u16, u16>);
impl DFI_T_CTRLUP_MIN_R {
    pub(crate) fn new(bits: u16) -> Self {
        DFI_T_CTRLUP_MIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_CTRLUP_MIN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_CTRLUP_MIN` writer - DFI_T_CTRLUP_MIN"]
pub struct DFI_T_CTRLUP_MIN_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUP_MIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `DFI_T_CTRLUP_MAX` reader - DFI_T_CTRLUP_MAX"]
pub struct DFI_T_CTRLUP_MAX_R(crate::FieldReader<u16, u16>);
impl DFI_T_CTRLUP_MAX_R {
    pub(crate) fn new(bits: u16) -> Self {
        DFI_T_CTRLUP_MAX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFI_T_CTRLUP_MAX_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFI_T_CTRLUP_MAX` writer - DFI_T_CTRLUP_MAX"]
pub struct DFI_T_CTRLUP_MAX_W<'a> {
    w: &'a mut W,
}
impl<'a> DFI_T_CTRLUP_MAX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | ((value as u32 & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Field `CTRLUPD_PRE_SRX` reader - CTRLUPD_PRE_SRX"]
pub struct CTRLUPD_PRE_SRX_R(crate::FieldReader<bool, bool>);
impl CTRLUPD_PRE_SRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTRLUPD_PRE_SRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRLUPD_PRE_SRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTRLUPD_PRE_SRX` writer - CTRLUPD_PRE_SRX"]
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
#[doc = "Field `DIS_AUTO_CTRLUPD_SRX` reader - DIS_AUTO_CTRLUPD_SRX"]
pub struct DIS_AUTO_CTRLUPD_SRX_R(crate::FieldReader<bool, bool>);
impl DIS_AUTO_CTRLUPD_SRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_AUTO_CTRLUPD_SRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_AUTO_CTRLUPD_SRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_AUTO_CTRLUPD_SRX` writer - DIS_AUTO_CTRLUPD_SRX"]
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `DIS_AUTO_CTRLUPD` reader - DIS_AUTO_CTRLUPD"]
pub struct DIS_AUTO_CTRLUPD_R(crate::FieldReader<bool, bool>);
impl DIS_AUTO_CTRLUPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIS_AUTO_CTRLUPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_AUTO_CTRLUPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIS_AUTO_CTRLUPD` writer - DIS_AUTO_CTRLUPD"]
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRCTRL DFI update register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrctrl_dfiupd0](index.html) module"]
pub struct DDRCTRL_DFIUPD0_SPEC;
impl crate::RegisterSpec for DDRCTRL_DFIUPD0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrctrl_dfiupd0::R](R) reader structure"]
impl crate::Readable for DDRCTRL_DFIUPD0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrctrl_dfiupd0::W](W) writer structure"]
impl crate::Writable for DDRCTRL_DFIUPD0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRCTRL_DFIUPD0 to value 0x0040_0003"]
impl crate::Resettable for DDRCTRL_DFIUPD0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0003
    }
}
