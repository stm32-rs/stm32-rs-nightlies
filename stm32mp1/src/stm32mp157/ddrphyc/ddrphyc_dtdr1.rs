#[doc = "Register `DDRPHYC_DTDR1` reader"]
pub struct R(crate::R<DDRPHYC_DTDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTDR1` writer"]
pub struct W(crate::W<DDRPHYC_DTDR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTDR1_SPEC>;
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
impl From<crate::W<DDRPHYC_DTDR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTDR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTBYTE4` reader - DTBYTE4"]
pub struct DTBYTE4_R(crate::FieldReader<u8, u8>);
impl DTBYTE4_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE4` writer - DTBYTE4"]
pub struct DTBYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DTBYTE5` reader - DTBYTE5"]
pub struct DTBYTE5_R(crate::FieldReader<u8, u8>);
impl DTBYTE5_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE5` writer - DTBYTE5"]
pub struct DTBYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DTBYTE6` reader - DTBYTE6"]
pub struct DTBYTE6_R(crate::FieldReader<u8, u8>);
impl DTBYTE6_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE6` writer - DTBYTE6"]
pub struct DTBYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DTBYTE7` reader - DTBYTE7"]
pub struct DTBYTE7_R(crate::FieldReader<u8, u8>);
impl DTBYTE7_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE7` writer - DTBYTE7"]
pub struct DTBYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&self) -> DTBYTE4_R {
        DTBYTE4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&self) -> DTBYTE5_R {
        DTBYTE5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&self) -> DTBYTE6_R {
        DTBYTE6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&self) -> DTBYTE7_R {
        DTBYTE7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE4"]
    #[inline(always)]
    pub fn dtbyte4(&mut self) -> DTBYTE4_W {
        DTBYTE4_W { w: self }
    }
    #[doc = "Bits 8:15 - DTBYTE5"]
    #[inline(always)]
    pub fn dtbyte5(&mut self) -> DTBYTE5_W {
        DTBYTE5_W { w: self }
    }
    #[doc = "Bits 16:23 - DTBYTE6"]
    #[inline(always)]
    pub fn dtbyte6(&mut self) -> DTBYTE6_W {
        DTBYTE6_W { w: self }
    }
    #[doc = "Bits 24:31 - DTBYTE7"]
    #[inline(always)]
    pub fn dtbyte7(&mut self) -> DTBYTE7_W {
        DTBYTE7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTD register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr1](index.html) module"]
pub struct DDRPHYC_DTDR1_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtdr1::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr1::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR1 to value 0x7788_bb44"]
impl crate::Resettable for DDRPHYC_DTDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x7788_bb44
    }
}
