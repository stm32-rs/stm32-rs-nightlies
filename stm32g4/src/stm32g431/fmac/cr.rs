#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET` reader - RESET"]
pub struct RESET_R(crate::FieldReader<bool, bool>);
impl RESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESET` writer - RESET"]
pub struct RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_W<'a> {
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
#[doc = "Field `CLIPEN` reader - CLIPEN"]
pub struct CLIPEN_R(crate::FieldReader<bool, bool>);
impl CLIPEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLIPEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLIPEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLIPEN` writer - CLIPEN"]
pub struct CLIPEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLIPEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `DMAWEN` reader - DMAWEN"]
pub struct DMAWEN_R(crate::FieldReader<bool, bool>);
impl DMAWEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAWEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAWEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAWEN` writer - DMAWEN"]
pub struct DMAWEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAWEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `DMAREN` reader - DMAREN"]
pub struct DMAREN_R(crate::FieldReader<bool, bool>);
impl DMAREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMAREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMAREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMAREN` writer - DMAREN"]
pub struct DMAREN_W<'a> {
    w: &'a mut W,
}
impl<'a> DMAREN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SATIEN` reader - SATIEN"]
pub struct SATIEN_R(crate::FieldReader<bool, bool>);
impl SATIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SATIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SATIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SATIEN` writer - SATIEN"]
pub struct SATIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SATIEN_W<'a> {
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
#[doc = "Field `UNFLIEN` reader - UNFLIEN"]
pub struct UNFLIEN_R(crate::FieldReader<bool, bool>);
impl UNFLIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNFLIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNFLIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNFLIEN` writer - UNFLIEN"]
pub struct UNFLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> UNFLIEN_W<'a> {
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
#[doc = "Field `OVFLIEN` reader - OVFLIEN"]
pub struct OVFLIEN_R(crate::FieldReader<bool, bool>);
impl OVFLIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OVFLIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVFLIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OVFLIEN` writer - OVFLIEN"]
pub struct OVFLIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> OVFLIEN_W<'a> {
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
#[doc = "Field `WIEN` reader - WIEN"]
pub struct WIEN_R(crate::FieldReader<bool, bool>);
impl WIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        WIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WIEN` writer - WIEN"]
pub struct WIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> WIEN_W<'a> {
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
#[doc = "Field `RIEN` reader - RIEN"]
pub struct RIEN_R(crate::FieldReader<bool, bool>);
impl RIEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        RIEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RIEN` writer - RIEN"]
pub struct RIEN_W<'a> {
    w: &'a mut W,
}
impl<'a> RIEN_W<'a> {
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
impl R {
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    pub fn clipen(&self) -> CLIPEN_R {
        CLIPEN_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&self) -> DMAWEN_R {
        DMAWEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&self) -> DMAREN_R {
        DMAREN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    pub fn satien(&self) -> SATIEN_R {
        SATIEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    pub fn unflien(&self) -> UNFLIEN_R {
        UNFLIEN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    pub fn ovflien(&self) -> OVFLIEN_R {
        OVFLIEN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    pub fn wien(&self) -> WIEN_R {
        WIEN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    pub fn rien(&self) -> RIEN_R {
        RIEN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - RESET"]
    #[inline(always)]
    pub fn reset(&mut self) -> RESET_W {
        RESET_W { w: self }
    }
    #[doc = "Bit 15 - CLIPEN"]
    #[inline(always)]
    pub fn clipen(&mut self) -> CLIPEN_W {
        CLIPEN_W { w: self }
    }
    #[doc = "Bit 9 - DMAWEN"]
    #[inline(always)]
    pub fn dmawen(&mut self) -> DMAWEN_W {
        DMAWEN_W { w: self }
    }
    #[doc = "Bit 8 - DMAREN"]
    #[inline(always)]
    pub fn dmaren(&mut self) -> DMAREN_W {
        DMAREN_W { w: self }
    }
    #[doc = "Bit 4 - SATIEN"]
    #[inline(always)]
    pub fn satien(&mut self) -> SATIEN_W {
        SATIEN_W { w: self }
    }
    #[doc = "Bit 3 - UNFLIEN"]
    #[inline(always)]
    pub fn unflien(&mut self) -> UNFLIEN_W {
        UNFLIEN_W { w: self }
    }
    #[doc = "Bit 2 - OVFLIEN"]
    #[inline(always)]
    pub fn ovflien(&mut self) -> OVFLIEN_W {
        OVFLIEN_W { w: self }
    }
    #[doc = "Bit 1 - WIEN"]
    #[inline(always)]
    pub fn wien(&mut self) -> WIEN_W {
        WIEN_W { w: self }
    }
    #[doc = "Bit 0 - RIEN"]
    #[inline(always)]
    pub fn rien(&mut self) -> RIEN_W {
        RIEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FMAC Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
