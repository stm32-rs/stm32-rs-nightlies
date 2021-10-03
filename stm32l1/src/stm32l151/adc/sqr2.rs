#[doc = "Register `SQR2` reader"]
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR2` writer"]
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ24` reader - 24th conversion in regular sequence"]
pub struct SQ24_R(crate::FieldReader<u8, u8>);
impl SQ24_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ24_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ24` writer - 24th conversion in regular sequence"]
pub struct SQ24_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ24_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `SQ23` reader - 23rd conversion in regular sequence"]
pub struct SQ23_R(crate::FieldReader<u8, u8>);
impl SQ23_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ23` writer - 23rd conversion in regular sequence"]
pub struct SQ23_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `SQ22` reader - 22nd conversion in regular sequence"]
pub struct SQ22_R(crate::FieldReader<u8, u8>);
impl SQ22_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ22_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ22` writer - 22nd conversion in regular sequence"]
pub struct SQ22_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ22_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `SQ21` reader - 21st conversion in regular sequence"]
pub struct SQ21_R(crate::FieldReader<u8, u8>);
impl SQ21_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ21` writer - 21st conversion in regular sequence"]
pub struct SQ21_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ21_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `SQ20` reader - 20th conversion in regular sequence"]
pub struct SQ20_R(crate::FieldReader<u8, u8>);
impl SQ20_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ20_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ20` writer - 20th conversion in regular sequence"]
pub struct SQ20_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `SQ19` reader - 19th conversion in regular sequence"]
pub struct SQ19_R(crate::FieldReader<u8, u8>);
impl SQ19_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ19_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ19` writer - 19th conversion in regular sequence"]
pub struct SQ19_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ19_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&self) -> SQ24_R {
        SQ24_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&self) -> SQ23_R {
        SQ23_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&self) -> SQ22_R {
        SQ22_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&self) -> SQ21_R {
        SQ21_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&self) -> SQ20_R {
        SQ20_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&self) -> SQ19_R {
        SQ19_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 24th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq24(&mut self) -> SQ24_W {
        SQ24_W { w: self }
    }
    #[doc = "Bits 20:24 - 23rd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq23(&mut self) -> SQ23_W {
        SQ23_W { w: self }
    }
    #[doc = "Bits 15:19 - 22nd conversion in regular sequence"]
    #[inline(always)]
    pub fn sq22(&mut self) -> SQ22_W {
        SQ22_W { w: self }
    }
    #[doc = "Bits 10:14 - 21st conversion in regular sequence"]
    #[inline(always)]
    pub fn sq21(&mut self) -> SQ21_W {
        SQ21_W { w: self }
    }
    #[doc = "Bits 5:9 - 20th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq20(&mut self) -> SQ20_W {
        SQ20_W { w: self }
    }
    #[doc = "Bits 0:4 - 19th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq19(&mut self) -> SQ19_W {
        SQ19_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr2](index.html) module"]
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr2::R](R) reader structure"]
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr2::W](W) writer structure"]
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR2 to value 0"]
impl crate::Resettable for SQR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
