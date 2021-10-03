#[doc = "Register `PDMDLY` reader"]
pub struct R(crate::R<PDMDLY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDMDLY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDMDLY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDMDLY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDMDLY` writer"]
pub struct W(crate::W<PDMDLY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDMDLY_SPEC>;
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
impl From<crate::W<PDMDLY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDMDLY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLYM1L` reader - Delay line adjust for first microphone of pair 1"]
pub struct DLYM1L_R(crate::FieldReader<u8, u8>);
impl DLYM1L_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM1L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM1L` writer - Delay line adjust for first microphone of pair 1"]
pub struct DLYM1L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM1L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `DLYM1R` reader - Delay line adjust for second microphone of pair 1"]
pub struct DLYM1R_R(crate::FieldReader<u8, u8>);
impl DLYM1R_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM1R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM1R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM1R` writer - Delay line adjust for second microphone of pair 1"]
pub struct DLYM1R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM1R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DLYM2L` reader - Delay line for first microphone of pair 2"]
pub struct DLYM2L_R(crate::FieldReader<u8, u8>);
impl DLYM2L_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM2L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM2L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM2L` writer - Delay line for first microphone of pair 2"]
pub struct DLYM2L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM2L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `DLYM2R` reader - Delay line for second microphone of pair 2"]
pub struct DLYM2R_R(crate::FieldReader<u8, u8>);
impl DLYM2R_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM2R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM2R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM2R` writer - Delay line for second microphone of pair 2"]
pub struct DLYM2R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM2R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `DLYM3L` reader - Delay line for first microphone of pair 3"]
pub struct DLYM3L_R(crate::FieldReader<u8, u8>);
impl DLYM3L_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM3L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM3L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM3L` writer - Delay line for first microphone of pair 3"]
pub struct DLYM3L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM3L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `DLYM3R` reader - Delay line for second microphone of pair 3"]
pub struct DLYM3R_R(crate::FieldReader<u8, u8>);
impl DLYM3R_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM3R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM3R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM3R` writer - Delay line for second microphone of pair 3"]
pub struct DLYM3R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM3R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `DLYM4L` reader - Delay line for first microphone of pair 4"]
pub struct DLYM4L_R(crate::FieldReader<u8, u8>);
impl DLYM4L_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM4L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM4L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM4L` writer - Delay line for first microphone of pair 4"]
pub struct DLYM4L_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM4L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `DLYM4R` reader - Delay line for second microphone of pair 4"]
pub struct DLYM4R_R(crate::FieldReader<u8, u8>);
impl DLYM4R_R {
    pub(crate) fn new(bits: u8) -> Self {
        DLYM4R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYM4R_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYM4R` writer - Delay line for second microphone of pair 4"]
pub struct DLYM4R_W<'a> {
    w: &'a mut W,
}
impl<'a> DLYM4R_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&self) -> DLYM1L_R {
        DLYM1L_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&self) -> DLYM1R_R {
        DLYM1R_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&self) -> DLYM2L_R {
        DLYM2L_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&self) -> DLYM2R_R {
        DLYM2R_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&self) -> DLYM3L_R {
        DLYM3L_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&self) -> DLYM3R_R {
        DLYM3R_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&self) -> DLYM4L_R {
        DLYM4L_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&self) -> DLYM4R_R {
        DLYM4R_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&mut self) -> DLYM1L_W {
        DLYM1L_W { w: self }
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&mut self) -> DLYM1R_W {
        DLYM1R_W { w: self }
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&mut self) -> DLYM2L_W {
        DLYM2L_W { w: self }
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&mut self) -> DLYM2R_W {
        DLYM2R_W { w: self }
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&mut self) -> DLYM3L_W {
        DLYM3L_W { w: self }
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&mut self) -> DLYM3R_W {
        DLYM3R_W { w: self }
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&mut self) -> DLYM4L_W {
        DLYM4L_W { w: self }
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&mut self) -> DLYM4R_W {
        DLYM4R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PDM delay register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdmdly](index.html) module"]
pub struct PDMDLY_SPEC;
impl crate::RegisterSpec for PDMDLY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdmdly::R](R) reader structure"]
impl crate::Readable for PDMDLY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdmdly::W](W) writer structure"]
impl crate::Writable for PDMDLY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PDMDLY to value 0"]
impl crate::Resettable for PDMDLY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
