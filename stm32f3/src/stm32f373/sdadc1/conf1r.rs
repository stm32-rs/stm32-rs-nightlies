#[doc = "Register `CONF1R` reader"]
pub struct R(crate::R<CONF1R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF1R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF1R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF1R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF1R` writer"]
pub struct W(crate::W<CONF1R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF1R_SPEC>;
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
impl From<crate::W<CONF1R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF1R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMMON1` reader - Common mode for configuration 1"]
pub struct COMMON1_R(crate::FieldReader<u8, u8>);
impl COMMON1_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMMON1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMMON1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMMON1` writer - Common mode for configuration 1"]
pub struct COMMON1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMMON1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `SE1` reader - Single-ended mode for configuration 1"]
pub struct SE1_R(crate::FieldReader<u8, u8>);
impl SE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        SE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SE1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SE1` writer - Single-ended mode for configuration 1"]
pub struct SE1_W<'a> {
    w: &'a mut W,
}
impl<'a> SE1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "Field `GAIN1` reader - Gain setting for configuration 1"]
pub struct GAIN1_R(crate::FieldReader<u8, u8>);
impl GAIN1_R {
    pub(crate) fn new(bits: u8) -> Self {
        GAIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GAIN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GAIN1` writer - Gain setting for configuration 1"]
pub struct GAIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> GAIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `OFFSET1` reader - Twelve-bit calibration offset for configuration 1"]
pub struct OFFSET1_R(crate::FieldReader<u16, u16>);
impl OFFSET1_R {
    pub(crate) fn new(bits: u16) -> Self {
        OFFSET1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OFFSET1_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OFFSET1` writer - Twelve-bit calibration offset for configuration 1"]
pub struct OFFSET1_W<'a> {
    w: &'a mut W,
}
impl<'a> OFFSET1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0fff) | (value as u32 & 0x0fff);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    pub fn common1(&self) -> COMMON1_R {
        COMMON1_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    pub fn se1(&self) -> SE1_R {
        SE1_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    pub fn gain1(&self) -> GAIN1_R {
        GAIN1_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 30:31 - Common mode for configuration 1"]
    #[inline(always)]
    pub fn common1(&mut self) -> COMMON1_W {
        COMMON1_W { w: self }
    }
    #[doc = "Bits 26:27 - Single-ended mode for configuration 1"]
    #[inline(always)]
    pub fn se1(&mut self) -> SE1_W {
        SE1_W { w: self }
    }
    #[doc = "Bits 20:22 - Gain setting for configuration 1"]
    #[inline(always)]
    pub fn gain1(&mut self) -> GAIN1_W {
        GAIN1_W { w: self }
    }
    #[doc = "Bits 0:11 - Twelve-bit calibration offset for configuration 1"]
    #[inline(always)]
    pub fn offset1(&mut self) -> OFFSET1_W {
        OFFSET1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration 1 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf1r](index.html) module"]
pub struct CONF1R_SPEC;
impl crate::RegisterSpec for CONF1R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf1r::R](R) reader structure"]
impl crate::Readable for CONF1R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf1r::W](W) writer structure"]
impl crate::Writable for CONF1R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONF1R to value 0"]
impl crate::Resettable for CONF1R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
