#[doc = "Register `SECCFGR2` reader"]
pub struct R(crate::R<SECCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECCFGR2` writer"]
pub struct W(crate::W<SECCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECCFGR2_SPEC>;
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
impl From<crate::W<SECCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEC32` reader - SEC32"]
pub struct SEC32_R(crate::FieldReader<bool, bool>);
impl SEC32_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC32` writer - SEC32"]
pub struct SEC32_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC32_W<'a> {
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
#[doc = "Field `SEC33` reader - SEC33"]
pub struct SEC33_R(crate::FieldReader<bool, bool>);
impl SEC33_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC33` writer - SEC33"]
pub struct SEC33_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC33_W<'a> {
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
#[doc = "Field `SEC34` reader - SEC34"]
pub struct SEC34_R(crate::FieldReader<bool, bool>);
impl SEC34_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC34` writer - SEC34"]
pub struct SEC34_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC34_W<'a> {
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
#[doc = "Field `SEC35` reader - SEC35"]
pub struct SEC35_R(crate::FieldReader<bool, bool>);
impl SEC35_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC35` writer - SEC35"]
pub struct SEC35_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC35_W<'a> {
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
#[doc = "Field `SEC36` reader - SEC36"]
pub struct SEC36_R(crate::FieldReader<bool, bool>);
impl SEC36_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC36` writer - SEC36"]
pub struct SEC36_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC36_W<'a> {
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
#[doc = "Field `SEC37` reader - SEC37"]
pub struct SEC37_R(crate::FieldReader<bool, bool>);
impl SEC37_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC37` writer - SEC37"]
pub struct SEC37_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC37_W<'a> {
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
#[doc = "Field `SEC38` reader - SEC38"]
pub struct SEC38_R(crate::FieldReader<bool, bool>);
impl SEC38_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC38` writer - SEC38"]
pub struct SEC38_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC38_W<'a> {
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
#[doc = "Field `SEC39` reader - SEC39"]
pub struct SEC39_R(crate::FieldReader<bool, bool>);
impl SEC39_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC39` writer - SEC39"]
pub struct SEC39_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC39_W<'a> {
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
#[doc = "Field `SEC40` reader - SEC40"]
pub struct SEC40_R(crate::FieldReader<bool, bool>);
impl SEC40_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC40` writer - SEC40"]
pub struct SEC40_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC40_W<'a> {
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
#[doc = "Field `SEC41` reader - SEC41"]
pub struct SEC41_R(crate::FieldReader<bool, bool>);
impl SEC41_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC41` writer - SEC41"]
pub struct SEC41_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC41_W<'a> {
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
#[doc = "Field `SEC42` reader - SEC42"]
pub struct SEC42_R(crate::FieldReader<bool, bool>);
impl SEC42_R {
    pub(crate) fn new(bits: bool) -> Self {
        SEC42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SEC42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SEC42` writer - SEC42"]
pub struct SEC42_W<'a> {
    w: &'a mut W,
}
impl<'a> SEC42_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    pub fn sec32(&self) -> SEC32_R {
        SEC32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    pub fn sec33(&self) -> SEC33_R {
        SEC33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    pub fn sec34(&self) -> SEC34_R {
        SEC34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    pub fn sec35(&self) -> SEC35_R {
        SEC35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    pub fn sec36(&self) -> SEC36_R {
        SEC36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    pub fn sec37(&self) -> SEC37_R {
        SEC37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    pub fn sec38(&self) -> SEC38_R {
        SEC38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    pub fn sec39(&self) -> SEC39_R {
        SEC39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    pub fn sec40(&self) -> SEC40_R {
        SEC40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    pub fn sec41(&self) -> SEC41_R {
        SEC41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    pub fn sec42(&self) -> SEC42_R {
        SEC42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SEC32"]
    #[inline(always)]
    pub fn sec32(&mut self) -> SEC32_W {
        SEC32_W { w: self }
    }
    #[doc = "Bit 1 - SEC33"]
    #[inline(always)]
    pub fn sec33(&mut self) -> SEC33_W {
        SEC33_W { w: self }
    }
    #[doc = "Bit 2 - SEC34"]
    #[inline(always)]
    pub fn sec34(&mut self) -> SEC34_W {
        SEC34_W { w: self }
    }
    #[doc = "Bit 3 - SEC35"]
    #[inline(always)]
    pub fn sec35(&mut self) -> SEC35_W {
        SEC35_W { w: self }
    }
    #[doc = "Bit 4 - SEC36"]
    #[inline(always)]
    pub fn sec36(&mut self) -> SEC36_W {
        SEC36_W { w: self }
    }
    #[doc = "Bit 5 - SEC37"]
    #[inline(always)]
    pub fn sec37(&mut self) -> SEC37_W {
        SEC37_W { w: self }
    }
    #[doc = "Bit 6 - SEC38"]
    #[inline(always)]
    pub fn sec38(&mut self) -> SEC38_W {
        SEC38_W { w: self }
    }
    #[doc = "Bit 7 - SEC39"]
    #[inline(always)]
    pub fn sec39(&mut self) -> SEC39_W {
        SEC39_W { w: self }
    }
    #[doc = "Bit 8 - SEC40"]
    #[inline(always)]
    pub fn sec40(&mut self) -> SEC40_W {
        SEC40_W { w: self }
    }
    #[doc = "Bit 9 - SEC41"]
    #[inline(always)]
    pub fn sec41(&mut self) -> SEC41_W {
        SEC41_W { w: self }
    }
    #[doc = "Bit 10 - SEC42"]
    #[inline(always)]
    pub fn sec42(&mut self) -> SEC42_W {
        SEC42_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTI security enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seccfgr2](index.html) module"]
pub struct SECCFGR2_SPEC;
impl crate::RegisterSpec for SECCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seccfgr2::R](R) reader structure"]
impl crate::Readable for SECCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seccfgr2::W](W) writer structure"]
impl crate::Writable for SECCFGR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECCFGR2 to value 0"]
impl crate::Resettable for SECCFGR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
