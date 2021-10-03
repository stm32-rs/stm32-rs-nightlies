#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TAMP1E` reader - TAMP1E"]
pub struct TAMP1E_R(crate::FieldReader<bool, bool>);
impl TAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP1E` writer - TAMP1E"]
pub struct TAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP1E_W<'a> {
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
#[doc = "Field `TAMP2E` reader - TAMP2E"]
pub struct TAMP2E_R(crate::FieldReader<bool, bool>);
impl TAMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP2E` writer - TAMP2E"]
pub struct TAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP2E_W<'a> {
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
#[doc = "Field `TAMP3E` reader - TAMP3E"]
pub struct TAMP3E_R(crate::FieldReader<bool, bool>);
impl TAMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP3E` writer - TAMP3E"]
pub struct TAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP3E_W<'a> {
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
#[doc = "Field `TAMP4E` reader - TAMP4E"]
pub struct TAMP4E_R(crate::FieldReader<bool, bool>);
impl TAMP4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP4E` writer - TAMP4E"]
pub struct TAMP4E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP4E_W<'a> {
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
#[doc = "Field `TAMP5E` reader - TAMP5E"]
pub struct TAMP5E_R(crate::FieldReader<bool, bool>);
impl TAMP5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP5E` writer - TAMP5E"]
pub struct TAMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP5E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `TAMP6E` reader - TAMP6E"]
pub struct TAMP6E_R(crate::FieldReader<bool, bool>);
impl TAMP6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP6E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP6E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP6E` writer - TAMP6E"]
pub struct TAMP6E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP6E_W<'a> {
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
#[doc = "Field `TAMP7E` reader - TAMP7E"]
pub struct TAMP7E_R(crate::FieldReader<bool, bool>);
impl TAMP7E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP7E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP7E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP7E` writer - TAMP7E"]
pub struct TAMP7E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP7E_W<'a> {
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
#[doc = "Field `TAMP8E` reader - TAMP8E"]
pub struct TAMP8E_R(crate::FieldReader<bool, bool>);
impl TAMP8E_R {
    pub(crate) fn new(bits: bool) -> Self {
        TAMP8E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TAMP8E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TAMP8E` writer - TAMP8E"]
pub struct TAMP8E_W<'a> {
    w: &'a mut W,
}
impl<'a> TAMP8E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `ITAMP1E` reader - ITAMP1E"]
pub struct ITAMP1E_R(crate::FieldReader<bool, bool>);
impl ITAMP1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP1E` writer - ITAMP1E"]
pub struct ITAMP1E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP1E_W<'a> {
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
#[doc = "Field `ITAMP2E` reader - ITAMP2E"]
pub struct ITAMP2E_R(crate::FieldReader<bool, bool>);
impl ITAMP2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP2E` writer - ITAMP2E"]
pub struct ITAMP2E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP2E_W<'a> {
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
#[doc = "Field `ITAMP3E` reader - ITAMP3E"]
pub struct ITAMP3E_R(crate::FieldReader<bool, bool>);
impl ITAMP3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3E` writer - ITAMP3E"]
pub struct ITAMP3E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3E_W<'a> {
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
#[doc = "Field `ITAMP5E` reader - ITAMP5E"]
pub struct ITAMP5E_R(crate::FieldReader<bool, bool>);
impl ITAMP5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP5E` writer - ITAMP5E"]
pub struct ITAMP5E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5E_W<'a> {
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
#[doc = "Field `ITAMP8E` reader - ITAMP5E"]
pub struct ITAMP8E_R(crate::FieldReader<bool, bool>);
impl ITAMP8E_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP8E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP8E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP8E` writer - ITAMP5E"]
pub struct ITAMP8E_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8E_W<'a> {
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
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TAMP3E"]
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TAMP4E"]
    #[inline(always)]
    pub fn tamp4e(&self) -> TAMP4E_R {
        TAMP4E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TAMP5E"]
    #[inline(always)]
    pub fn tamp5e(&self) -> TAMP5E_R {
        TAMP5E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TAMP6E"]
    #[inline(always)]
    pub fn tamp6e(&self) -> TAMP6E_R {
        TAMP6E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TAMP7E"]
    #[inline(always)]
    pub fn tamp7e(&self) -> TAMP7E_R {
        TAMP7E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TAMP8E"]
    #[inline(always)]
    pub fn tamp8e(&self) -> TAMP8E_R {
        TAMP8E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 16 - ITAMP1E"]
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - ITAMP2E"]
    #[inline(always)]
    pub fn itamp2e(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 23 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TAMP1E"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W {
        TAMP1E_W { w: self }
    }
    #[doc = "Bit 1 - TAMP2E"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W {
        TAMP2E_W { w: self }
    }
    #[doc = "Bit 2 - TAMP3E"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W {
        TAMP3E_W { w: self }
    }
    #[doc = "Bit 3 - TAMP4E"]
    #[inline(always)]
    pub fn tamp4e(&mut self) -> TAMP4E_W {
        TAMP4E_W { w: self }
    }
    #[doc = "Bit 4 - TAMP5E"]
    #[inline(always)]
    pub fn tamp5e(&mut self) -> TAMP5E_W {
        TAMP5E_W { w: self }
    }
    #[doc = "Bit 5 - TAMP6E"]
    #[inline(always)]
    pub fn tamp6e(&mut self) -> TAMP6E_W {
        TAMP6E_W { w: self }
    }
    #[doc = "Bit 6 - TAMP7E"]
    #[inline(always)]
    pub fn tamp7e(&mut self) -> TAMP7E_W {
        TAMP7E_W { w: self }
    }
    #[doc = "Bit 7 - TAMP8E"]
    #[inline(always)]
    pub fn tamp8e(&mut self) -> TAMP8E_W {
        TAMP8E_W { w: self }
    }
    #[doc = "Bit 16 - ITAMP1E"]
    #[inline(always)]
    pub fn itamp1e(&mut self) -> ITAMP1E_W {
        ITAMP1E_W { w: self }
    }
    #[doc = "Bit 17 - ITAMP2E"]
    #[inline(always)]
    pub fn itamp2e(&mut self) -> ITAMP2E_W {
        ITAMP2E_W { w: self }
    }
    #[doc = "Bit 18 - ITAMP3E"]
    #[inline(always)]
    pub fn itamp3e(&mut self) -> ITAMP3E_W {
        ITAMP3E_W { w: self }
    }
    #[doc = "Bit 20 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp5e(&mut self) -> ITAMP5E_W {
        ITAMP5E_W { w: self }
    }
    #[doc = "Bit 23 - ITAMP5E"]
    #[inline(always)]
    pub fn itamp8e(&mut self) -> ITAMP8E_W {
        ITAMP8E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_0000
    }
}
