#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITAMP1NOER` reader - ITAMP1NOER"]
pub struct ITAMP1NOER_R(crate::FieldReader<bool, bool>);
impl ITAMP1NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP1NOER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP1NOER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP1NOER` writer - ITAMP1NOER"]
pub struct ITAMP1NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP1NOER_W<'a> {
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
#[doc = "Field `ITAMP2NOER` reader - ITAMP2NOER"]
pub struct ITAMP2NOER_R(crate::FieldReader<bool, bool>);
impl ITAMP2NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP2NOER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP2NOER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP2NOER` writer - ITAMP2NOER"]
pub struct ITAMP2NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP2NOER_W<'a> {
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
#[doc = "Field `ITAMP3NOER` reader - ITAMP3NOER"]
pub struct ITAMP3NOER_R(crate::FieldReader<bool, bool>);
impl ITAMP3NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP3NOER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP3NOER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP3NOER` writer - ITAMP3NOER"]
pub struct ITAMP3NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP3NOER_W<'a> {
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
#[doc = "Field `ITAMP5NOER` reader - ITAMP5NOER"]
pub struct ITAMP5NOER_R(crate::FieldReader<bool, bool>);
impl ITAMP5NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP5NOER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP5NOER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP5NOER` writer - ITAMP5NOER"]
pub struct ITAMP5NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP5NOER_W<'a> {
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
#[doc = "Field `ITAMP8NOER` reader - ITAMP8NOER"]
pub struct ITAMP8NOER_R(crate::FieldReader<bool, bool>);
impl ITAMP8NOER_R {
    pub(crate) fn new(bits: bool) -> Self {
        ITAMP8NOER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ITAMP8NOER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ITAMP8NOER` writer - ITAMP8NOER"]
pub struct ITAMP8NOER_W<'a> {
    w: &'a mut W,
}
impl<'a> ITAMP8NOER_W<'a> {
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
impl R {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&self) -> ITAMP1NOER_R {
        ITAMP1NOER_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&self) -> ITAMP2NOER_R {
        ITAMP2NOER_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&self) -> ITAMP3NOER_R {
        ITAMP3NOER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&self) -> ITAMP5NOER_R {
        ITAMP5NOER_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&self) -> ITAMP8NOER_R {
        ITAMP8NOER_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ITAMP1NOER"]
    #[inline(always)]
    pub fn itamp1noer(&mut self) -> ITAMP1NOER_W {
        ITAMP1NOER_W { w: self }
    }
    #[doc = "Bit 1 - ITAMP2NOER"]
    #[inline(always)]
    pub fn itamp2noer(&mut self) -> ITAMP2NOER_W {
        ITAMP2NOER_W { w: self }
    }
    #[doc = "Bit 2 - ITAMP3NOER"]
    #[inline(always)]
    pub fn itamp3noer(&mut self) -> ITAMP3NOER_W {
        ITAMP3NOER_W { w: self }
    }
    #[doc = "Bit 4 - ITAMP5NOER"]
    #[inline(always)]
    pub fn itamp5noer(&mut self) -> ITAMP5NOER_W {
        ITAMP5NOER_W { w: self }
    }
    #[doc = "Bit 7 - ITAMP8NOER"]
    #[inline(always)]
    pub fn itamp8noer(&mut self) -> ITAMP8NOER_W {
        ITAMP8NOER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
