#[doc = "Register `IOGCSR` reader"]
pub struct R(crate::R<IOGCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOGCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOGCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOGCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IOGCSR` writer"]
pub struct W(crate::W<IOGCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOGCSR_SPEC>;
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
impl From<crate::W<IOGCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOGCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `G8S` reader - Analog I/O group x status"]
pub struct G8S_R(crate::FieldReader<bool, bool>);
impl G8S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G8S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G7S` reader - Analog I/O group x status"]
pub struct G7S_R(crate::FieldReader<bool, bool>);
impl G7S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G7S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G6S` reader - Analog I/O group x status"]
pub struct G6S_R(crate::FieldReader<bool, bool>);
impl G6S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G6S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G5S` reader - Analog I/O group x status"]
pub struct G5S_R(crate::FieldReader<bool, bool>);
impl G5S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G5S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G4S` reader - Analog I/O group x status"]
pub struct G4S_R(crate::FieldReader<bool, bool>);
impl G4S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G4S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G3S` reader - Analog I/O group x status"]
pub struct G3S_R(crate::FieldReader<bool, bool>);
impl G3S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G3S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G2S` reader - Analog I/O group x status"]
pub struct G2S_R(crate::FieldReader<bool, bool>);
impl G2S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G2S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G1S` reader - Analog I/O group x status"]
pub struct G1S_R(crate::FieldReader<bool, bool>);
impl G1S_R {
    pub(crate) fn new(bits: bool) -> Self {
        G1S_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1S_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G8E` reader - Analog I/O group x enable"]
pub struct G8E_R(crate::FieldReader<bool, bool>);
impl G8E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G8E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G8E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G8E` writer - Analog I/O group x enable"]
pub struct G8E_W<'a> {
    w: &'a mut W,
}
impl<'a> G8E_W<'a> {
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
#[doc = "Field `G7E` reader - Analog I/O group x enable"]
pub struct G7E_R(crate::FieldReader<bool, bool>);
impl G7E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G7E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G7E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G7E` writer - Analog I/O group x enable"]
pub struct G7E_W<'a> {
    w: &'a mut W,
}
impl<'a> G7E_W<'a> {
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
#[doc = "Field `G6E` reader - Analog I/O group x enable"]
pub struct G6E_R(crate::FieldReader<bool, bool>);
impl G6E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G6E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G6E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G6E` writer - Analog I/O group x enable"]
pub struct G6E_W<'a> {
    w: &'a mut W,
}
impl<'a> G6E_W<'a> {
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
#[doc = "Field `G5E` reader - Analog I/O group x enable"]
pub struct G5E_R(crate::FieldReader<bool, bool>);
impl G5E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G5E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G5E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G5E` writer - Analog I/O group x enable"]
pub struct G5E_W<'a> {
    w: &'a mut W,
}
impl<'a> G5E_W<'a> {
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
#[doc = "Field `G4E` reader - Analog I/O group x enable"]
pub struct G4E_R(crate::FieldReader<bool, bool>);
impl G4E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G4E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G4E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G4E` writer - Analog I/O group x enable"]
pub struct G4E_W<'a> {
    w: &'a mut W,
}
impl<'a> G4E_W<'a> {
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
#[doc = "Field `G3E` reader - Analog I/O group x enable"]
pub struct G3E_R(crate::FieldReader<bool, bool>);
impl G3E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G3E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G3E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G3E` writer - Analog I/O group x enable"]
pub struct G3E_W<'a> {
    w: &'a mut W,
}
impl<'a> G3E_W<'a> {
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
#[doc = "Field `G2E` reader - Analog I/O group x enable"]
pub struct G2E_R(crate::FieldReader<bool, bool>);
impl G2E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G2E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G2E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G2E` writer - Analog I/O group x enable"]
pub struct G2E_W<'a> {
    w: &'a mut W,
}
impl<'a> G2E_W<'a> {
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
#[doc = "Field `G1E` reader - Analog I/O group x enable"]
pub struct G1E_R(crate::FieldReader<bool, bool>);
impl G1E_R {
    pub(crate) fn new(bits: bool) -> Self {
        G1E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for G1E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `G1E` writer - Analog I/O group x enable"]
pub struct G1E_W<'a> {
    w: &'a mut W,
}
impl<'a> G1E_W<'a> {
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
    #[doc = "Bit 23 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g8s(&self) -> G8S_R {
        G8S_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Analog I/O group x status"]
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&self) -> G8E_R {
        G8E_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g8e(&mut self) -> G8E_W {
        G8E_W { w: self }
    }
    #[doc = "Bit 6 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g7e(&mut self) -> G7E_W {
        G7E_W { w: self }
    }
    #[doc = "Bit 5 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g6e(&mut self) -> G6E_W {
        G6E_W { w: self }
    }
    #[doc = "Bit 4 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g5e(&mut self) -> G5E_W {
        G5E_W { w: self }
    }
    #[doc = "Bit 3 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g4e(&mut self) -> G4E_W {
        G4E_W { w: self }
    }
    #[doc = "Bit 2 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g3e(&mut self) -> G3E_W {
        G3E_W { w: self }
    }
    #[doc = "Bit 1 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g2e(&mut self) -> G2E_W {
        G2E_W { w: self }
    }
    #[doc = "Bit 0 - Analog I/O group x enable"]
    #[inline(always)]
    pub fn g1e(&mut self) -> G1E_W {
        G1E_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O group control status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iogcsr](index.html) module"]
pub struct IOGCSR_SPEC;
impl crate::RegisterSpec for IOGCSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iogcsr::R](R) reader structure"]
impl crate::Readable for IOGCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iogcsr::W](W) writer structure"]
impl crate::Writable for IOGCSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IOGCSR to value 0"]
impl crate::Resettable for IOGCSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
