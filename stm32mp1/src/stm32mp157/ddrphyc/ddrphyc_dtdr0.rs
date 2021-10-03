#[doc = "Register `DDRPHYC_DTDR0` reader"]
pub struct R(crate::R<DDRPHYC_DTDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DDRPHYC_DTDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DDRPHYC_DTDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DDRPHYC_DTDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DDRPHYC_DTDR0` writer"]
pub struct W(crate::W<DDRPHYC_DTDR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DDRPHYC_DTDR0_SPEC>;
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
impl From<crate::W<DDRPHYC_DTDR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DDRPHYC_DTDR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTBYTE0` reader - DTBYTE0"]
pub struct DTBYTE0_R(crate::FieldReader<u8, u8>);
impl DTBYTE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE0` writer - DTBYTE0"]
pub struct DTBYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `DTBYTE1` reader - DTBYTE1"]
pub struct DTBYTE1_R(crate::FieldReader<u8, u8>);
impl DTBYTE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE1` writer - DTBYTE1"]
pub struct DTBYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `DTBYTE2` reader - DTBYTE2"]
pub struct DTBYTE2_R(crate::FieldReader<u8, u8>);
impl DTBYTE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE2` writer - DTBYTE2"]
pub struct DTBYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
#[doc = "Field `DTBYTE3` reader - DTBYTE3"]
pub struct DTBYTE3_R(crate::FieldReader<u8, u8>);
impl DTBYTE3_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTBYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTBYTE3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTBYTE3` writer - DTBYTE3"]
pub struct DTBYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> DTBYTE3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&self) -> DTBYTE0_R {
        DTBYTE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&self) -> DTBYTE1_R {
        DTBYTE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&self) -> DTBYTE2_R {
        DTBYTE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&self) -> DTBYTE3_R {
        DTBYTE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DTBYTE0"]
    #[inline(always)]
    pub fn dtbyte0(&mut self) -> DTBYTE0_W {
        DTBYTE0_W { w: self }
    }
    #[doc = "Bits 8:15 - DTBYTE1"]
    #[inline(always)]
    pub fn dtbyte1(&mut self) -> DTBYTE1_W {
        DTBYTE1_W { w: self }
    }
    #[doc = "Bits 16:23 - DTBYTE2"]
    #[inline(always)]
    pub fn dtbyte2(&mut self) -> DTBYTE2_W {
        DTBYTE2_W { w: self }
    }
    #[doc = "Bits 24:31 - DTBYTE3"]
    #[inline(always)]
    pub fn dtbyte3(&mut self) -> DTBYTE3_W {
        DTBYTE3_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DDRPHYC DTD register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ddrphyc_dtdr0](index.html) module"]
pub struct DDRPHYC_DTDR0_SPEC;
impl crate::RegisterSpec for DDRPHYC_DTDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ddrphyc_dtdr0::R](R) reader structure"]
impl crate::Readable for DDRPHYC_DTDR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ddrphyc_dtdr0::W](W) writer structure"]
impl crate::Writable for DDRPHYC_DTDR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DDRPHYC_DTDR0 to value 0xdd22_ee11"]
impl crate::Resettable for DDRPHYC_DTDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xdd22_ee11
    }
}
