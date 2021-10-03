#[doc = "Register `ECR` reader"]
pub struct R(crate::R<ECR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ECR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ECR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ECR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ECR` writer"]
pub struct W(crate::W<ECR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ECR_SPEC>;
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
impl From<crate::W<ECR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ECR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IE` reader - Index Enable"]
pub struct IE_R(crate::FieldReader<bool, bool>);
impl IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IE` writer - Index Enable"]
pub struct IE_W<'a> {
    w: &'a mut W,
}
impl<'a> IE_W<'a> {
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
#[doc = "Field `IDIR` reader - Index Direction"]
pub struct IDIR_R(crate::FieldReader<u8, u8>);
impl IDIR_R {
    pub(crate) fn new(bits: u8) -> Self {
        IDIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IDIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IDIR` writer - Index Direction"]
pub struct IDIR_W<'a> {
    w: &'a mut W,
}
impl<'a> IDIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `IBLK` reader - Index Blanking"]
pub struct IBLK_R(crate::FieldReader<u8, u8>);
impl IBLK_R {
    pub(crate) fn new(bits: u8) -> Self {
        IBLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IBLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IBLK` writer - Index Blanking"]
pub struct IBLK_W<'a> {
    w: &'a mut W,
}
impl<'a> IBLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | ((value as u32 & 0x03) << 3);
        self.w
    }
}
#[doc = "Field `FIDX` reader - First Index"]
pub struct FIDX_R(crate::FieldReader<bool, bool>);
impl FIDX_R {
    pub(crate) fn new(bits: bool) -> Self {
        FIDX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIDX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIDX` writer - First Index"]
pub struct FIDX_W<'a> {
    w: &'a mut W,
}
impl<'a> FIDX_W<'a> {
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
#[doc = "Field `IPOS` reader - Index Positioning"]
pub struct IPOS_R(crate::FieldReader<u8, u8>);
impl IPOS_R {
    pub(crate) fn new(bits: u8) -> Self {
        IPOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPOS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPOS` writer - Index Positioning"]
pub struct IPOS_W<'a> {
    w: &'a mut W,
}
impl<'a> IPOS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `PW` reader - Pulse width"]
pub struct PW_R(crate::FieldReader<u8, u8>);
impl PW_R {
    pub(crate) fn new(bits: u8) -> Self {
        PW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PW` writer - Pulse width"]
pub struct PW_W<'a> {
    w: &'a mut W,
}
impl<'a> PW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `PWPRSC` reader - Pulse Width prescaler"]
pub struct PWPRSC_R(crate::FieldReader<u8, u8>);
impl PWPRSC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PWPRSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWPRSC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWPRSC` writer - Pulse Width prescaler"]
pub struct PWPRSC_W<'a> {
    w: &'a mut W,
}
impl<'a> PWPRSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&self) -> IDIR_R {
        IDIR_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&self) -> IBLK_R {
        IBLK_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FIDX_R {
        FIDX_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&self) -> IPOS_R {
        IPOS_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&self) -> PW_R {
        PW_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&self) -> PWPRSC_R {
        PWPRSC_R::new(((self.bits >> 24) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Index Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W { w: self }
    }
    #[doc = "Bits 1:2 - Index Direction"]
    #[inline(always)]
    pub fn idir(&mut self) -> IDIR_W {
        IDIR_W { w: self }
    }
    #[doc = "Bits 3:4 - Index Blanking"]
    #[inline(always)]
    pub fn iblk(&mut self) -> IBLK_W {
        IBLK_W { w: self }
    }
    #[doc = "Bit 5 - First Index"]
    #[inline(always)]
    pub fn fidx(&mut self) -> FIDX_W {
        FIDX_W { w: self }
    }
    #[doc = "Bits 6:7 - Index Positioning"]
    #[inline(always)]
    pub fn ipos(&mut self) -> IPOS_W {
        IPOS_W { w: self }
    }
    #[doc = "Bits 16:23 - Pulse width"]
    #[inline(always)]
    pub fn pw(&mut self) -> PW_W {
        PW_W { w: self }
    }
    #[doc = "Bits 24:26 - Pulse Width prescaler"]
    #[inline(always)]
    pub fn pwprsc(&mut self) -> PWPRSC_W {
        PWPRSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ecr](index.html) module"]
pub struct ECR_SPEC;
impl crate::RegisterSpec for ECR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ecr::R](R) reader structure"]
impl crate::Readable for ECR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ecr::W](W) writer structure"]
impl crate::Writable for ECR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ECR to value 0"]
impl crate::Resettable for ECR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
