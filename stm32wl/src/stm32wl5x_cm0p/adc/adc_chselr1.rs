#[doc = "Register `ADC_CHSELR1` reader"]
pub struct R(crate::R<ADC_CHSELR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_CHSELR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_CHSELR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_CHSELR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_CHSELR1` writer"]
pub struct W(crate::W<ADC_CHSELR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_CHSELR1_SPEC>;
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
impl From<crate::W<ADC_CHSELR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_CHSELR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SQ1` reader - SQ1"]
pub struct SQ1_R(crate::FieldReader<u8, u8>);
impl SQ1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ1` writer - SQ1"]
pub struct SQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SQ2` reader - SQ2"]
pub struct SQ2_R(crate::FieldReader<u8, u8>);
impl SQ2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ2` writer - SQ2"]
pub struct SQ2_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `SQ3` reader - SQ3"]
pub struct SQ3_R(crate::FieldReader<u8, u8>);
impl SQ3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ3` writer - SQ3"]
pub struct SQ3_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `SQ4` reader - SQ4"]
pub struct SQ4_R(crate::FieldReader<u8, u8>);
impl SQ4_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ4` writer - SQ4"]
pub struct SQ4_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `SQ5` reader - SQ5"]
pub struct SQ5_R(crate::FieldReader<u8, u8>);
impl SQ5_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ5` writer - SQ5"]
pub struct SQ5_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `SQ6` reader - SQ6"]
pub struct SQ6_R(crate::FieldReader<u8, u8>);
impl SQ6_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ6` writer - SQ6"]
pub struct SQ6_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `SQ7` reader - SQ7"]
pub struct SQ7_R(crate::FieldReader<u8, u8>);
impl SQ7_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ7` writer - SQ7"]
pub struct SQ7_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `SQ8` reader - SQ8"]
pub struct SQ8_R(crate::FieldReader<u8, u8>);
impl SQ8_R {
    pub(crate) fn new(bits: u8) -> Self {
        SQ8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SQ8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SQ8` writer - SQ8"]
pub struct SQ8_W<'a> {
    w: &'a mut W,
}
impl<'a> SQ8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W {
        SQ1_W { w: self }
    }
    #[doc = "Bits 4:7 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W {
        SQ2_W { w: self }
    }
    #[doc = "Bits 8:11 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W {
        SQ3_W { w: self }
    }
    #[doc = "Bits 12:15 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W {
        SQ4_W { w: self }
    }
    #[doc = "Bits 16:19 - SQ5"]
    #[inline(always)]
    pub fn sq5(&mut self) -> SQ5_W {
        SQ5_W { w: self }
    }
    #[doc = "Bits 20:23 - SQ6"]
    #[inline(always)]
    pub fn sq6(&mut self) -> SQ6_W {
        SQ6_W { w: self }
    }
    #[doc = "Bits 24:27 - SQ7"]
    #[inline(always)]
    pub fn sq7(&mut self) -> SQ7_W {
        SQ7_W { w: self }
    }
    #[doc = "Bits 28:31 - SQ8"]
    #[inline(always)]
    pub fn sq8(&mut self) -> SQ8_W {
        SQ8_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel selection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_chselr1](index.html) module"]
pub struct ADC_CHSELR1_SPEC;
impl crate::RegisterSpec for ADC_CHSELR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_chselr1::R](R) reader structure"]
impl crate::Readable for ADC_CHSELR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_chselr1::W](W) writer structure"]
impl crate::Writable for ADC_CHSELR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_CHSELR1 to value 0"]
impl crate::Resettable for ADC_CHSELR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
