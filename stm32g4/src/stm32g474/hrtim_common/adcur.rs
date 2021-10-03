#[doc = "Register `ADCUR` reader"]
pub struct R(crate::R<ADCUR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCUR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCUR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCUR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCUR` writer"]
pub struct W(crate::W<ADCUR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCUR_SPEC>;
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
impl From<crate::W<ADCUR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCUR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AD10USRC` reader - AD10USRC"]
pub struct AD10USRC_R(crate::FieldReader<u8, u8>);
impl AD10USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD10USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD10USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD10USRC` writer - AD10USRC"]
pub struct AD10USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD10USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `AD9USRC` reader - AD9USRC"]
pub struct AD9USRC_R(crate::FieldReader<u8, u8>);
impl AD9USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD9USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD9USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD9USRC` writer - AD9USRC"]
pub struct AD9USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD9USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `AD8USRC` reader - AD8USRC"]
pub struct AD8USRC_R(crate::FieldReader<u8, u8>);
impl AD8USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD8USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD8USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD8USRC` writer - AD8USRC"]
pub struct AD8USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD8USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `AD7USRC` reader - AD7USRC"]
pub struct AD7USRC_R(crate::FieldReader<u8, u8>);
impl AD7USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD7USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD7USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD7USRC` writer - AD7USRC"]
pub struct AD7USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD7USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `AD6USRC` reader - AD6USRC"]
pub struct AD6USRC_R(crate::FieldReader<u8, u8>);
impl AD6USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD6USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD6USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD6USRC` writer - AD6USRC"]
pub struct AD6USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD6USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `AD5USRC` reader - AD5USRC"]
pub struct AD5USRC_R(crate::FieldReader<u8, u8>);
impl AD5USRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        AD5USRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AD5USRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AD5USRC` writer - AD5USRC"]
pub struct AD5USRC_W<'a> {
    w: &'a mut W,
}
impl<'a> AD5USRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&self) -> AD10USRC_R {
        AD10USRC_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&self) -> AD9USRC_R {
        AD9USRC_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&self) -> AD8USRC_R {
        AD8USRC_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&self) -> AD7USRC_R {
        AD7USRC_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&self) -> AD6USRC_R {
        AD6USRC_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&self) -> AD5USRC_R {
        AD5USRC_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22 - AD10USRC"]
    #[inline(always)]
    pub fn ad10usrc(&mut self) -> AD10USRC_W {
        AD10USRC_W { w: self }
    }
    #[doc = "Bits 16:18 - AD9USRC"]
    #[inline(always)]
    pub fn ad9usrc(&mut self) -> AD9USRC_W {
        AD9USRC_W { w: self }
    }
    #[doc = "Bits 12:14 - AD8USRC"]
    #[inline(always)]
    pub fn ad8usrc(&mut self) -> AD8USRC_W {
        AD8USRC_W { w: self }
    }
    #[doc = "Bits 8:10 - AD7USRC"]
    #[inline(always)]
    pub fn ad7usrc(&mut self) -> AD7USRC_W {
        AD7USRC_W { w: self }
    }
    #[doc = "Bits 4:6 - AD6USRC"]
    #[inline(always)]
    pub fn ad6usrc(&mut self) -> AD6USRC_W {
        AD6USRC_W { w: self }
    }
    #[doc = "Bits 0:2 - AD5USRC"]
    #[inline(always)]
    pub fn ad5usrc(&mut self) -> AD5USRC_W {
        AD5USRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM ADC Trigger Update Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcur](index.html) module"]
pub struct ADCUR_SPEC;
impl crate::RegisterSpec for ADCUR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcur::R](R) reader structure"]
impl crate::Readable for ADCUR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcur::W](W) writer structure"]
impl crate::Writable for ADCUR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCUR to value 0"]
impl crate::Resettable for ADCUR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
