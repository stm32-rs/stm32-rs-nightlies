#[doc = "Register `OTG_HCFG` reader"]
pub struct R(crate::R<OTG_HCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTG_HCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTG_HCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTG_HCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTG_HCFG` writer"]
pub struct W(crate::W<OTG_HCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTG_HCFG_SPEC>;
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
impl From<crate::W<OTG_HCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTG_HCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSLSPCS` reader - FSLSPCS"]
pub struct FSLSPCS_R(crate::FieldReader<u8, u8>);
impl FSLSPCS_R {
    pub(crate) fn new(bits: u8) -> Self {
        FSLSPCS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLSPCS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FSLSPCS` writer - FSLSPCS"]
pub struct FSLSPCS_W<'a> {
    w: &'a mut W,
}
impl<'a> FSLSPCS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `FSLSS` reader - FSLSS"]
pub struct FSLSS_R(crate::FieldReader<bool, bool>);
impl FSLSS_R {
    pub(crate) fn new(bits: bool) -> Self {
        FSLSS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FSLSS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESCDMA` reader - DESCDMA"]
pub struct DESCDMA_R(crate::FieldReader<bool, bool>);
impl DESCDMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        DESCDMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DESCDMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DESCDMA` writer - DESCDMA"]
pub struct DESCDMA_W<'a> {
    w: &'a mut W,
}
impl<'a> DESCDMA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `FRLSTEN` reader - FRLSTEN"]
pub struct FRLSTEN_R(crate::FieldReader<u8, u8>);
impl FRLSTEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        FRLSTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FRLSTEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FRLSTEN` writer - FRLSTEN"]
pub struct FRLSTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> FRLSTEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `PERSSCHEDENA` reader - PERSSCHEDENA"]
pub struct PERSSCHEDENA_R(crate::FieldReader<bool, bool>);
impl PERSSCHEDENA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERSSCHEDENA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERSSCHEDENA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERSSCHEDENA` writer - PERSSCHEDENA"]
pub struct PERSSCHEDENA_W<'a> {
    w: &'a mut W,
}
impl<'a> PERSSCHEDENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | ((value as u32 & 0x01) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FSLSPCS_R {
        FSLSPCS_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - FSLSS"]
    #[inline(always)]
    pub fn fslss(&self) -> FSLSS_R {
        FSLSS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&self) -> DESCDMA_R {
        DESCDMA_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&self) -> FRLSTEN_R {
        FRLSTEN_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&self) -> PERSSCHEDENA_R {
        PERSSCHEDENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FSLSPCS"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FSLSPCS_W {
        FSLSPCS_W { w: self }
    }
    #[doc = "Bit 23 - DESCDMA"]
    #[inline(always)]
    pub fn descdma(&mut self) -> DESCDMA_W {
        DESCDMA_W { w: self }
    }
    #[doc = "Bits 24:25 - FRLSTEN"]
    #[inline(always)]
    pub fn frlsten(&mut self) -> FRLSTEN_W {
        FRLSTEN_W { w: self }
    }
    #[doc = "Bit 26 - PERSSCHEDENA"]
    #[inline(always)]
    pub fn persschedena(&mut self) -> PERSSCHEDENA_W {
        PERSSCHEDENA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register configures the core after power-on. Do not make changes to this register after initializing the host.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otg_hcfg](index.html) module"]
pub struct OTG_HCFG_SPEC;
impl crate::RegisterSpec for OTG_HCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otg_hcfg::R](R) reader structure"]
impl crate::Readable for OTG_HCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otg_hcfg::W](W) writer structure"]
impl crate::Writable for OTG_HCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTG_HCFG to value 0"]
impl crate::Resettable for OTG_HCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
