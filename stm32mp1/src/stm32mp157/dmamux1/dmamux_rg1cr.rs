#[doc = "Register `DMAMUX_RG1CR` reader"]
pub struct R(crate::R<DMAMUX_RG1CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMAMUX_RG1CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMAMUX_RG1CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMAMUX_RG1CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMAMUX_RG1CR` writer"]
pub struct W(crate::W<DMAMUX_RG1CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMAMUX_RG1CR_SPEC>;
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
impl From<crate::W<DMAMUX_RG1CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMAMUX_RG1CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIG_ID` reader - SIG_ID"]
pub struct SIG_ID_R(crate::FieldReader<u8, u8>);
impl SIG_ID_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIG_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIG_ID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIG_ID` writer - SIG_ID"]
pub struct SIG_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SIG_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `OIE` reader - OIE"]
pub struct OIE_R(crate::FieldReader<bool, bool>);
impl OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        OIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OIE` writer - OIE"]
pub struct OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> OIE_W<'a> {
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
#[doc = "Field `GE` reader - GE"]
pub struct GE_R(crate::FieldReader<bool, bool>);
impl GE_R {
    pub(crate) fn new(bits: bool) -> Self {
        GE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GE` writer - GE"]
pub struct GE_W<'a> {
    w: &'a mut W,
}
impl<'a> GE_W<'a> {
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
#[doc = "Field `GPOL` reader - GPOL"]
pub struct GPOL_R(crate::FieldReader<u8, u8>);
impl GPOL_R {
    pub(crate) fn new(bits: u8) -> Self {
        GPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPOL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GPOL` writer - GPOL"]
pub struct GPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPOL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 17)) | ((value as u32 & 0x03) << 17);
        self.w
    }
}
#[doc = "Field `GNBREQ` reader - GNBREQ"]
pub struct GNBREQ_R(crate::FieldReader<u8, u8>);
impl GNBREQ_R {
    pub(crate) fn new(bits: u8) -> Self {
        GNBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNBREQ_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GNBREQ` writer - GNBREQ"]
pub struct GNBREQ_W<'a> {
    w: &'a mut W,
}
impl<'a> GNBREQ_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 19)) | ((value as u32 & 0x1f) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 0x03) as u8)
    }
    #[doc = "Bits 19:23 - GNBREQ"]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SIG_ID"]
    #[inline(always)]
    pub fn sig_id(&mut self) -> SIG_ID_W {
        SIG_ID_W { w: self }
    }
    #[doc = "Bit 8 - OIE"]
    #[inline(always)]
    pub fn oie(&mut self) -> OIE_W {
        OIE_W { w: self }
    }
    #[doc = "Bit 16 - GE"]
    #[inline(always)]
    pub fn ge(&mut self) -> GE_W {
        GE_W { w: self }
    }
    #[doc = "Bits 17:18 - GPOL"]
    #[inline(always)]
    pub fn gpol(&mut self) -> GPOL_W {
        GPOL_W { w: self }
    }
    #[doc = "Bits 19:23 - GNBREQ"]
    #[inline(always)]
    pub fn gnbreq(&mut self) -> GNBREQ_W {
        GNBREQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMAMUX request generator channel 1 configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmamux_rg1cr](index.html) module"]
pub struct DMAMUX_RG1CR_SPEC;
impl crate::RegisterSpec for DMAMUX_RG1CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmamux_rg1cr::R](R) reader structure"]
impl crate::Readable for DMAMUX_RG1CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmamux_rg1cr::W](W) writer structure"]
impl crate::Writable for DMAMUX_RG1CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMAMUX_RG1CR to value 0"]
impl crate::Resettable for DMAMUX_RG1CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
