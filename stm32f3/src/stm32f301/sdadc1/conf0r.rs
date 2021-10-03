#[doc = "Register `CONF0R` reader"]
pub struct R(crate::R<CONF0R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0R` writer"]
pub struct W(crate::W<CONF0R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0R_SPEC>;
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
impl From<crate::W<CONF0R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMON0` reader - Common mode for configuration 0"]
pub struct COMMON0_R(crate::FieldReader<u8, u8>);
impl COMMON0_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMMON0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMON0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMON0` writer - Common mode for configuration 0"]
pub struct COMMON0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `SE0` reader - Single-ended mode for configuration 0"]
pub struct SE0_R(crate::FieldReader<u8, u8>);
impl SE0_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE0` writer - Single-ended mode for configuration 0"]
pub struct SE0_W<'a> {
    w: &'a mut W,
}
impl<'a> SE0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `GAIN0` reader - Gain setting for configuration 0"]
pub struct GAIN0_R(crate::FieldReader<u8, u8>);
impl GAIN0_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN0` writer - Gain setting for configuration 0"]
pub struct GAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `OFFSET0` reader - Twelve-bit calibration offset for configuration 0"]
pub struct OFFSET0_R(crate::FieldReader<u16, u16>);
impl OFFSET0_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET0_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET0` writer - Twelve-bit calibration offset for configuration 0"]
pub struct OFFSET0_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&self) -> COMMON0_R {
        COMMON0_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&self) -> SE0_R {
        SE0_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&self) -> GAIN0_R {
        GAIN0_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&self) -> OFFSET0_R {
        OFFSET0_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 0"]
    #[inline(always)]
    pub fn common0(&mut self) -> COMMON0_W {
        COMMON0_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 0"]
    #[inline(always)]
    pub fn se0(&mut self) -> SE0_W {
        SE0_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 0"]
    #[inline(always)]
    pub fn gain0(&mut self) -> GAIN0_W {
        GAIN0_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 0"]
    #[inline(always)]
    pub fn offset0(&mut self) -> OFFSET0_W {
        OFFSET0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration 0 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0r](index.html) module"]
pub struct CONF0R_SPEC;
impl crate::RegisterSpec for CONF0R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0r::R](R) reader structure"]
impl crate::Readable for CONF0R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0r::W](W) writer structure"]
impl crate::Writable for CONF0R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF0R to value 0"]
impl crate::Resettable for CONF0R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
