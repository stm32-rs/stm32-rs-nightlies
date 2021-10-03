#[doc = "Register `TAMP_IER` reader"]
pub struct R(crate::R<TAMP_IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TAMP_IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TAMP_IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TAMP_IER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TAMP_IER` writer"]
pub struct W(crate::W<TAMP_IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TAMP_IER_SPEC>;
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
impl From<crate::W<TAMP_IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TAMP_IER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1IE` reader - TAMP1IE"]
pub struct TAMP1IE_R(crate::FieldReader<bool, bool>);
impl TAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1IE` writer - TAMP1IE"]
pub struct TAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1IE_W<'a> {
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
#[doc = "Field `TAMP2IE` reader - TAMP2IE"]
pub struct TAMP2IE_R(crate::FieldReader<bool, bool>);
impl TAMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2IE` writer - TAMP2IE"]
pub struct TAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2IE_W<'a> {
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
#[doc = "Field `TAMP3IE` reader - TAMP3IE"]
pub struct TAMP3IE_R(crate::FieldReader<bool, bool>);
impl TAMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3IE` writer - TAMP3IE"]
pub struct TAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3IE_W<'a> {
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
#[doc = "Field `ITAMP1IE` reader - ITAMP1IE"]
pub struct ITAMP1IE_R(crate::FieldReader<bool, bool>);
impl ITAMP1IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP1IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP1IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP1IE` writer - ITAMP1IE"]
pub struct ITAMP1IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP1IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `ITAMP2IE` reader - ITAMP2IE"]
pub struct ITAMP2IE_R(crate::FieldReader<bool, bool>);
impl ITAMP2IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP2IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP2IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP2IE` writer - ITAMP2IE"]
pub struct ITAMP2IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP2IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `ITAMP3IE` reader - ITAMP3IE"]
pub struct ITAMP3IE_R(crate::FieldReader<bool, bool>);
impl ITAMP3IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP3IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3IE` writer - ITAMP3IE"]
pub struct ITAMP3IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `ITAMP4IE` reader - ITAMP4IE"]
pub struct ITAMP4IE_R(crate::FieldReader<bool, bool>);
impl ITAMP4IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP4IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP4IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP4IE` writer - ITAMP4IE"]
pub struct ITAMP4IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP4IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `ITAMP5IE` reader - ITAMP5IE"]
pub struct ITAMP5IE_R(crate::FieldReader<bool, bool>);
impl ITAMP5IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP5IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP5IE` writer - ITAMP5IE"]
pub struct ITAMP5IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `ITAMP8IE` reader - ITAMP8IE"]
pub struct ITAMP8IE_R(crate::FieldReader<bool, bool>);
impl ITAMP8IE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP8IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP8IE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP8IE` writer - ITAMP8IE"]
pub struct ITAMP8IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8IE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1IE"]
    #[inline(always)]
    pub fn itamp1ie(&self) -> ITAMP1IE_R {
        ITAMP1IE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ITAMP2IE"]
    #[inline(always)]
    pub fn itamp2ie(&self) -> ITAMP2IE_R {
        ITAMP2IE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    pub fn itamp4ie(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&self) -> ITAMP8IE_R {
        ITAMP8IE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1IE"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> TAMP1IE_W {
        TAMP1IE_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2IE"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> TAMP2IE_W {
        TAMP2IE_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3IE"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> TAMP3IE_W {
        TAMP3IE_W { w: self }
    }
    #[doc = "Bit 16 - ITAMP1IE"]
    #[inline(always)]
    pub fn itamp1ie(&mut self) -> ITAMP1IE_W {
        ITAMP1IE_W { w: self }
    }
    #[doc = "Bit 17 - ITAMP2IE"]
    #[inline(always)]
    pub fn itamp2ie(&mut self) -> ITAMP2IE_W {
        ITAMP2IE_W { w: self }
    }
    #[doc = "Bit 18 - ITAMP3IE"]
    #[inline(always)]
    pub fn itamp3ie(&mut self) -> ITAMP3IE_W {
        ITAMP3IE_W { w: self }
    }
    #[doc = "Bit 19 - ITAMP4IE"]
    #[inline(always)]
    pub fn itamp4ie(&mut self) -> ITAMP4IE_W {
        ITAMP4IE_W { w: self }
    }
    #[doc = "Bit 20 - ITAMP5IE"]
    #[inline(always)]
    pub fn itamp5ie(&mut self) -> ITAMP5IE_W {
        ITAMP5IE_W { w: self }
    }
    #[doc = "Bit 23 - ITAMP8IE"]
    #[inline(always)]
    pub fn itamp8ie(&mut self) -> ITAMP8IE_W {
        ITAMP8IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register can be protected against non-secure access. Refer to Section51.3.3: TAMP secure protection modes.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tamp_ier](index.html) module"]
pub struct TAMP_IER_SPEC;
impl crate::RegisterSpec for TAMP_IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tamp_ier::R](R) reader structure"]
impl crate::Readable for TAMP_IER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tamp_ier::W](W) writer structure"]
impl crate::Writable for TAMP_IER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TAMP_IER to value 0"]
impl crate::Resettable for TAMP_IER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
