#[doc = "Register `OTR` reader"]
pub struct R(crate::R<OTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTR` writer"]
pub struct W(crate::W<OTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTR_SPEC>;
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
impl From<crate::W<OTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OT_USER` reader - Select user or factory trimming value"]
pub struct OT_USER_R(crate::FieldReader<bool, bool>);
impl OT_USER_R {
    pub(crate) fn new(bits: bool) -> Self {
        OT_USER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OT_USER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OT_USER` writer - Select user or factory trimming value"]
pub struct OT_USER_W<'a> {
    w: &'a mut W,
}
impl<'a> OT_USER_W<'a> {
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
#[doc = "Field `AO3_OPT_OFFSET_TRIM` reader - OPAMP3, 10-bit offset trim value for normal mode"]
pub struct AO3_OPT_OFFSET_TRIM_R(crate::FieldReader<u16, u16>);
impl AO3_OPT_OFFSET_TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        AO3_OPT_OFFSET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AO3_OPT_OFFSET_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AO3_OPT_OFFSET_TRIM` writer - OPAMP3, 10-bit offset trim value for normal mode"]
pub struct AO3_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO3_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 20)) | ((value as u32 & 0x03ff) << 20);
        self.w
    }
}
#[doc = "Field `AO2_OPT_OFFSET_TRIM` reader - OPAMP2, 10-bit offset trim value for normal mode"]
pub struct AO2_OPT_OFFSET_TRIM_R(crate::FieldReader<u16, u16>);
impl AO2_OPT_OFFSET_TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        AO2_OPT_OFFSET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AO2_OPT_OFFSET_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AO2_OPT_OFFSET_TRIM` writer - OPAMP2, 10-bit offset trim value for normal mode"]
pub struct AO2_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO2_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 10)) | ((value as u32 & 0x03ff) << 10);
        self.w
    }
}
#[doc = "Field `AO1_OPT_OFFSET_TRIM` reader - OPAMP1, 10-bit offset trim value for normal mode"]
pub struct AO1_OPT_OFFSET_TRIM_R(crate::FieldReader<u16, u16>);
impl AO1_OPT_OFFSET_TRIM_R {
    pub(crate) fn new(bits: u16) -> Self {
        AO1_OPT_OFFSET_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AO1_OPT_OFFSET_TRIM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AO1_OPT_OFFSET_TRIM` writer - OPAMP1, 10-bit offset trim value for normal mode"]
pub struct AO1_OPT_OFFSET_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> AO1_OPT_OFFSET_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&self) -> OT_USER_R {
        OT_USER_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&self) -> AO3_OPT_OFFSET_TRIM_R {
        AO3_OPT_OFFSET_TRIM_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&self) -> AO2_OPT_OFFSET_TRIM_R {
        AO2_OPT_OFFSET_TRIM_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&self) -> AO1_OPT_OFFSET_TRIM_R {
        AO1_OPT_OFFSET_TRIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 31 - Select user or factory trimming value"]
    #[inline(always)]
    pub fn ot_user(&mut self) -> OT_USER_W {
        OT_USER_W { w: self }
    }
    #[doc = "Bits 20:29 - OPAMP3, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao3_opt_offset_trim(&mut self) -> AO3_OPT_OFFSET_TRIM_W {
        AO3_OPT_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Bits 10:19 - OPAMP2, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao2_opt_offset_trim(&mut self) -> AO2_OPT_OFFSET_TRIM_W {
        AO2_OPT_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Bits 0:9 - OPAMP1, 10-bit offset trim value for normal mode"]
    #[inline(always)]
    pub fn ao1_opt_offset_trim(&mut self) -> AO1_OPT_OFFSET_TRIM_W {
        AO1_OPT_OFFSET_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "offset trimming register for normal mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otr](index.html) module"]
pub struct OTR_SPEC;
impl crate::RegisterSpec for OTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otr::R](R) reader structure"]
impl crate::Readable for OTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otr::W](W) writer structure"]
impl crate::Writable for OTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OTR to value 0"]
impl crate::Resettable for OTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
