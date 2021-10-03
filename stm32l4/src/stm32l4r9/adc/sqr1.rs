#[doc = "Register `SQR1` reader"]
pub struct R(crate::R<SQR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SQR1` writer"]
pub struct W(crate::W<SQR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR1_SPEC>;
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
impl From<crate::W<SQR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR1_SPEC>) -> Self {
        W(writer)
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
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
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
        self.w.bits = (self.w.bits & !(0x1f << 18)) | ((value as u32 & 0x1f) << 18);
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
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
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
        self.w.bits = (self.w.bits & !(0x1f << 6)) | ((value as u32 & 0x1f) << 6);
        self.w
    }
}
#[doc = "Field `L` reader - Regular channel sequence length"]
pub struct L_R(crate::FieldReader<u8, u8>);
impl L_R {
    pub(crate) fn new(bits: u8) -> Self {
        L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for L_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `L` writer - Regular channel sequence length"]
pub struct L_W<'a> {
    w: &'a mut W,
}
impl<'a> L_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&self) -> SQ4_R {
        SQ4_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&self) -> SQ3_R {
        SQ3_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&self) -> SQ2_R {
        SQ2_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&self) -> SQ1_R {
        SQ1_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&self) -> L_R {
        L_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:28 - SQ4"]
    #[inline(always)]
    pub fn sq4(&mut self) -> SQ4_W {
        SQ4_W { w: self }
    }
    #[doc = "Bits 18:22 - SQ3"]
    #[inline(always)]
    pub fn sq3(&mut self) -> SQ3_W {
        SQ3_W { w: self }
    }
    #[doc = "Bits 12:16 - SQ2"]
    #[inline(always)]
    pub fn sq2(&mut self) -> SQ2_W {
        SQ2_W { w: self }
    }
    #[doc = "Bits 6:10 - SQ1"]
    #[inline(always)]
    pub fn sq1(&mut self) -> SQ1_W {
        SQ1_W { w: self }
    }
    #[doc = "Bits 0:3 - Regular channel sequence length"]
    #[inline(always)]
    pub fn l(&mut self) -> L_W {
        L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "regular sequence register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sqr1](index.html) module"]
pub struct SQR1_SPEC;
impl crate::RegisterSpec for SQR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sqr1::R](R) reader structure"]
impl crate::Readable for SQR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sqr1::W](W) writer structure"]
impl crate::Writable for SQR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SQR1 to value 0"]
impl crate::Resettable for SQR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
