#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTAMP1F` reader - CTAMP1F"]
pub struct CTAMP1F_R(crate::FieldReader<bool, bool>);
impl CTAMP1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTAMP1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTAMP1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTAMP1F` writer - CTAMP1F"]
pub struct CTAMP1F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP1F_W<'a> {
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
#[doc = "Field `CTAMP2F` reader - CTAMP2F"]
pub struct CTAMP2F_R(crate::FieldReader<bool, bool>);
impl CTAMP2F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTAMP2F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTAMP2F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTAMP2F` writer - CTAMP2F"]
pub struct CTAMP2F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP2F_W<'a> {
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
#[doc = "Field `CTAMP3F` reader - CTAMP3F"]
pub struct CTAMP3F_R(crate::FieldReader<bool, bool>);
impl CTAMP3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTAMP3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTAMP3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTAMP3F` writer - CTAMP3F"]
pub struct CTAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CTAMP3F_W<'a> {
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
#[doc = "Field `CITAMP3F` reader - CITAMP3F"]
pub struct CITAMP3F_R(crate::FieldReader<bool, bool>);
impl CITAMP3F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CITAMP3F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITAMP3F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITAMP3F` writer - CITAMP3F"]
pub struct CITAMP3F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP3F_W<'a> {
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
#[doc = "Field `CITAMP4F` reader - CITAMP4F"]
pub struct CITAMP4F_R(crate::FieldReader<bool, bool>);
impl CITAMP4F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CITAMP4F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITAMP4F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITAMP4F` writer - CITAMP4F"]
pub struct CITAMP4F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP4F_W<'a> {
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
#[doc = "Field `CITAMP5F` reader - CITAMP5F"]
pub struct CITAMP5F_R(crate::FieldReader<bool, bool>);
impl CITAMP5F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CITAMP5F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITAMP5F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITAMP5F` writer - CITAMP5F"]
pub struct CITAMP5F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP5F_W<'a> {
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
#[doc = "Field `CITAMP6F` reader - CITAMP6F"]
pub struct CITAMP6F_R(crate::FieldReader<bool, bool>);
impl CITAMP6F_R {
    pub(crate) fn new(bits: bool) -> Self {
        CITAMP6F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CITAMP6F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CITAMP6F` writer - CITAMP6F"]
pub struct CITAMP6F_W<'a> {
    w: &'a mut W,
}
impl<'a> CITAMP6F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&self) -> CTAMP1F_R {
        CTAMP1F_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&self) -> CTAMP2F_R {
        CTAMP2F_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&self) -> CTAMP3F_R {
        CTAMP3F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&self) -> CITAMP3F_R {
        CITAMP3F_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&self) -> CITAMP4F_R {
        CITAMP4F_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&self) -> CITAMP5F_R {
        CITAMP5F_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&self) -> CITAMP6F_R {
        CITAMP6F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTAMP1F"]
    #[inline(always)]
    pub fn ctamp1f(&mut self) -> CTAMP1F_W {
        CTAMP1F_W { w: self }
    }
    #[doc = "Bit 1 - CTAMP2F"]
    #[inline(always)]
    pub fn ctamp2f(&mut self) -> CTAMP2F_W {
        CTAMP2F_W { w: self }
    }
    #[doc = "Bit 2 - CTAMP3F"]
    #[inline(always)]
    pub fn ctamp3f(&mut self) -> CTAMP3F_W {
        CTAMP3F_W { w: self }
    }
    #[doc = "Bit 18 - CITAMP3F"]
    #[inline(always)]
    pub fn citamp3f(&mut self) -> CITAMP3F_W {
        CITAMP3F_W { w: self }
    }
    #[doc = "Bit 19 - CITAMP4F"]
    #[inline(always)]
    pub fn citamp4f(&mut self) -> CITAMP4F_W {
        CITAMP4F_W { w: self }
    }
    #[doc = "Bit 20 - CITAMP5F"]
    #[inline(always)]
    pub fn citamp5f(&mut self) -> CITAMP5F_W {
        CITAMP5F_W { w: self }
    }
    #[doc = "Bit 21 - CITAMP6F"]
    #[inline(always)]
    pub fn citamp6f(&mut self) -> CITAMP6F_W {
        CITAMP6F_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TAMP status clear register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
