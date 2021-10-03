#[doc = "Register `RNG_SR` reader"]
pub struct R(crate::R<RNG_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RNG_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RNG_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RNG_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RNG_SR` writer"]
pub struct W(crate::W<RNG_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RNG_SR_SPEC>;
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
impl From<crate::W<RNG_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RNG_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRDY` reader - DRDY"]
pub struct DRDY_R(crate::FieldReader<bool, bool>);
impl DRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECS` reader - CECS"]
pub struct CECS_R(crate::FieldReader<bool, bool>);
impl CECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SECS` reader - SECS"]
pub struct SECS_R(crate::FieldReader<bool, bool>);
impl SECS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SECS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIS` reader - CEIS"]
pub struct CEIS_R(crate::FieldReader<bool, bool>);
impl CEIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CEIS` writer - CEIS"]
pub struct CEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CEIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `SEIS` reader - SEIS"]
pub struct SEIS_R(crate::FieldReader<bool, bool>);
impl SEIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEIS` writer - SEIS"]
pub struct SEIS_W<'a> {
    w: &'a mut W,
}
impl<'a> SEIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DRDY"]
    #[inline(always)]
    pub fn drdy(&self) -> DRDY_R {
        DRDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CECS"]
    #[inline(always)]
    pub fn cecs(&self) -> CECS_R {
        CECS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SECS"]
    #[inline(always)]
    pub fn secs(&self) -> SECS_R {
        SECS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CEIS"]
    #[inline(always)]
    pub fn ceis(&self) -> CEIS_R {
        CEIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SEIS"]
    #[inline(always)]
    pub fn seis(&self) -> SEIS_R {
        SEIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - CEIS"]
    #[inline(always)]
    pub fn ceis(&mut self) -> CEIS_W {
        CEIS_W { w: self }
    }
    #[doc = "Bit 6 - SEIS"]
    #[inline(always)]
    pub fn seis(&mut self) -> SEIS_W {
        SEIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RNG status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rng_sr](index.html) module"]
pub struct RNG_SR_SPEC;
impl crate::RegisterSpec for RNG_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rng_sr::R](R) reader structure"]
impl crate::Readable for RNG_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rng_sr::W](W) writer structure"]
impl crate::Writable for RNG_SR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RNG_SR to value 0"]
impl crate::Resettable for RNG_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
