#[doc = "Register `OTG_FS_GCCFG` reader"]
pub struct R(crate::R<OTG_FS_GCCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_FS_GCCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_FS_GCCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_FS_GCCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_FS_GCCFG` writer"]
pub struct W(crate::W<OTG_FS_GCCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_FS_GCCFG_SPEC>;
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
impl From<crate::W<OTG_FS_GCCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_FS_GCCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWRDWN` reader - Power down"]
pub struct PWRDWN_R(crate::FieldReader<bool, bool>);
impl PWRDWN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRDWN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRDWN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRDWN` writer - Power down"]
pub struct PWRDWN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRDWN_W<'a> {
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
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable"]
pub struct BCDEN_R(crate::FieldReader<bool, bool>);
impl BCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        BCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable"]
pub struct BCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> BCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable"]
pub struct DCDEN_R(crate::FieldReader<bool, bool>);
impl DCDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable"]
pub struct DCDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable"]
pub struct PDEN_R(crate::FieldReader<bool, bool>);
impl PDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable"]
pub struct PDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable"]
pub struct SDEN_R(crate::FieldReader<bool, bool>);
impl SDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable"]
pub struct SDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `VBDEN` reader - USB VBUS detection enable"]
pub struct VBDEN_R(crate::FieldReader<bool, bool>);
impl VBDEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VBDEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBDEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VBDEN` writer - USB VBUS detection enable"]
pub struct VBDEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VBDEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status"]
pub struct DCDET_R(crate::FieldReader<bool, bool>);
impl DCDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        DCDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDET` writer - Data contact detection (DCD) status"]
pub struct DCDET_W<'a> {
    w: &'a mut W,
}
impl<'a> DCDET_W<'a> {
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
#[doc = "Field `PDET` reader - Primary detection (PD) status"]
pub struct PDET_R(crate::FieldReader<bool, bool>);
impl PDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDET` writer - Primary detection (PD) status"]
pub struct PDET_W<'a> {
    w: &'a mut W,
}
impl<'a> PDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SDET` reader - Secondary detection (SD) status"]
pub struct SDET_R(crate::FieldReader<bool, bool>);
impl SDET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDET` writer - Secondary detection (SD) status"]
pub struct SDET_W<'a> {
    w: &'a mut W,
}
impl<'a> SDET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub struct PS2DET_R(crate::FieldReader<bool, bool>);
impl PS2DET_R {
    pub(crate) fn new(bits: bool) -> Self {
        PS2DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS2DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PS2DET` writer - DM pull-up detection status"]
pub struct PS2DET_W<'a> {
    w: &'a mut W,
}
impl<'a> PS2DET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PWRDWN_R {
        PWRDWN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&self) -> BCDEN_R {
        BCDEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DCDEN_R {
        DCDEN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&self) -> PDEN_R {
        PDEN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&self) -> SDEN_R {
        SDEN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    pub fn vbden(&self) -> VBDEN_R {
        VBDEN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&self) -> DCDET_R {
        DCDET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&self) -> PDET_R {
        PDET_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&self) -> SDET_R {
        SDET_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> PS2DET_R {
        PS2DET_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PWRDWN_W {
        PWRDWN_W { w: self }
    }
    #[doc = "Bit 17 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&mut self) -> BCDEN_W {
        BCDEN_W { w: self }
    }
    #[doc = "Bit 18 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&mut self) -> DCDEN_W {
        DCDEN_W { w: self }
    }
    #[doc = "Bit 19 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PDEN_W {
        PDEN_W { w: self }
    }
    #[doc = "Bit 20 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&mut self) -> SDEN_W {
        SDEN_W { w: self }
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    pub fn vbden(&mut self) -> VBDEN_W {
        VBDEN_W { w: self }
    }
    #[doc = "Bit 0 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&mut self) -> DCDET_W {
        DCDET_W { w: self }
    }
    #[doc = "Bit 1 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&mut self) -> PDET_W {
        PDET_W { w: self }
    }
    #[doc = "Bit 2 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&mut self) -> SDET_W {
        SDET_W { w: self }
    }
    #[doc = "Bit 3 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&mut self) -> PS2DET_W {
        PS2DET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG_FS general core configuration register (OTG_FS_GCCFG)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_fs_gccfg](index.html) module"]
pub struct OTG_FS_GCCFG_SPEC;
impl crate::RegisterSpec for OTG_FS_GCCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_fs_gccfg::R](R) reader structure"]
impl crate::Readable for OTG_FS_GCCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_fs_gccfg::W](W) writer structure"]
impl crate::Writable for OTG_FS_GCCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_FS_GCCFG to value 0"]
impl crate::Resettable for OTG_FS_GCCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
