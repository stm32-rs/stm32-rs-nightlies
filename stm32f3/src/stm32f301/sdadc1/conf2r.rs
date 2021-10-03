#[doc = "Register `CONF2R` reader"]
pub struct R(crate::R<CONF2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF2R` writer"]
pub struct W(crate::W<CONF2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF2R_SPEC>;
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
impl From<crate::W<CONF2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMON2` reader - Common mode for configuration 2"]
pub struct COMMON2_R(crate::FieldReader<u8, u8>);
impl COMMON2_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMMON2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMON2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMON2` writer - Common mode for configuration 2"]
pub struct COMMON2_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `SE2` reader - Single-ended mode for configuration 2"]
pub struct SE2_R(crate::FieldReader<u8, u8>);
impl SE2_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE2` writer - Single-ended mode for configuration 2"]
pub struct SE2_W<'a> {
    w: &'a mut W,
}
impl<'a> SE2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `GAIN2` reader - Gain setting for configuration 2"]
pub struct GAIN2_R(crate::FieldReader<u8, u8>);
impl GAIN2_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN2` writer - Gain setting for configuration 2"]
pub struct GAIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `OFFSET2` reader - Twelve-bit calibration offset for configuration 2"]
pub struct OFFSET2_R(crate::FieldReader<u16, u16>);
impl OFFSET2_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET2_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET2` writer - Twelve-bit calibration offset for configuration 2"]
pub struct OFFSET2_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&self) -> COMMON2_R {
        COMMON2_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&self) -> GAIN2_R {
        GAIN2_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&self) -> OFFSET2_R {
        OFFSET2_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 2"]
    #[inline(always)]
    pub fn common2(&mut self) -> COMMON2_W {
        COMMON2_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 2"]
    #[inline(always)]
    pub fn se2(&mut self) -> SE2_W {
        SE2_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 2"]
    #[inline(always)]
    pub fn gain2(&mut self) -> GAIN2_W {
        GAIN2_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 2"]
    #[inline(always)]
    pub fn offset2(&mut self) -> OFFSET2_W {
        OFFSET2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf2r](index.html) module"]
pub struct CONF2R_SPEC;
impl crate::RegisterSpec for CONF2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf2r::R](R) reader structure"]
impl crate::Readable for CONF2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf2r::W](W) writer structure"]
impl crate::Writable for CONF2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF2R to value 0"]
impl crate::Resettable for CONF2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
