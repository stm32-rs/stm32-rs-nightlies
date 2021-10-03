#[doc = "Register `RXF0S` reader"]
pub struct R(crate::R<RXF0S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXF0S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXF0S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXF0S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXF0S` writer"]
pub struct W(crate::W<RXF0S_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXF0S_SPEC>;
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
impl From<crate::W<RXF0S_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXF0S_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `F0FL` reader - F0FL"]
pub struct F0FL_R(crate::FieldReader<u8, u8>);
impl F0FL_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0FL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0FL` writer - F0FL"]
pub struct F0FL_W<'a> {
    w: &'a mut W,
}
impl<'a> F0FL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `F0GI` reader - F0GI"]
pub struct F0GI_R(crate::FieldReader<u8, u8>);
impl F0GI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0GI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0GI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0GI` writer - F0GI"]
pub struct F0GI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0GI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `F0PI` reader - F0PI"]
pub struct F0PI_R(crate::FieldReader<u8, u8>);
impl F0PI_R {
    pub(crate) fn new(bits: u8) -> Self {
        F0PI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0PI_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0PI` writer - F0PI"]
pub struct F0PI_W<'a> {
    w: &'a mut W,
}
impl<'a> F0PI_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Field `F0F` reader - F0F"]
pub struct F0F_R(crate::FieldReader<bool, bool>);
impl F0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        F0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for F0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `F0F` writer - F0F"]
pub struct F0F_W<'a> {
    w: &'a mut W,
}
impl<'a> F0F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `RF0L` reader - RF0L"]
pub struct RF0L_R(crate::FieldReader<bool, bool>);
impl RF0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0L` writer - RF0L"]
pub struct RF0L_W<'a> {
    w: &'a mut W,
}
impl<'a> RF0L_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 25)) | ((value as u32 & 0x01) << 25);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0FL_R {
        F0FL_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&self) -> F0GI_R {
        F0GI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&self) -> F0PI_R {
        F0PI_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&self) -> F0F_R {
        F0F_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 25) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - F0FL"]
    #[inline(always)]
    pub fn f0fl(&mut self) -> F0FL_W {
        F0FL_W { w: self }
    }
    #[doc = "Bits 8:13 - F0GI"]
    #[inline(always)]
    pub fn f0gi(&mut self) -> F0GI_W {
        F0GI_W { w: self }
    }
    #[doc = "Bits 16:21 - F0PI"]
    #[inline(always)]
    pub fn f0pi(&mut self) -> F0PI_W {
        F0PI_W { w: self }
    }
    #[doc = "Bit 24 - F0F"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0F_W {
        F0F_W { w: self }
    }
    #[doc = "Bit 25 - RF0L"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W {
        RF0L_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxf0s](index.html) module"]
pub struct RXF0S_SPEC;
impl crate::RegisterSpec for RXF0S_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxf0s::R](R) reader structure"]
impl crate::Readable for RXF0S_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxf0s::W](W) writer structure"]
impl crate::Writable for RXF0S_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RXF0S to value 0"]
impl crate::Resettable for RXF0S_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
