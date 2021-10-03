#[doc = "Register `ADC_SMPR1` reader"]
pub struct R(crate::R<ADC_SMPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC_SMPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC_SMPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC_SMPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADC_SMPR1` writer"]
pub struct W(crate::W<ADC_SMPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADC_SMPR1_SPEC>;
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
impl From<crate::W<ADC_SMPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADC_SMPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP0` reader - SMP0"]
pub struct SMP0_R(crate::FieldReader<u8, u8>);
impl SMP0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP0` writer - SMP0"]
pub struct SMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `SMP1` reader - SMP1"]
pub struct SMP1_R(crate::FieldReader<u8, u8>);
impl SMP1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP1` writer - SMP1"]
pub struct SMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `SMP2` reader - SMP2"]
pub struct SMP2_R(crate::FieldReader<u8, u8>);
impl SMP2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP2` writer - SMP2"]
pub struct SMP2_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | ((value as u32 & 0x07) << 6);
        self.w
    }
}
#[doc = "Field `SMP3` reader - SMP3"]
pub struct SMP3_R(crate::FieldReader<u8, u8>);
impl SMP3_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP3` writer - SMP3"]
pub struct SMP3_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 9)) | ((value as u32 & 0x07) << 9);
        self.w
    }
}
#[doc = "Field `SMP4` reader - SMP4"]
pub struct SMP4_R(crate::FieldReader<u8, u8>);
impl SMP4_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP4` writer - SMP4"]
pub struct SMP4_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `SMP5` reader - SMP5"]
pub struct SMP5_R(crate::FieldReader<u8, u8>);
impl SMP5_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP5` writer - SMP5"]
pub struct SMP5_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 15)) | ((value as u32 & 0x07) << 15);
        self.w
    }
}
#[doc = "Field `SMP6` reader - SMP6"]
pub struct SMP6_R(crate::FieldReader<u8, u8>);
impl SMP6_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP6` writer - SMP6"]
pub struct SMP6_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 18)) | ((value as u32 & 0x07) << 18);
        self.w
    }
}
#[doc = "Field `SMP7` reader - SMP7"]
pub struct SMP7_R(crate::FieldReader<u8, u8>);
impl SMP7_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP7` writer - SMP7"]
pub struct SMP7_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 21)) | ((value as u32 & 0x07) << 21);
        self.w
    }
}
#[doc = "Field `SMP8` reader - SMP8"]
pub struct SMP8_R(crate::FieldReader<u8, u8>);
impl SMP8_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP8_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP8` writer - SMP8"]
pub struct SMP8_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP8_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `SMP9` reader - SMP9"]
pub struct SMP9_R(crate::FieldReader<u8, u8>);
impl SMP9_R {
    pub(crate) fn new(bits: u8) -> Self {
        SMP9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMP9_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMP9` writer - SMP9"]
pub struct SMP9_W<'a> {
    w: &'a mut W,
}
impl<'a> SMP9_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | ((value as u32 & 0x07) << 27);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SMP0"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 0x07) as u8)
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 0x07) as u8)
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP0"]
    #[inline(always)]
    pub fn smp0(&mut self) -> SMP0_W {
        SMP0_W { w: self }
    }
    #[doc = "Bits 3:5 - SMP1"]
    #[inline(always)]
    pub fn smp1(&mut self) -> SMP1_W {
        SMP1_W { w: self }
    }
    #[doc = "Bits 6:8 - SMP2"]
    #[inline(always)]
    pub fn smp2(&mut self) -> SMP2_W {
        SMP2_W { w: self }
    }
    #[doc = "Bits 9:11 - SMP3"]
    #[inline(always)]
    pub fn smp3(&mut self) -> SMP3_W {
        SMP3_W { w: self }
    }
    #[doc = "Bits 12:14 - SMP4"]
    #[inline(always)]
    pub fn smp4(&mut self) -> SMP4_W {
        SMP4_W { w: self }
    }
    #[doc = "Bits 15:17 - SMP5"]
    #[inline(always)]
    pub fn smp5(&mut self) -> SMP5_W {
        SMP5_W { w: self }
    }
    #[doc = "Bits 18:20 - SMP6"]
    #[inline(always)]
    pub fn smp6(&mut self) -> SMP6_W {
        SMP6_W { w: self }
    }
    #[doc = "Bits 21:23 - SMP7"]
    #[inline(always)]
    pub fn smp7(&mut self) -> SMP7_W {
        SMP7_W { w: self }
    }
    #[doc = "Bits 24:26 - SMP8"]
    #[inline(always)]
    pub fn smp8(&mut self) -> SMP8_W {
        SMP8_W { w: self }
    }
    #[doc = "Bits 27:29 - SMP9"]
    #[inline(always)]
    pub fn smp9(&mut self) -> SMP9_W {
        SMP9_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC sample time register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc_smpr1](index.html) module"]
pub struct ADC_SMPR1_SPEC;
impl crate::RegisterSpec for ADC_SMPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc_smpr1::R](R) reader structure"]
impl crate::Readable for ADC_SMPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adc_smpr1::W](W) writer structure"]
impl crate::Writable for ADC_SMPR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADC_SMPR1 to value 0"]
impl crate::Resettable for ADC_SMPR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
