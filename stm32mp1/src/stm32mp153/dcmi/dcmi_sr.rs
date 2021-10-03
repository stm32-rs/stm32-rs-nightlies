#[doc = "Register `DCMI_SR` reader"]
pub struct R(crate::R<DCMI_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCMI_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCMI_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCMI_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HSYNC` reader - HSYNC"]
pub struct HSYNC_R(crate::FieldReader<bool, bool>);
impl HSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        HSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VSYNC` reader - VSYNC"]
pub struct VSYNC_R(crate::FieldReader<bool, bool>);
impl VSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        VSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FNE` reader - FNE"]
pub struct FNE_R(crate::FieldReader<bool, bool>);
impl FNE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HSYNC"]
    #[inline(always)]
    pub fn hsync(&self) -> HSYNC_R {
        HSYNC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VSYNC"]
    #[inline(always)]
    pub fn vsync(&self) -> VSYNC_R {
        VSYNC_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - FNE"]
    #[inline(always)]
    pub fn fne(&self) -> FNE_R {
        FNE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "DCMI status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcmi_sr](index.html) module"]
pub struct DCMI_SR_SPEC;
impl crate::RegisterSpec for DCMI_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcmi_sr::R](R) reader structure"]
impl crate::Readable for DCMI_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DCMI_SR to value 0"]
impl crate::Resettable for DCMI_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
