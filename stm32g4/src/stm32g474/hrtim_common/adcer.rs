#[doc = "Register `ADCER` reader"]
pub struct R(crate::R<ADCER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCER` writer"]
pub struct W(crate::W<ADCER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCER_SPEC>;
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
impl From<crate::W<ADCER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC10TRG` reader - ADC10TRG"]
pub struct ADC10TRG_R(crate::FieldReader<u8, u8>);
impl ADC10TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC10TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC10TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC10TRG` writer - ADC10TRG"]
pub struct ADC10TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC10TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 26)) | ((value as u32 & 0x1f) << 26);
        self.w
    }
}
#[doc = "Field `ADC9TRG` reader - ADC9TRG"]
pub struct ADC9TRG_R(crate::FieldReader<u8, u8>);
impl ADC9TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC9TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC9TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC9TRG` writer - ADC9TRG"]
pub struct ADC9TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC9TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 21)) | ((value as u32 & 0x1f) << 21);
        self.w
    }
}
#[doc = "Field `ADC8TRG` reader - ADC8TRG"]
pub struct ADC8TRG_R(crate::FieldReader<u8, u8>);
impl ADC8TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC8TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC8TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC8TRG` writer - ADC8TRG"]
pub struct ADC8TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC8TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `ADC7TRG` reader - ADC7TRG"]
pub struct ADC7TRG_R(crate::FieldReader<u8, u8>);
impl ADC7TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC7TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC7TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC7TRG` writer - ADC7TRG"]
pub struct ADC7TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC7TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 10)) | ((value as u32 & 0x1f) << 10);
        self.w
    }
}
#[doc = "Field `ADC6TRG` reader - ADC6TRG"]
pub struct ADC6TRG_R(crate::FieldReader<u8, u8>);
impl ADC6TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC6TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC6TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC6TRG` writer - ADC6TRG"]
pub struct ADC6TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC6TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 5)) | ((value as u32 & 0x1f) << 5);
        self.w
    }
}
#[doc = "Field `ADC5TRG` reader - ADC5TRG"]
pub struct ADC5TRG_R(crate::FieldReader<u8, u8>);
impl ADC5TRG_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC5TRG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC5TRG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC5TRG` writer - ADC5TRG"]
pub struct ADC5TRG_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC5TRG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&self) -> ADC10TRG_R {
        ADC10TRG_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&self) -> ADC9TRG_R {
        ADC9TRG_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&self) -> ADC8TRG_R {
        ADC8TRG_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&self) -> ADC7TRG_R {
        ADC7TRG_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&self) -> ADC6TRG_R {
        ADC6TRG_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&self) -> ADC5TRG_R {
        ADC5TRG_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 26:30 - ADC10TRG"]
    #[inline(always)]
    pub fn adc10trg(&mut self) -> ADC10TRG_W {
        ADC10TRG_W { w: self }
    }
    #[doc = "Bits 21:25 - ADC9TRG"]
    #[inline(always)]
    pub fn adc9trg(&mut self) -> ADC9TRG_W {
        ADC9TRG_W { w: self }
    }
    #[doc = "Bits 16:20 - ADC8TRG"]
    #[inline(always)]
    pub fn adc8trg(&mut self) -> ADC8TRG_W {
        ADC8TRG_W { w: self }
    }
    #[doc = "Bits 10:14 - ADC7TRG"]
    #[inline(always)]
    pub fn adc7trg(&mut self) -> ADC7TRG_W {
        ADC7TRG_W { w: self }
    }
    #[doc = "Bits 5:9 - ADC6TRG"]
    #[inline(always)]
    pub fn adc6trg(&mut self) -> ADC6TRG_W {
        ADC6TRG_W { w: self }
    }
    #[doc = "Bits 0:4 - ADC5TRG"]
    #[inline(always)]
    pub fn adc5trg(&mut self) -> ADC5TRG_W {
        ADC5TRG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HRTIM ADC Extended Trigger Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcer](index.html) module"]
pub struct ADCER_SPEC;
impl crate::RegisterSpec for ADCER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcer::R](R) reader structure"]
impl crate::Readable for ADCER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcer::W](W) writer structure"]
impl crate::Writable for ADCER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCER to value 0"]
impl crate::Resettable for ADCER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
