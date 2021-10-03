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
#[doc = "Field `B0OIE` reader - Buffer 0 overflow interrupt enable"]
pub struct B0OIE_R(crate::FieldReader<bool, bool>);
impl B0OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        B0OIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B0OIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B0OIE` writer - Buffer 0 overflow interrupt enable"]
pub struct B0OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B0OIE_W<'a> {
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
#[doc = "Field `B1OIE` reader - Buffer 1 overflow interrupt enable"]
pub struct B1OIE_R(crate::FieldReader<bool, bool>);
impl B1OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        B1OIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B1OIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B1OIE` writer - Buffer 1 overflow interrupt enable"]
pub struct B1OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B1OIE_W<'a> {
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
#[doc = "Field `B2OIE` reader - Buffer 2 overflow interrupt enable"]
pub struct B2OIE_R(crate::FieldReader<bool, bool>);
impl B2OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        B2OIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B2OIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B2OIE` writer - Buffer 2 overflow interrupt enable"]
pub struct B2OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B2OIE_W<'a> {
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
#[doc = "Field `B3OIE` reader - Buffer 3 overflow interrupt enable"]
pub struct B3OIE_R(crate::FieldReader<bool, bool>);
impl B3OIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        B3OIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B3OIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B3OIE` writer - Buffer 3 overflow interrupt enable"]
pub struct B3OIE_W<'a> {
    w: &'a mut W,
}
impl<'a> B3OIE_W<'a> {
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
#[doc = "Field `AMEIE` reader - AHB master error interrupt enable"]
pub struct AMEIE_R(crate::FieldReader<bool, bool>);
impl AMEIE_R {
    pub(crate) fn new(bits: bool) -> Self {
        AMEIE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AMEIE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AMEIE` writer - AHB master error interrupt enable"]
pub struct AMEIE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMEIE_W<'a> {
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
#[doc = "Field `BM192` reader - 192 Block mode"]
pub struct BM192_R(crate::FieldReader<bool, bool>);
impl BM192_R {
    pub(crate) fn new(bits: bool) -> Self {
        BM192_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BM192_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BM192` writer - 192 Block mode"]
pub struct BM192_W<'a> {
    w: &'a mut W,
}
impl<'a> BM192_W<'a> {
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
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn b0oie(&self) -> B0OIE_R {
        B0OIE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn b1oie(&self) -> B1OIE_R {
        B1OIE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    pub fn b2oie(&self) -> B2OIE_R {
        B2OIE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    pub fn b3oie(&self) -> B3OIE_R {
        B3OIE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    pub fn ameie(&self) -> AMEIE_R {
        AMEIE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    pub fn bm192(&self) -> BM192_R {
        BM192_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buffer 0 overflow interrupt enable"]
    #[inline(always)]
    pub fn b0oie(&mut self) -> B0OIE_W {
        B0OIE_W { w: self }
    }
    #[doc = "Bit 1 - Buffer 1 overflow interrupt enable"]
    #[inline(always)]
    pub fn b1oie(&mut self) -> B1OIE_W {
        B1OIE_W { w: self }
    }
    #[doc = "Bit 2 - Buffer 2 overflow interrupt enable"]
    #[inline(always)]
    pub fn b2oie(&mut self) -> B2OIE_W {
        B2OIE_W { w: self }
    }
    #[doc = "Bit 3 - Buffer 3 overflow interrupt enable"]
    #[inline(always)]
    pub fn b3oie(&mut self) -> B3OIE_W {
        B3OIE_W { w: self }
    }
    #[doc = "Bit 4 - AHB master error interrupt enable"]
    #[inline(always)]
    pub fn ameie(&mut self) -> AMEIE_W {
        AMEIE_W { w: self }
    }
    #[doc = "Bit 6 - 192 Block mode"]
    #[inline(always)]
    pub fn bm192(&mut self) -> BM192_W {
        BM192_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
