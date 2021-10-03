#[doc = "Register `SIPCR` reader"]
pub struct R(crate::R<SIPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SIPCR` writer"]
pub struct W(crate::W<SIPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SIPCR_SPEC>;
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
impl From<crate::W<SIPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SIPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAES1` reader - Enable AES1 KEY\\[7:0\\]
security."]
pub struct SAES1_R(crate::FieldReader<bool, bool>);
impl SAES1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAES1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAES1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAES1` writer - Enable AES1 KEY\\[7:0\\]
security."]
pub struct SAES1_W<'a> {
    w: &'a mut W,
}
impl<'a> SAES1_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `SAES2` reader - Enable AES2 security."]
pub struct SAES2_R(crate::FieldReader<bool, bool>);
impl SAES2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SAES2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAES2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAES2` writer - Enable AES2 security."]
pub struct SAES2_W<'a> {
    w: &'a mut W,
}
impl<'a> SAES2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `SPKA` reader - Enable PKA security"]
pub struct SPKA_R(crate::FieldReader<bool, bool>);
impl SPKA_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPKA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPKA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPKA` writer - Enable PKA security"]
pub struct SPKA_W<'a> {
    w: &'a mut W,
}
impl<'a> SPKA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `SRNG` reader - Enable True RNG security"]
pub struct SRNG_R(crate::FieldReader<bool, bool>);
impl SRNG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SRNG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRNG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRNG` writer - Enable True RNG security"]
pub struct SRNG_W<'a> {
    w: &'a mut W,
}
impl<'a> SRNG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&self) -> SAES1_R {
        SAES1_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&self) -> SAES2_R {
        SAES2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&self) -> SPKA_R {
        SPKA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&self) -> SRNG_R {
        SRNG_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable AES1 KEY\\[7:0\\]
security."]
    #[inline(always)]
    pub fn saes1(&mut self) -> SAES1_W {
        SAES1_W { w: self }
    }
    #[doc = "Bit 1 - Enable AES2 security."]
    #[inline(always)]
    pub fn saes2(&mut self) -> SAES2_W {
        SAES2_W { w: self }
    }
    #[doc = "Bit 2 - Enable PKA security"]
    #[inline(always)]
    pub fn spka(&mut self) -> SPKA_W {
        SPKA_W { w: self }
    }
    #[doc = "Bit 3 - Enable True RNG security"]
    #[inline(always)]
    pub fn srng(&mut self) -> SRNG_W {
        SRNG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "secure IP control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sipcr](index.html) module"]
pub struct SIPCR_SPEC;
impl crate::RegisterSpec for SIPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sipcr::R](R) reader structure"]
impl crate::Readable for SIPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sipcr::W](W) writer structure"]
impl crate::Writable for SIPCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SIPCR to value 0"]
impl crate::Resettable for SIPCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
