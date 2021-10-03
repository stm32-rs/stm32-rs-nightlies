#[doc = "Register `DDRPHYC_DX3DQSTR` reader"]
pub struct R(crate::R<DDRPHYC_DX3DQSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DX3DQSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DX3DQSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DX3DQSTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DX3DQSTR` writer"]
pub struct W(crate::W<DDRPHYC_DX3DQSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DX3DQSTR_SPEC>;
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
impl From<crate::W<DDRPHYC_DX3DQSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DX3DQSTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `R0DGSL` reader - R0DGSL"]
pub struct R0DGSL_R(crate::FieldReader<u8, u8>);
impl R0DGSL_R {
    pub(crate) fn new(bits: u8) -> Self {
        R0DGSL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0DGSL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0DGSL` writer - R0DGSL"]
pub struct R0DGSL_W<'a> {
    w: &'a mut W,
}
impl<'a> R0DGSL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `R0DGPS` reader - R0DGPS"]
pub struct R0DGPS_R(crate::FieldReader<u8, u8>);
impl R0DGPS_R {
    pub(crate) fn new(bits: u8) -> Self {
        R0DGPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for R0DGPS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `R0DGPS` writer - R0DGPS"]
pub struct R0DGPS_W<'a> {
    w: &'a mut W,
}
impl<'a> R0DGPS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `DQSDLY` reader - DQSDLY"]
pub struct DQSDLY_R(crate::FieldReader<u8, u8>);
impl DQSDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSDLY` writer - DQSDLY"]
pub struct DQSDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `DQSNDLY` reader - DQSNDLY"]
pub struct DQSNDLY_R(crate::FieldReader<u8, u8>);
impl DQSNDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DQSNDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DQSNDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DQSNDLY` writer - DQSNDLY"]
pub struct DQSNDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DQSNDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 23)) | ((value as u32 & 0x07) << 23);
        self.w
    }
}
#[doc = "Field `DMDLY` reader - DMDLY"]
pub struct DMDLY_R(crate::FieldReader<u8, u8>);
impl DMDLY_R {
    pub(crate) fn new(bits: u8) -> Self {
        DMDLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMDLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMDLY` writer - DMDLY"]
pub struct DMDLY_W<'a> {
    w: &'a mut W,
}
impl<'a> DMDLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 26)) | ((value as u32 & 0x0f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    pub fn r0dgsl(&self) -> R0DGSL_R {
        R0DGSL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    pub fn r0dgps(&self) -> R0DGPS_R {
        R0DGPS_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    pub fn dqsdly(&self) -> DQSDLY_R {
        DQSDLY_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    pub fn dqsndly(&self) -> DQSNDLY_R {
        DQSNDLY_R::new(((self.bits >> 23) & 0x07) as u8)
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    pub fn dmdly(&self) -> DMDLY_R {
        DMDLY_R::new(((self.bits >> 26) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - R0DGSL"]
    #[inline(always)]
    pub fn r0dgsl(&mut self) -> R0DGSL_W {
        R0DGSL_W { w: self }
    }
    #[doc = "Bits 12:13 - R0DGPS"]
    #[inline(always)]
    pub fn r0dgps(&mut self) -> R0DGPS_W {
        R0DGPS_W { w: self }
    }
    #[doc = "Bits 20:22 - DQSDLY"]
    #[inline(always)]
    pub fn dqsdly(&mut self) -> DQSDLY_W {
        DQSDLY_W { w: self }
    }
    #[doc = "Bits 23:25 - DQSNDLY"]
    #[inline(always)]
    pub fn dqsndly(&mut self) -> DQSNDLY_W {
        DQSNDLY_W { w: self }
    }
    #[doc = "Bits 26:29 - DMDLY"]
    #[inline(always)]
    pub fn dmdly(&mut self) -> DMDLY_W {
        DMDLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC byte lane 3 DQST register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dx3dqstr](index.html) module"]
pub struct DDRPHYC_DX3DQSTR_SPEC;
impl crate::RegisterSpec for DDRPHYC_DX3DQSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dx3dqstr::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DX3DQSTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dx3dqstr::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DX3DQSTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DX3DQSTR to value 0x3db0_2000"]
impl crate::Resettable for DDRPHYC_DX3DQSTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3db0_2000
    }
}
