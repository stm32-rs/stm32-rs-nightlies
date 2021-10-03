#[doc = "Register `SAI_ACR2` reader"]
pub struct R(crate::R<SAI_ACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAI_ACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAI_ACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAI_ACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAI_ACR2` writer"]
pub struct W(crate::W<SAI_ACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAI_ACR2_SPEC>;
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
impl From<crate::W<SAI_ACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAI_ACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTH` reader - FTH"]
pub struct FTH_R(crate::FieldReader<u8, u8>);
impl FTH_R {
    pub(crate) fn new(bits: u8) -> Self {
        FTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FTH_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FTH` writer - FTH"]
pub struct FTH_W<'a> {
    w: &'a mut W,
}
impl<'a> FTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `FFLUSH` writer - FFLUSH"]
pub struct FFLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FFLUSH_W<'a> {
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
#[doc = "Field `TRIS` reader - TRIS"]
pub struct TRIS_R(crate::FieldReader<bool, bool>);
impl TRIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TRIS` writer - TRIS"]
pub struct TRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIS_W<'a> {
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
#[doc = "Field `MUTE` reader - MUTE"]
pub struct MUTE_R(crate::FieldReader<bool, bool>);
impl MUTE_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTE` writer - MUTE"]
pub struct MUTE_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTE_W<'a> {
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
#[doc = "Field `MUTEVAL` reader - MUTEVAL"]
pub struct MUTEVAL_R(crate::FieldReader<bool, bool>);
impl MUTEVAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MUTEVAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTEVAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTEVAL` writer - MUTEVAL"]
pub struct MUTEVAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTEVAL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `MUTECNT` reader - MUTECNT"]
pub struct MUTECNT_R(crate::FieldReader<u8, u8>);
impl MUTECNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        MUTECNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MUTECNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MUTECNT` writer - MUTECNT"]
pub struct MUTECNT_W<'a> {
    w: &'a mut W,
}
impl<'a> MUTECNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 7)) | ((value as u32 & 0x3f) << 7);
        self.w
    }
}
#[doc = "Field `CPL` reader - CPL"]
pub struct CPL_R(crate::FieldReader<bool, bool>);
impl CPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPL` writer - CPL"]
pub struct CPL_W<'a> {
    w: &'a mut W,
}
impl<'a> CPL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `COMP` reader - COMP"]
pub struct COMP_R(crate::FieldReader<u8, u8>);
impl COMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMP` writer - COMP"]
pub struct COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FTH"]
    #[inline(always)]
    pub fn fth(&mut self) -> FTH_W {
        FTH_W { w: self }
    }
    #[doc = "Bit 3 - FFLUSH"]
    #[inline(always)]
    pub fn fflush(&mut self) -> FFLUSH_W {
        FFLUSH_W { w: self }
    }
    #[doc = "Bit 4 - TRIS"]
    #[inline(always)]
    pub fn tris(&mut self) -> TRIS_W {
        TRIS_W { w: self }
    }
    #[doc = "Bit 5 - MUTE"]
    #[inline(always)]
    pub fn mute(&mut self) -> MUTE_W {
        MUTE_W { w: self }
    }
    #[doc = "Bit 6 - MUTEVAL"]
    #[inline(always)]
    pub fn muteval(&mut self) -> MUTEVAL_W {
        MUTEVAL_W { w: self }
    }
    #[doc = "Bits 7:12 - MUTECNT"]
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MUTECNT_W {
        MUTECNT_W { w: self }
    }
    #[doc = "Bit 13 - CPL"]
    #[inline(always)]
    pub fn cpl(&mut self) -> CPL_W {
        CPL_W { w: self }
    }
    #[doc = "Bits 14:15 - COMP"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sai_acr2](index.html) module"]
pub struct SAI_ACR2_SPEC;
impl crate::RegisterSpec for SAI_ACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sai_acr2::R](R) reader structure"]
impl crate::Readable for SAI_ACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sai_acr2::W](W) writer structure"]
impl crate::Writable for SAI_ACR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SAI_ACR2 to value 0"]
impl crate::Resettable for SAI_ACR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
