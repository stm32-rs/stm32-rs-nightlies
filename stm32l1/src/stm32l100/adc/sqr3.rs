#[doc = "Register `SQR3` reader"]
pub struct R(crate::R<SQR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR3` writer"]
pub struct W(crate::W<SQR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR3_SPEC>;
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
impl From<crate::W<SQR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ18` reader - 18th conversion in regular sequence"]
pub struct SQ18_R(crate::FieldReader<u8, u8>);
impl SQ18_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ18_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ18` writer - 18th conversion in regular sequence"]
pub struct SQ18_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ18_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 25)) | ((value as u32 & 0x1f) << 25);
        self.w
    }
}
#[doc = "Field `SQ17` reader - 17th conversion in regular sequence"]
pub struct SQ17_R(crate::FieldReader<u8, u8>);
impl SQ17_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ17_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ17` writer - 17th conversion in regular sequence"]
pub struct SQ17_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 20)) | ((value as u32 & 0x1f) << 20);
        self.w
    }
}
#[doc = "Field `SQ16` reader - 16th conversion in regular sequence"]
pub struct SQ16_R(crate::FieldReader<u8, u8>);
impl SQ16_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ16_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ16` writer - 16th conversion in regular sequence"]
pub struct SQ16_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ16_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 15)) | ((value as u32 & 0x1f) << 15);
        self.w
    }
}
#[doc = "Field `SQ15` reader - 15th conversion in regular sequence"]
pub struct SQ15_R(crate::FieldReader<u8, u8>);
impl SQ15_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ15` writer - 15th conversion in regular sequence"]
pub struct SQ15_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `SQ14` reader - 14th conversion in regular sequence"]
pub struct SQ14_R(crate::FieldReader<u8, u8>);
impl SQ14_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ14` writer - 14th conversion in regular sequence"]
pub struct SQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `SQ13` reader - 13th conversion in regular sequence"]
pub struct SQ13_R(crate::FieldReader<u8, u8>);
impl SQ13_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ13_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ13` writer - 13th conversion in regular sequence"]
pub struct SQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq18(&self) -> SQ18_R {
        SQ18_R::new(((self.bits >> 25) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq17(&self) -> SQ17_R {
        SQ17_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&self) -> SQ16_R {
        SQ16_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&self) -> SQ15_R {
        SQ15_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 25:29 - 18th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq18(&mut self) -> SQ18_W {
        SQ18_W { w: self }
    }
    #[doc = "Bits 20:24 - 17th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq17(&mut self) -> SQ17_W {
        SQ17_W { w: self }
    }
    #[doc = "Bits 15:19 - 16th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq16(&mut self) -> SQ16_W {
        SQ16_W { w: self }
    }
    #[doc = "Bits 10:14 - 15th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq15(&mut self) -> SQ15_W {
        SQ15_W { w: self }
    }
    #[doc = "Bits 5:9 - 14th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W {
        SQ14_W { w: self }
    }
    #[doc = "Bits 0:4 - 13th conversion in regular sequence"]
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W {
        SQ13_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr3](index.html) module"]
pub struct SQR3_SPEC;
impl crate::RegisterSpec for SQR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr3::R](R) reader structure"]
impl crate::Readable for SQR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr3::W](W) writer structure"]
impl crate::Writable for SQR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR3 to value 0"]
impl crate::Resettable for SQR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
