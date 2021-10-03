#[doc = "Register `DAC_STMODR` reader"]
pub struct R(crate::R<DAC_STMODR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAC_STMODR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAC_STMODR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAC_STMODR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DAC_STMODR` writer"]
pub struct W(crate::W<DAC_STMODR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DAC_STMODR_SPEC>;
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
impl From<crate::W<DAC_STMODR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DAC_STMODR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STRSTTRIGSEL1` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub struct STRSTTRIGSEL1_R(crate::FieldReader<u8, u8>);
impl STRSTTRIGSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        STRSTTRIGSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRSTTRIGSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRSTTRIGSEL1` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub struct STRSTTRIGSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTTRIGSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `STINCTRIGSEL1` reader - DAC Channel 1 Sawtooth Increment trigger selection"]
pub struct STINCTRIGSEL1_R(crate::FieldReader<u8, u8>);
impl STINCTRIGSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        STINCTRIGSEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STINCTRIGSEL1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STINCTRIGSEL1` writer - DAC Channel 1 Sawtooth Increment trigger selection"]
pub struct STINCTRIGSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCTRIGSEL1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `STRSTTRIGSEL2` reader - DAC Channel 1 Sawtooth Reset trigger selection"]
pub struct STRSTTRIGSEL2_R(crate::FieldReader<u8, u8>);
impl STRSTTRIGSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        STRSTTRIGSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STRSTTRIGSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STRSTTRIGSEL2` writer - DAC Channel 1 Sawtooth Reset trigger selection"]
pub struct STRSTTRIGSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STRSTTRIGSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `STINCTRIGSEL2` reader - DAC Channel 2 Sawtooth Increment trigger selection"]
pub struct STINCTRIGSEL2_R(crate::FieldReader<u8, u8>);
impl STINCTRIGSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        STINCTRIGSEL2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STINCTRIGSEL2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STINCTRIGSEL2` writer - DAC Channel 2 Sawtooth Increment trigger selection"]
pub struct STINCTRIGSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> STINCTRIGSEL2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> STRSTTRIGSEL1_R {
        STRSTTRIGSEL1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> STINCTRIGSEL1_R {
        STINCTRIGSEL1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> STRSTTRIGSEL2_R {
        STRSTTRIGSEL2_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> STINCTRIGSEL2_R {
        STINCTRIGSEL2_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel1(&mut self) -> STRSTTRIGSEL1_W {
        STRSTTRIGSEL1_W { w: self }
    }
    #[doc = "Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel1(&mut self) -> STINCTRIGSEL1_W {
        STINCTRIGSEL1_W { w: self }
    }
    #[doc = "Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection"]
    #[inline(always)]
    pub fn strsttrigsel2(&mut self) -> STRSTTRIGSEL2_W {
        STRSTTRIGSEL2_W { w: self }
    }
    #[doc = "Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection"]
    #[inline(always)]
    pub fn stinctrigsel2(&mut self) -> STINCTRIGSEL2_W {
        STINCTRIGSEL2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sawtooth Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dac_stmodr](index.html) module"]
pub struct DAC_STMODR_SPEC;
impl crate::RegisterSpec for DAC_STMODR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dac_stmodr::R](R) reader structure"]
impl crate::Readable for DAC_STMODR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dac_stmodr::W](W) writer structure"]
impl crate::Writable for DAC_STMODR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DAC_STMODR to value 0"]
impl crate::Resettable for DAC_STMODR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
