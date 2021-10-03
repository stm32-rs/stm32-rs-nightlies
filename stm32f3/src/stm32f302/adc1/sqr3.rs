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
#[doc = "Field `SQ14` reader - SQ14"]
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
#[doc = "Field `SQ14` writer - SQ14"]
pub struct SQ14_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Field `SQ13` reader - SQ13"]
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
#[doc = "Field `SQ13` writer - SQ13"]
pub struct SQ13_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ13_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
        self.w
    }
}
#[doc = "Field `SQ12` reader - SQ12"]
pub struct SQ12_R(crate::FieldReader<u8, u8>);
impl SQ12_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ12_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ12` writer - SQ12"]
pub struct SQ12_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ12_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
#[doc = "Field `SQ11` reader - SQ11"]
pub struct SQ11_R(crate::FieldReader<u8, u8>);
impl SQ11_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ11` writer - SQ11"]
pub struct SQ11_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ11_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `SQ10` reader - SQ10"]
pub struct SQ10_R(crate::FieldReader<u8, u8>);
impl SQ10_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ10_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ10` writer - SQ10"]
pub struct SQ10_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - SQ14"]
    #[inline(always)]
    pub fn sq14(&self) -> SQ14_R {
        SQ14_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ13"]
    #[inline(always)]
    pub fn sq13(&self) -> SQ13_R {
        SQ13_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ12"]
    #[inline(always)]
    pub fn sq12(&self) -> SQ12_R {
        SQ12_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ11"]
    #[inline(always)]
    pub fn sq11(&self) -> SQ11_R {
        SQ11_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - SQ10"]
    #[inline(always)]
    pub fn sq10(&self) -> SQ10_R {
        SQ10_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - SQ14"]
    #[inline(always)]
    pub fn sq14(&mut self) -> SQ14_W {
        SQ14_W { w: self }
    }
    #[doc = "Bits 18:22 - SQ13"]
    #[inline(always)]
    pub fn sq13(&mut self) -> SQ13_W {
        SQ13_W { w: self }
    }
    #[doc = "Bits 12:16 - SQ12"]
    #[inline(always)]
    pub fn sq12(&mut self) -> SQ12_W {
        SQ12_W { w: self }
    }
    #[doc = "Bits 6:10 - SQ11"]
    #[inline(always)]
    pub fn sq11(&mut self) -> SQ11_W {
        SQ11_W { w: self }
    }
    #[doc = "Bits 0:4 - SQ10"]
    #[inline(always)]
    pub fn sq10(&mut self) -> SQ10_W {
        SQ10_W { w: self }
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
